pub trait FrostCoordinator {
    fn run_dkg_round(&mut self) -> PublicKey;
    fn sign_message(&mut self, message: &str) -> Signature;
}

// TODO: Define these types
pub type Signature = String;
pub type PublicKey = String;
