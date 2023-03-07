use wtfrost::{common::PolyCommitment, Scalar};

use super::super::Protocol;

/// Should be send by each signer after `DkgBegin`
/// A signer should send multiple events if multiple parties were assigned.
pub struct Share<P: Protocol> {
    pub dkg_id: P::DkgId,
    pub party_id: u32,
    pub public: PolyCommitment,
    /// The size of the vector should be `N-1`.
    /// They are messages for all parties except `party_id`.
    pub private: Vec<Scalar>,
}

impl<P: Protocol> Share<P> {
    fn private_index(&self, party_id: u32) -> usize {
        assert_ne!(self.party_id, party_id);
        (if party_id < self.party_id {
            party_id
        } else {
            party_id - 1
        }) as usize
    }
    pub fn private(&self, party_id: u32) -> &Scalar {
        self.private.get(self.private_index(party_id)).unwrap()
    }
    pub fn new(
        dkg_id: &P::DkgId,
        party_id: u32,
        public: PolyCommitment,
        n: u32,
        map: impl Iterator<Item = (u32, Scalar)>,
    ) -> Self {
        let mut result = Self {
            dkg_id: dkg_id.clone(),
            party_id,
            public,
            private: vec![Scalar::default(); n as usize],
        };
        for (private_party_id, private) in map {
            let index = result.private_index(private_party_id);
            *result.private.get_mut(index).unwrap() = private;
        }
        result
    }
}
