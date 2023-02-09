pub use frost;
use frost::{
    common::{PolyCommitment, PublicNonce},
    Scalar,
};
use hashbrown::HashMap;
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use tracing::{debug, info, warn};

use crate::state_machine::{StateMachine, States};

type KeyShares = HashMap<usize, Scalar>;

pub struct SigningRound {
    pub dkg_id: Option<u64>,
    pub threshold: usize,
    pub total: usize,
    pub signer: Signer,
    pub state: States,
    pub commitments: BTreeMap<u32, PolyCommitment>,
    pub shares: HashMap<u32, HashMap<usize, Scalar>>,
}

pub struct Signer {
    pub frost_signer: frost::v1::Signer,
    pub signer_id: u64,
}

impl StateMachine for SigningRound {
    fn move_to(&mut self, state: States) -> Result<(), String> {
        self.can_move_to(&state)?;
        self.state = state;
        Ok(())
    }

    fn can_move_to(&self, state: &States) -> Result<(), String> {
        let previous_state = &self.state;
        let accepted = match state {
            States::Idle => true,
            States::DkgDistribute => {
                self.state == States::Idle || self.state == States::DkgDistribute
            }
            States::DkgGather => self.state == States::DkgDistribute,
            States::SignGather => self.state == States::Idle,
            States::Signed => self.state == States::SignGather,
        };
        if accepted {
            info!("state change from {:?} to {:?}", previous_state, state);
            Ok(())
        } else {
            Err(format!("bad state change: {:?} to {:?}", self.state, state))
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageTypes {
    DkgBegin(DkgBegin),
    DkgEnd(DkgEnd),
    DkgQuery,
    DkgQueryResponse(DkgQueryResponse),
    DkgPublicShare(DkgPublicShare),
    DkgPrivateShares(DkgPrivateShares),
    NonceRequest(NonceRequest),
    NonceResponse(NonceResponse),
    SignShareRequest(SignatureShareRequest),
    SignShareResponse(SignatureShareResponse),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DkgPublicShare {
    pub dkg_id: u64,
    pub party_id: u32,
    pub public_share: PolyCommitment,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DkgPrivateShares {
    pub dkg_id: u64,
    pub party_id: u32,
    pub private_shares: KeyShares,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DkgBegin {
    pub dkg_id: u64, //TODO: Strong typing for this, alternatively introduce a type alias
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DkgEnd {
    pub dkg_id: u64,
    pub signer_id: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DkgQueryResponse {
    pub dkg_id: u64,
    pub public_share: PolyCommitment,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NonceRequest {
    pub dkg_id: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NonceResponse {
    pub dkg_id: u64,
    pub signer_id: usize,
    pub nonce: PublicNonce,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignatureShareRequest {
    pub dkg_id: u64,
    pub correlation_id: u64,
    pub signer_id: usize,
    pub nonce: PublicNonce,
    pub message: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignatureShareResponse {
    pub dkg_id: u64,
    pub correlation_id: u64,
    pub signer_id: usize,
    pub signature_share: frost::v1::SignatureShare,
}

impl SigningRound {
    pub fn new(
        threshold: usize,
        total: usize,
        signer_id: u64,
        party_ids: Vec<usize>,
    ) -> SigningRound {
        assert!(threshold <= total);
        let mut rng = OsRng::default();
        let frost_signer = frost::v1::Signer::new(&party_ids, total, threshold, &mut rng);
        let signer = Signer {
            frost_signer,
            signer_id,
        };

        SigningRound {
            dkg_id: None,
            threshold,
            total,
            signer,
            state: States::Idle,
            commitments: BTreeMap::new(),
            shares: HashMap::new(),
        }
    }

    pub fn process(&mut self, message: MessageTypes) -> Result<Vec<MessageTypes>, String> {
        let out_msgs = match message {
            MessageTypes::DkgBegin(dkg_begin) => self.dkg_begin(dkg_begin),
            MessageTypes::DkgPublicShare(dkg_public_shares) => {
                self.dkg_public_share(dkg_public_shares)
            }
            MessageTypes::DkgPrivateShares(dkg_private_shares) => {
                self.dkg_private_shares(dkg_private_shares)
            }
            _ => Ok(vec![]), // TODO
        };

        match out_msgs {
            Ok(mut out) => {
                if self.can_dkg_end() {
                    info!(
                        "can_dkg_end==true. shares {} commitments {}",
                        self.shares.len(),
                        self.commitments.len()
                    );
                    let dkg_end_msgs = self.dkg_ended().unwrap();
                    out.push(dkg_end_msgs);
                    self.move_to(States::Idle).unwrap();
                }
                Ok(out)
            }
            Err(e) => Err(e),
        }
    }

    pub fn dkg_ended(&self) -> Result<MessageTypes, String> {
        let parties = self.signer.frost_signer.parties.clone();
        for mut party in parties {
            let commitments: Vec<PolyCommitment> = self.commitments.clone().into_values().collect();
            let mut shares: HashMap<usize, Scalar> = HashMap::new();
            for (party_id, party_shares) in &self.shares {
                info!(
                    "building shares with k: {} v: party_shares[{}] len {} keys: {:?}",
                    party_id,
                    party.id,
                    party_shares.len(),
                    party_shares.keys()
                );
                shares.insert(*party_id as usize, party_shares[&party.id]);
            }
            info!(
                "party{}.compute_secret shares_for_id:{:?}",
                party.id,
                shares.keys()
            );
            if let Err(secret_error) = party.compute_secret(shares, &commitments) {
                warn!(
                    "DKG round #{}: party {} compute_secret failed in : {}",
                    self.dkg_id.unwrap(),
                    party.id,
                    secret_error
                );
            }
        }
        let dkg_end = MessageTypes::DkgEnd(DkgEnd {
            dkg_id: self.dkg_id.unwrap() as u64,
            signer_id: self.signer.signer_id as usize,
        });
        info!(
            "DKG_END round #{} signer_id {}",
            self.dkg_id.unwrap(),
            self.signer.signer_id
        );
        Ok(dkg_end)
    }

    pub fn can_dkg_end(&self) -> bool {
        debug!(
            "can_dkg_end state {:?} commitments {} shares {}",
            self.state,
            self.commitments.len(),
            self.shares.len()
        );
        self.state == States::DkgGather
            && self.commitments.len() == self.total
            && self.shares.len() == self.total
    }

    pub fn key_share_for_party(&self, party_id: usize) -> KeyShares {
        self.signer.frost_signer.parties[party_id].get_shares()
    }

    pub fn reset(&mut self, dkg_id: u64) {
        self.dkg_id = Some(dkg_id);
        self.commitments.clear();
        self.shares.clear();
    }

    pub fn dkg_begin(&mut self, dkg_begin: DkgBegin) -> Result<Vec<MessageTypes>, String> {
        self.reset(dkg_begin.dkg_id);
        self.move_to(States::DkgDistribute).unwrap();
        let _party_state = self.signer.frost_signer.save();

        let mut rng = OsRng::default();
        let mut msgs = vec![];
        for (_idx, party) in self.signer.frost_signer.parties.iter().enumerate() {
            info!("sending dkg private share for party #{}", party.id);
            let private_shares = MessageTypes::DkgPrivateShares(DkgPrivateShares {
                dkg_id: self.dkg_id.unwrap() as u64,
                party_id: party.id as u32,
                private_shares: party.get_shares(),
            });
            msgs.push(private_shares);
            info!("sending dkg public commitment for party #{}", party.id);
            let public_share = MessageTypes::DkgPublicShare(DkgPublicShare {
                dkg_id: self.dkg_id.unwrap() as u64,
                party_id: party.id as u32,
                public_share: party.get_poly_commitment(&mut rng),
            });
            msgs.push(public_share);
        }

        self.move_to(States::DkgGather).unwrap();
        Ok(msgs)
    }

    pub fn dkg_public_share(
        &mut self,
        dkg_public_share: DkgPublicShare,
    ) -> Result<Vec<MessageTypes>, String> {
        self.commitments
            .insert(dkg_public_share.party_id, dkg_public_share.public_share);
        info!(
            "received party #{} PUBLIC commitments {}/{}",
            dkg_public_share.party_id,
            self.commitments.len(),
            self.total
        );
        Ok(vec![])
    }

    pub fn dkg_private_shares(
        &mut self,
        dkg_private_shares: DkgPrivateShares,
    ) -> Result<Vec<MessageTypes>, String> {
        let shares_clone = dkg_private_shares.private_shares.clone();
        self.shares.insert(
            dkg_private_shares.party_id,
            dkg_private_shares.private_shares,
        );
        info!(
            "received party #{} PRIVATE shares {}/{} {:?}",
            dkg_private_shares.party_id,
            self.shares.len(),
            self.total,
            shares_clone.keys(),
        );
        Ok(vec![])
    }
}

#[cfg(test)]
mod test {
    use frost::common::PolyCommitment;
    use frost::schnorr::ID;
    use frost::Scalar;
    use hashbrown::HashMap;
    use rand_core::{CryptoRng, OsRng, RngCore};

    use crate::signing_round::{DkgPublicShare, MessageTypes, SigningRound};
    use crate::state_machine::States;

    fn get_rng() -> impl RngCore + CryptoRng {
        let rnd = OsRng::default();
        //rand::rngs::StdRng::seed_from_u64(rnd.next_u64()) // todo: fix trait `rand_core::RngCore` is not implemented for `StdRng`
        rnd
    }

    #[test]
    fn dkg_public_share() {
        let mut rnd = get_rng();
        let mut signing_round = SigningRound::new(1, 1, 1, vec![1]);
        let public_share = DkgPublicShare {
            dkg_id: 0,
            party_id: 0,
            public_share: PolyCommitment {
                id: ID::new(&Scalar::new(), &Scalar::new(), &mut rnd),
                A: vec![],
            },
        };
        signing_round.dkg_public_share(public_share).unwrap();
        assert_eq!(1, signing_round.commitments.len())
    }

    #[test]
    fn can_dkg_end() {
        let mut rnd = get_rng();
        let mut signing_round = SigningRound::new(1, 1, 1, vec![1]);
        // can_dkg_end starts out as false
        assert_eq!(false, signing_round.can_dkg_end());

        // meet the conditions for DKG_END
        signing_round.state = States::DkgGather;
        signing_round.commitments.insert(
            1,
            PolyCommitment {
                id: ID::new(&Scalar::new(), &Scalar::new(), &mut rnd),
                A: vec![],
            },
        );
        let shares: HashMap<usize, Scalar> = HashMap::new();
        signing_round.shares.insert(1, shares);

        // can_dkg_end should be true
        assert!(signing_round.can_dkg_end());
    }

    #[test]
    fn dkg_ended() {
        let mut signing_round = SigningRound::new(1, 1, 1, vec![1]);
        signing_round.reset(1);
        if let Ok(end_msg) = signing_round.dkg_ended() {
            match end_msg {
                MessageTypes::DkgEnd(dkg_end) => assert_eq!(dkg_end.dkg_id, 1),
                _ => {}
            }
        } else {
            assert!(false)
        }
    }
}
