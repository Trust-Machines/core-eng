use wtfrost::v1::SignatureShare;

use crate::event::Protocol;

/// Should be send by a signer.
/// A signer should send multiple events if multiple parties were assigned.
pub struct Share<P: Protocol> {
    pub signature_id: P::SignatureId,
    pub party_id: u32,
    pub share: SignatureShare,
}
