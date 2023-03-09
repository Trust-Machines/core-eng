use frost_coordinator::create_coordinator;
use frost_signer::net::HttpNetListen;
use std::sync::mpsc;
use wtfrost::{common::Signature, Point};

use crate::config::Config;
use crate::frost_coordinator::FrostCoordinator as FrostCoordinatorTrait;
use crate::peg_wallet::StacksWallet;
use crate::peg_wallet::{BitcoinWallet, PegWallet};
use crate::stacks_node;

// Traits in scope
use crate::bitcoin_node::BitcoinNode;
use crate::peg_queue::{PegQueue, SbtcOp};
use crate::stacks_node::StacksNode;

type FrostCoordinator = frost_coordinator::coordinator::Coordinator<HttpNetListen>;

// TODO: Define these types
pub type PublicKey = Point;

pub trait Coordinator: Sized {
    type PegQueue: PegQueue;
    type FeeWallet: PegWallet;
    type StacksNode: StacksNode;
    type BitcoinNode: BitcoinNode;

    // Required methods
    fn peg_queue(&self) -> &Self::PegQueue;
    fn fee_wallet(&mut self) -> &mut Self::FeeWallet;
    fn frost_coordinator(&self) -> &FrostCoordinator;
    fn frost_coordinator_mut(&mut self) -> &mut FrostCoordinator;
    fn stacks_node(&self) -> &Self::StacksNode;
    fn bitcoin_node(&self) -> &Self::BitcoinNode;

    // Provided methods
    fn run(mut self, commands: mpsc::Receiver<Command>) {
        loop {
            match self.peg_queue().sbtc_op().unwrap() {
                Some(SbtcOp::PegIn(op)) => self.peg_in(op),
                Some(SbtcOp::PegOutRequest(op)) => self.peg_out(op),
                None => self.peg_queue().poll(self.stacks_node()).unwrap(),
            }

            match commands.try_recv() {
                Ok(Command::Stop) => break,
                Err(mpsc::TryRecvError::Disconnected) => break,
                Err(mpsc::TryRecvError::Empty) => continue,
            }
        }
    }
}

// Private helper functions
trait CoordinatorHelpers: Coordinator {
    fn peg_in(&mut self, op: stacks_node::PegInOp) {
        let tx = self.fee_wallet().stacks_mut().mint(&op);
        self.stacks_node().broadcast_transaction(&tx);
    }

    fn peg_out(&mut self, op: stacks_node::PegOutRequestOp) {
        let _stacks = self.fee_wallet().stacks_mut();
        let burn_tx = self.fee_wallet().stacks_mut().burn(&op);
        let fulfill_tx = self.fee_wallet().bitcoin_mut().fulfill_peg_out(&op);

        //TODO: what do we do with the returned signature?
        self.frost_coordinator_mut()
            .sign_message(fulfill_tx.as_bytes())
            .unwrap();

        self.stacks_node().broadcast_transaction(&burn_tx);
        self.bitcoin_node().broadcast_transaction(&fulfill_tx);
    }
}

impl<T: Coordinator> CoordinatorHelpers for T {}

pub enum Command {
    Stop,
}

pub struct StacksCoordinator {
    config: Config,
    frost_coordinator: FrostCoordinator,
}

impl StacksCoordinator {
    pub fn run_dkg_round(&mut self) -> PublicKey {
        self.frost_coordinator
            .run_distributed_key_generation()
            .unwrap()
    }

    pub fn sign_message(&mut self, message: &str) -> Signature {
        self.frost_coordinator
            .sign_message(message.as_bytes())
            .unwrap()
    }
}

impl From<Config> for StacksCoordinator {
    fn from(config: Config) -> Self {
        Self {
            config,
            frost_coordinator: create_coordinator(),
        }
    }
}

pub struct CoordinatorFoo<PQ: PegQueue, SW: StacksWallet, SN: StacksNode, BN: BitcoinNode> {
    peg_queue: PQ,
    fee_wallet: SW,
    frost_coordinator: FrostCoordinator,
    stacks_node: SN,
    bitcoin_node: BN,
}

#[cfg(test)]
mod tests {}
