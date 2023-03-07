use crate::event::Protocol;

pub struct End<P: Protocol> {
    pub dkg_id: P::DkgId,
    pub party_id: u32,
}
