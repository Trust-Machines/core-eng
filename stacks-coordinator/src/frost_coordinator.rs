use frost_coordinator::coordinator::{Coordinator, Error};
use frost_signer::net::NetListen;
use wtfrost::Point;

pub trait FrostCoordinator {
    fn run_dkg_round(&mut self) -> PublicKey;
    fn sign_message(&mut self, message: &str) -> Signature;
}

// TODO: Define these types
pub type Signature = String;
pub type PublicKey = Point;

impl<Network: NetListen> FrostCoordinator for Coordinator<Network>
where
    Error: From<Network::Error>,
{
    fn run_dkg_round(&mut self) -> PublicKey {
        self.run_distributed_key_generation().unwrap()
    }

    fn sign_message(&mut self, message: &str) -> Signature {
        todo!()
    }
}
