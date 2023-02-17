use bitcoin::{PackedLockTime, Transaction};
use hashbrown::HashMap;
use rand_core::OsRng;
use wtfrost::common::{PolyCommitment, Signature};
use wtfrost::errors::AggregatorError;
use wtfrost::{
    common::PublicNonce,
    traits::Signer,
    v1::{self, SignatureAggregator},
};

#[test]
fn pure_frost_test() {
    // Singer setup
    let T = 3;
    let N = 4;
    let mut rng = OsRng::default();
    let mut signers = [
        v1::Signer::new(&[0, 1], N, T, &mut rng),
        v1::Signer::new(&[2], N, T, &mut rng),
        v1::Signer::new(&[3], N, T, &mut rng),
    ];

    // DKG (Distributed Key Generation)
    let public_key_shares = dkg_round(&mut rng, &mut signers);

    // signing. Signers: 0 (parties: 0, 1) and 1 (parties: 2)
    let result = signing_round(T, N, &mut rng, &mut signers, public_key_shares);

    assert!(result.is_ok());
}

fn build_peg_in(satoshis: u64, stx_address: [u8; 32]) -> Transaction {
    // Peg-In TX
    let peg_in_input = bitcoin::TxIn {
        previous_output: Default::default(),
        script_sig: Default::default(),
        sequence: Default::default(),
        witness: Default::default(),
    };
    let peg_in_output = bitcoin::TxOut {
        value: satoshis,
        script_pubkey: Default::default(),
    };
    bitcoin::blockdata::transaction::Transaction {
        version: 0,
        lock_time: PackedLockTime(0),
        input: vec![peg_in_input],
        output: vec![peg_in_output],
    }
}

fn signing_round(
    T: usize,
    N: usize,
    mut rng: &mut OsRng,
    signers: &mut [v1::Signer; 3],
    A: Vec<PolyCommitment>,
) -> Result<Signature, AggregatorError> {
    // decide which signers will be used
    let mut signers = [signers[0].clone(), signers[1].clone()];

    const MSG: &[u8] = "It was many and many a year ago".as_bytes();

    // get nonces and shares
    let (nonces, shares) = {
        let ids: Vec<usize> = signers.iter().flat_map(|s| s.get_ids()).collect();
        // get nonces
        let nonces: Vec<PublicNonce> = signers
            .iter_mut()
            .flat_map(|s| s.gen_nonces(&mut rng))
            .collect();
        // get shares
        let shares = signers
            .iter()
            .flat_map(|s| s.sign(MSG, &ids, &nonces))
            .collect::<Vec<_>>();

        (nonces, shares)
    };

    SignatureAggregator::new(N, T, A.clone())
        .unwrap()
        .sign(&MSG, &nonces, &shares)
}

fn dkg_round(mut rng: &mut OsRng, signers: &mut [v1::Signer; 3]) -> Vec<PolyCommitment> {
    {
        let public_key_shares = signers
            .iter()
            .flat_map(|s| s.get_poly_commitments(&mut rng))
            .collect::<Vec<_>>();

        // each party broadcasts their commitments
        // these hashmaps will need to be serialized in tuples w/ the value encrypted
        let broadcast_shares = signers
            .iter()
            .flat_map(|signer| signer.parties.iter())
            .map(|party| (party.id, party.get_shares()))
            .collect::<Vec<_>>();

        // each party collects its shares from the broadcasts
        // maybe this should collect into a hashmap first?
        let secret_errors = signers
            .iter_mut()
            .flat_map(|s| s.parties.iter_mut())
            .filter_map(|party| {
                let h = broadcast_shares
                    .iter()
                    .map(|(id, share)| (*id, share[&party.id]))
                    .collect::<HashMap<_, _>>();

                // should a signer go at error state if error?
                if let Err(secret_error) = party.compute_secret(h, &public_key_shares) {
                    Some((party.id, secret_error))
                } else {
                    None
                }
            })
            .collect::<HashMap<_, _>>();

        if secret_errors.is_empty() {
            Ok(public_key_shares)
        } else {
            Err(secret_errors)
        }
    }
    .unwrap()
}