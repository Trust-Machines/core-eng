use crate::{
    event::{dkg, Event, Protocol},
    participant::Participant,
};
use hashbrown::HashMap;
use rand_core::{CryptoRng, RngCore};
use wtfrost::{common::PolyCommitment, errors::DkgError, v1, Scalar};

pub trait SignerPolicy: Protocol {
    type R: RngCore + CryptoRng;
}

struct DkgSigner {
    signer: v1::Signer,
    countdown: usize,
    public: Vec<Option<PolyCommitment>>,
    private: HashMap<u32, HashMap<usize, Scalar>>,
}

impl DkgSigner {
    fn new(signer: v1::Signer, private: HashMap<u32, HashMap<usize, Scalar>>) -> DkgSigner {
        let n = signer.n;
        DkgSigner {
            signer,
            countdown: n,
            public: vec![None; n],
            private,
        }
    }
    fn share<P: Protocol>(&mut self, dkg_share: &dkg::Share<P>) -> usize {
        // TODO: check if `self.public[dkg_share.party_id as usze].is_none()`
        self.public[dkg_share.party_id as usize] = Some(dkg_share.public.clone());
        for (party_id, shares) in self.private.iter_mut() {
            shares.insert(dkg_share.party_id as usize, *dkg_share.private(*party_id));
        }
        self.countdown -= 1;
        self.countdown
    }
    fn end(mut self) -> Result<v1::Signer, HashMap<usize, DkgError>> {
        let public = self
            .public
            .iter()
            .filter_map(|v| v.clone())
            .collect::<Vec<_>>();
        let errors = self
            .signer
            .parties
            .iter_mut()
            .filter_map(|party| {
                let p =
                    party.compute_secret(self.private.remove(&(party.id as u32)).unwrap(), &public);
                if let Err(secret_error) = p {
                    Some((party.id, secret_error))
                } else {
                    None
                }
            })
            .collect::<HashMap<_, _>>();
        if errors.is_empty() {
            Ok(self.signer)
        } else {
            Err(errors)
        }
    }
}

pub struct Signer<P: SignerPolicy> {
    id: P::ParticipantId,
    rng: P::R,
    dkg: HashMap<P::DkgId, DkgSigner>,
    signers: HashMap<P::DkgId, v1::Signer>,
}

fn get_indices<T: Eq>(map: &[T], id: &T) -> Vec<usize> {
    map.iter()
        .enumerate()
        .filter_map(|(i, map_id)| if *id == *map_id { Some(i) } else { None })
        .collect()
}

fn get_dkg_shares<P: SignerPolicy>(
    signer: &v1::Signer,
    dkg_id: &P::DkgId,
    n: usize,
    rng: &mut P::R,
) -> Vec<Event<P>> {
    signer
        .parties
        .iter()
        .map(|party| {
            // Should `get_poly_commitment` be called before `get_shares`?
            // If yes, it's an anti-pattern https://en.wikipedia.org/wiki/Sequential_coupling.
            // Should we `signer.get_poly_commitment` instead of `party.get_poly_commitment` in FROST v2?
            // Proposal: calculate poly commitments at `Signer::new()`.
            let public = party.get_poly_commitment(rng);
            Event::DkgShare(dkg::Share::new(
                dkg_id,
                party.id as u32,
                public,
                n as u32,
                party
                    .get_shares()
                    .into_iter()
                    .map(|(id, scalar)| (id as u32, scalar)),
            ))
        })
        .collect()
}

impl<P: SignerPolicy> Signer<P> {
    fn dkg_begin(&mut self, dkg_begin: &dkg::Begin<P>) -> Vec<Event<P>> {
        let n = dkg_begin.party_to_signer_map.len();
        let indices = get_indices(&dkg_begin.party_to_signer_map, &self.id);
        if indices.is_empty() {
            return Vec::default();
        }
        // Should we check if `self.dkg` already has the `dkg_id`?
        let signer = v1::Signer::new(&indices, n, dkg_begin.t as usize, &mut self.rng);
        let events = get_dkg_shares(&signer, &dkg_begin.dkg_id, n, &mut self.rng);
        let private = signer
            .parties
            .iter()
            .map(|party| (party.id as u32, HashMap::default()))
            .collect();
        let dkg_signer = DkgSigner::new(signer, private);
        self.dkg.insert(dkg_begin.dkg_id.clone(), dkg_signer);
        events
    }
    fn dkg_share(&mut self, dkg_share: &dkg::Share<P>) -> Vec<Event<P>> {
        if let Some(signer) = self.dkg.get_mut(&dkg_share.dkg_id) {
            if signer.share(dkg_share) == 0 {
                let ds = self.dkg.remove(&dkg_share.dkg_id).unwrap();
                if let Ok(signer) = ds.end() {
                    let result = signer
                        .parties
                        .iter()
                        .map(|p| {
                            Event::DkgEnd(dkg::End::<P> {
                                dkg_id: dkg_share.dkg_id.clone(),
                                party_id: p.id as u32,
                            })
                        })
                        .collect();
                    self.signers.insert(dkg_share.dkg_id.clone(), signer);
                    return result;
                }
            }
        }
        Vec::default()
    }
    fn dkg_end(&mut self, _: &dkg::End<P>) -> Vec<Event<P>> {
        Vec::default()
    }
}

impl<P: SignerPolicy> Participant for Signer<P> {
    type P = P;

    fn call(&mut self, event: &Event<P>) -> Vec<Event<P>> {
        match event {
            Event::DkgBegin(v) => self.dkg_begin(v),
            Event::DkgShare(v) => self.dkg_share(v),
            Event::DkgEnd(v) => self.dkg_end(v),
            _ => todo!(),
        }
    }
}
