use std::io;

use serde::Serialize;

use crate::bitcoin_node;
use crate::stacks_node;

pub trait StacksWallet {
    fn mint(&mut self, op: &stacks_node::PegInOp) -> stacks_node::StacksTransaction;
    fn burn(&mut self, op: &stacks_node::PegOutRequestOp) -> stacks_node::StacksTransaction;
    fn set_wallet_address(&mut self, address: PegWalletAddress) -> stacks_node::StacksTransaction;
}

pub trait BitcoinWallet {
    fn fulfill_peg_out(
        &self,
        op: &stacks_node::PegOutRequestOp,
    ) -> bitcoin_node::BitcoinTransaction;
}

pub trait PegWallet {
    type StacksWallet: StacksWallet;
    type BitcoinWallet: BitcoinWallet;
    fn stacks_mut(&mut self) -> &mut Self::StacksWallet;
    fn bitcoin_mut(&mut self) -> &mut Self::BitcoinWallet;
}

// TODO: Representation
// Should correspond to a [u8; 32] - perhaps reuse a FROST type?
#[derive(Serialize)]
pub struct PegWalletAddress(pub [u8; 32]);
