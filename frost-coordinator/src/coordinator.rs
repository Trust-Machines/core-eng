use std::any::Any;
use std::collections::BTreeMap;
use std::time::Duration;

use frost_signer::config::Config;
use frost_signer::net::{HttpNetError, Message, NetListen};
use frost_signer::signing_round::{
    DkgBegin, DkgPublicShare, MessageTypes, NonceRequest, NonceResponse, SignatureShareRequest,
};
use hashbrown::HashSet;
use tracing::{debug, info};
use wtfrost::{
    bip340::{Error as Bip340Error, SchnorrProof},
    common::{PolyCommitment, PublicNonce, Signature},
    compute,
    errors::AggregatorError,
    v1, Point,
};

use serde::{Deserialize, Serialize};

pub const DEVNET_COORDINATOR_ID: usize = 0;
pub const DEVNET_COORDINATOR_DKG_ID: u64 = 0; //TODO: Remove, this is a correlation id

#[derive(clap::Subcommand, Debug)]
pub enum Command {
    Dkg,
    Sign { msg: Vec<u8> },
    DkgSign { msg: Vec<u8> },
    GetAggregatePublicKey,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Coordinator<Network: NetListen> {
    id: u32, // Used for relay coordination
    current_dkg_id: u64,
    total_signers: usize, // Assuming the signers cover all id:s in {1, 2, ..., total_signers}
    total_keys: usize,
    threshold: usize,
    network: Network,
    dkg_public_shares: BTreeMap<u32, DkgPublicShare>,
    public_nonces: BTreeMap<u32, NonceResponse>,
    signature_shares: BTreeMap<u32, v1::SignatureShare>,
    aggregate_public_key: Point,
}

impl<Network: NetListen> Coordinator<Network> {
    pub fn new(id: usize, dkg_id: u64, config: &Config, network: Network) -> Self {
        Self {
            id: id as u32,
            current_dkg_id: dkg_id,
            total_signers: config.total_signers,
            total_keys: config.total_keys,
            threshold: config.keys_threshold,
            network,
            dkg_public_shares: Default::default(),
            public_nonces: Default::default(),
            aggregate_public_key: Point::default(),
            signature_shares: Default::default(),
        }
    }
}

impl<Network: NetListen> Coordinator<Network>
where
    Error: From<Network::Error>,
{
    pub fn run(&mut self, command: &Command) -> Result<(), Error> {
        match command {
            Command::Dkg => {
                self.run_distributed_key_generation()?;
                Ok(())
            }
            Command::Sign { msg } => {
                self.sign_message(msg)?;
                Ok(())
            }
            Command::DkgSign { msg } => {
                info!("sign msg: {:?}", msg);
                self.run_distributed_key_generation()?;
                self.sign_message(msg)?;
                Ok(())
            }
            Command::GetAggregatePublicKey => {
                let key = self.get_aggregate_public_key()?;
                info!("aggregate public key {}", key);
                Ok(())
            }
        }
    }

    pub fn run_distributed_key_generation(&mut self) -> Result<Point, Error> {
        self.start_dkg()?;
        let result = self.wait_for_dkg_end();
        info!("DKG round #{} finished", self.current_dkg_id);
        result
    }

    fn start_dkg(&mut self) -> Result<(), Error> {
        self.dkg_public_shares.clear();
        self.current_dkg_id += 1;
        info!("Starting DKG round #{}", self.current_dkg_id);
        let dkg_begin_message = Message {
            msg: MessageTypes::DkgBegin(DkgBegin {
                dkg_id: self.current_dkg_id,
            }),
            sig: [0; 32],
        };

        self.network.send_message(dkg_begin_message)?;
        Ok(())
    }

    fn collect_nonces(&mut self) -> Result<(), Error> {
        self.public_nonces.clear();

        let nonce_request_message = Message {
            msg: MessageTypes::NonceRequest(NonceRequest {
                dkg_id: self.current_dkg_id,
            }),
            sig: [0; 32],
        };

        info!("dkg_id #{}. NonceRequest sent.", self.current_dkg_id);
        self.network.send_message(nonce_request_message)?;

        loop {
            match self.wait_for_next_message()?.msg {
                MessageTypes::NonceRequest(_) => {}
                MessageTypes::NonceResponse(nonce_response) => {
                    let party_id = nonce_response.party_id;
                    self.public_nonces.insert(party_id, nonce_response);
                    info!(
                        "NonceResponse from party #{:?}. Got {} nonce responses of threshold {}",
                        party_id,
                        self.public_nonces.len(),
                        self.threshold,
                    );
                }
                msg => {
                    info!("NonceLoop Got unexpected message {:?})", msg.type_id());
                }
            }

            if self.public_nonces.len() == self.total_keys {
                info!("Nonce threshold of {} met.", self.threshold);
                break;
            }
        }
        Ok(())
    }

    pub fn sign_message(&mut self, msg: &[u8]) -> Result<(Signature, SchnorrProof), Error> {
        if self.aggregate_public_key == Point::default() {
            return Err(Error::NoAggregatePublicKey);
        }

        loop {
            self.collect_nonces()?;

            // check to see if the aggregate nonce R has even y
            let ids: Vec<usize> = self
                .public_nonces
                .iter()
                .map(|(i, _n)| *i as usize)
                .collect();
            let nonces: Vec<PublicNonce> = self
                .public_nonces
                .iter()
                .map(|(_i, n)| n.nonce.clone())
                .collect();
            let (_, R) = compute::intermediate(msg, &ids, &nonces);
            if R.has_even_y() {
                info!("R has even y coord: {}", &R);
                break;
            } else {
                info!("R does not have even y coord: {}", &R);
            }
        }

        // get the parties who responded with a nonce
        let mut waiting_for_signature_shares: HashSet<u32> =
            HashSet::from_iter(self.public_nonces.keys().cloned());

        // make an array of dkg public share polys for SignatureAggregator
        info!(
            "collecting commitments from 1..{} in {:?}",
            self.total_keys,
            self.dkg_public_shares.keys().collect::<Vec<&u32>>()
        );
        let polys: Vec<PolyCommitment> = self
            .dkg_public_shares
            .values()
            .map(|ps| ps.public_share.clone())
            .collect();

        info!(
            "SignatureAggregator::new total_keys: {} threshold: {} commitments: {} ",
            self.total_keys,
            self.threshold,
            polys.len()
        );
        let mut aggregator =
            match v1::SignatureAggregator::new(self.total_keys, self.threshold, polys) {
                Ok(aggregator) => aggregator,
                Err(e) => return Err(Error::Aggregator(e)),
            };

        let id_nonces: Vec<(u32, PublicNonce)> = self
            .public_nonces
            .iter()
            .map(|(i, n)| (*i, n.nonce.clone()))
            .collect();

        // request signature shares
        for party_id in self.public_nonces.keys() {
            let signature_share_request_message = Message {
                msg: MessageTypes::SignShareRequest(SignatureShareRequest {
                    dkg_id: self.current_dkg_id,
                    correlation_id: 0,
                    party_id: *party_id,
                    nonces: id_nonces.clone(),
                    message: msg.to_vec(),
                }),
                sig: [0; 32],
            };

            self.network.send_message(signature_share_request_message)?;
        }

        loop {
            match self.wait_for_next_message()?.msg {
                MessageTypes::SignShareResponse(response) => {
                    if let Some(_party_id) = waiting_for_signature_shares.take(&response.party_id) {
                        self.signature_shares
                            .insert(response.party_id, response.signature_share);
                    }
                    info!(
                        "signature share for {} received.  left to receive: {:?}",
                        response.party_id, waiting_for_signature_shares
                    );
                }
                MessageTypes::SignShareRequest(_) => {}
                msg => {
                    debug!("SigShare loop got unexpected msg {:?}", msg.type_id());
                }
            }

            if waiting_for_signature_shares.is_empty() {
                break;
            }
        }

        // call aggregator.sign()
        let nonces = id_nonces
            .iter()
            .map(|(_i, n)| n.clone())
            .collect::<Vec<PublicNonce>>();
        let shares = id_nonces
            .iter()
            .map(|(i, _n)| self.signature_shares[i].clone())
            .collect::<Vec<v1::SignatureShare>>();
        info!(
            "aggregator.sign({:?}, {:?}, {:?})",
            msg,
            nonces.len(),
            shares.len()
        );

        let sig = match aggregator.sign(msg, &nonces, &shares) {
            Ok(sig) => sig,
            Err(e) => return Err(Error::Aggregator(e)),
        };

        info!("Signature ({}, {})", sig.R, sig.z);

        let proof = match SchnorrProof::new(&sig) {
            Ok(proof) => proof,
            Err(e) => {
                return Err(Error::Bip340(e));
            }
        };

        info!("SchnorrProof ({}, {})", proof.r, proof.s);

        if !proof.verify(&self.aggregate_public_key.x(), msg) {
            info!("SchnorrProof failed to verify!");
            return Err(Error::SchnorrProofFailed);
        }

        Ok((sig, proof))
    }

    pub fn calculate_aggregate_public_key(&mut self) -> Result<Point, Error> {
        self.aggregate_public_key = self
            .dkg_public_shares
            .iter()
            .fold(Point::default(), |s, (_, dps)| s + dps.public_share.A[0]);
        Ok(self.aggregate_public_key)
    }

    pub fn get_aggregate_public_key(&mut self) -> Result<Point, Error> {
        if self.aggregate_public_key == Point::default() {
            Err(Error::NoAggregatePublicKey)
        } else {
            Ok(self.aggregate_public_key)
        }
    }

    fn wait_for_dkg_end(&mut self) -> Result<Point, Error> {
        let mut ids_to_await: HashSet<usize> = (1..=self.total_signers).collect();

        info!(
            "DKG round #{} started. Waiting for DkgEnd from signers {:?}",
            self.current_dkg_id, ids_to_await
        );

        loop {
            if ids_to_await.is_empty() {
                let key = self.calculate_aggregate_public_key()?;
                // check to see if aggregate public key has even y
                if key.has_even_y() {
                    info!("Aggregate public key has even y coord!");
                    info!("Aggregate public key: {}", key);
                    return Ok(key);
                } else {
                    info!("Aggregate public key does not have even y coord, re-run dkg!");
                    ids_to_await = (1..=self.total_signers).collect();
                    self.start_dkg()?;
                }
            }

            match self.wait_for_next_message()?.msg {
                MessageTypes::DkgEnd(dkg_end_msg) => {
                    ids_to_await.remove(&dkg_end_msg.signer_id);
                    info!(
                        "DKG_End round #{} from signer #{}. Waiting on {:?}",
                        dkg_end_msg.dkg_id, dkg_end_msg.signer_id, ids_to_await
                    );
                }
                MessageTypes::DkgPublicShare(dkg_public_share) => {
                    self.dkg_public_shares
                        .insert(dkg_public_share.party_id, dkg_public_share.clone());

                    info!(
                        "DKG round #{} DkgPublicSharefrom party #{}",
                        dkg_public_share.dkg_id, dkg_public_share.party_id
                    );
                }
                _ => {}
            }
        }
    }

    fn wait_for_next_message(&mut self) -> Result<Message, Error> {
        let get_next_message = || {
            self.network.poll(self.id);
            self.network
                .next_message()
                .ok_or_else(|| "No message yet".to_owned())
                .map_err(backoff::Error::transient)
        };

        let notify = |_err, dur| {
            debug!("No message. Next poll in {:?}", dur);
        };

        let backoff_timer = backoff::ExponentialBackoffBuilder::new()
            .with_max_interval(Duration::from_secs(3))
            .build();
        backoff::retry_notify(backoff_timer, get_next_message, notify).map_err(|_| Error::Timeout)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Http network error: {0}")]
    NetworkError(#[from] HttpNetError),
    #[error("No aggregate public key")]
    NoAggregatePublicKey,
    #[error("Aggregate failed to sign")]
    Aggregator(AggregatorError),
    #[error("BIP-340 error")]
    Bip340(Bip340Error),
    #[error("SchnorrProof failed to verify")]
    SchnorrProofFailed,
    #[error("Operation timed out")]
    Timeout,
}
