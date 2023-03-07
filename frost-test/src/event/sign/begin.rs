use crate::event::Protocol;

/// Should be sent by a signature aggregator.
/// In theory, a signer can be involved in signing multiple messages at the same time.
pub struct Begin<P: Protocol> {
    pub dkg_id: P::DkgId,
    pub signature_id: P::SignatureId,
    // It's a set of T parties so it can be a vector of bits.
    pub parties: Vec<u32>,
    pub message: Vec<u8>,
}
