use stacks_coordinator::{stacks_node, fee_wallet::PegWalletAddress, bitcoin_node};

pub trait FeeWallet {
    fn mint_sbtc(&self, op: &stacks_node::PegInOp) -> stacks_node::StacksTransaction;
    fn burn_sbtc(&self, op: &stacks_node::PegOutRequestOp) -> stacks_node::StacksTransaction;
    fn set_wallet_address(&self, address: PegWalletAddress) -> stacks_node::StacksTransaction;

    fn fulfill_peg_out(
        &self,
        op: &stacks_node::PegOutRequestOp,
    ) -> bitcoin_node::BitcoinTransaction;
}
