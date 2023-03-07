use crate::event::Protocol;

/// Should be sent by DKG coordinator.
/// In theory, a signer can participate in multiple DKGs and be
/// responsible for multiple parties (DKG and signature shares).
pub struct Begin<P: Protocol> {
    pub dkg_id: P::DkgId,
    pub t: u32,
    /// Has a size of N
    pub party_to_signer_map: Vec<P::ParticipantId>,
}
