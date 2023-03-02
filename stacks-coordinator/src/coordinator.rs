use std::sync::mpsc;

use crate::peg_queue;
use crate::peg_wallet::StacksWallet;
use crate::peg_wallet::{BitcoinWallet, PegWallet};
use crate::stacks_node;

// Traits in scope
use crate::bitcoin_node::BitcoinNode;
use crate::frost_coordinator::FrostCoordinator;
use crate::peg_queue::PegQueue;
use crate::stacks_node::StacksNode;

pub trait Coordinator: Sized {
    type PegQueue: PegQueue;
    type FeeWallet: PegWallet;
    type FrostCoordinator: FrostCoordinator;
    type StacksNode: StacksNode;
    type BitcoinNode: BitcoinNode;

    // Required methods
    fn peg_queue(&self) -> &Self::PegQueue;
    fn fee_wallet(&mut self) -> &mut Self::FeeWallet;
    fn frost_coordinator(&self) -> &Self::FrostCoordinator;
    fn stacks_node(&self) -> &Self::StacksNode;
    fn bitcoin_node(&self) -> &Self::BitcoinNode;

    // Provided methods
    fn run(mut self, commands: mpsc::Receiver<Command>) {
        loop {
            match self.peg_queue().sbtc_op().unwrap() {
                Some(peg_queue::SbtcOp::PegIn(op)) => self.peg_in(op),
                Some(peg_queue::SbtcOp::PegOutRequest(op)) => self.peg_out(op),
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

        // TODO: Sign fulfill tx with frost

        self.stacks_node().broadcast_transaction(&burn_tx);
        self.bitcoin_node().broadcast_transaction(&fulfill_tx);
    }
}

impl<T: Coordinator> CoordinatorHelpers for T {}

pub enum Command {
    Stop,
}

#[cfg(test)]
mod tests {}
