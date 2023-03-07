use std::hash::Hash;

/// Protocol policy.
pub trait Protocol {
    type DkgId: Eq + Clone + Hash;
    type ParticipantId: Eq;
    type SignatureId;
}
