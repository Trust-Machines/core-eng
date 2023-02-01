use crate::bitcoin_node;
use crate::stacks_node;

pub trait FeeWallet {
    fn mint_sbtc(&self, op: &stacks_node::PegInOp) -> stacks_node::StacksTransaction;
    fn burn_sbtc(&self, op: &stacks_node::PegOutRequestOp) -> stacks_node::StacksTransaction;
    fn set_wallet_address(&self, address: PegWalletAddress) -> stacks_node::StacksTransaction;

    fn fulfill_peg_out(
        &self,
        op: &stacks_node::PegOutRequestOp,
    ) -> bitcoin_node::BitcoinTransaction;
}

// TODO: Representation
pub struct PegWalletAddress {} // Should correspond to a [u8; 32] - perhaps reuse a FROST type?
