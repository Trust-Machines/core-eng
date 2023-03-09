pub mod client;

use blockstack_lib::chainstate::burn::operations as burn_ops;
use blockstack_lib::types::chainstate::StacksAddress;

pub use blockstack_lib::chainstate::stacks::StacksTransaction;

#[cfg_attr(test, mockall::automock)]
pub trait StacksNode {
    fn get_peg_in_ops(&self, block_height: u64) -> Vec<PegInOp>;
    fn get_peg_out_request_ops(&self, block_height: u64) -> Vec<PegOutRequestOp>;
    fn burn_block_height(&self) -> u64;
    fn next_nonce(&self, addr: StacksAddress) -> u64;
    fn broadcast_transaction(&self, tx: &StacksTransaction);
}

pub type PegInOp = burn_ops::PegInOp;
pub type PegOutRequestOp = burn_ops::PegOutRequestOp;
