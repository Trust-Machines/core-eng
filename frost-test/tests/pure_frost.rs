use hashbrown::HashMap;
use rand_core::OsRng;
use wtfrost::{
    common::PublicNonce,
    traits::Signer,
    v1::{self, SignatureAggregator},
};

#[test]
fn pure_frost_test() {
    let T = 3;
    let N = 4;
    let mut rng = OsRng::default();
    let mut signers = [
        v1::Signer::new(&[0, 1], N, T, &mut rng),
        v1::Signer::new(&[2], N, T, &mut rng),
        v1::Signer::new(&[3], N, T, &mut rng),
    ];

    // DKG (Distributed Key Generation)
    let A = {
        let A = signers
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
                if let Err(secret_error) = party.compute_secret(h, &A) {
                    Some((party.id, secret_error))
                } else {
                    None
                }
            })
            .collect::<HashMap<_, _>>();

        if secret_errors.is_empty() {
            Ok(A)
        } else {
            Err(secret_errors)
        }
    }
    .unwrap();

    // signing. Signers: 0 (parties: 0, 1) and 1 (parties: 2)
    let result = {
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
    };

    assert!(result.is_ok());
}
