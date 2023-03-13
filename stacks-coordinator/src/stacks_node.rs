use blockstack_lib::chainstate::burn::operations as burn_ops;
use blockstack_lib::types::chainstate::StacksAddress;

use crate::stacks_transaction::StacksTransaction;

#[cfg_attr(test, mockall::automock)]
pub trait StacksNode {
    fn get_peg_in_ops(&self, block_height: u64) -> Vec<PegInOp>;
    fn get_peg_out_request_ops(&self, block_height: u64) -> Vec<PegOutRequestOp>;
    fn burn_block_height(&self) -> u64;
    fn next_nonce(&self, addr: StacksAddress);
    fn broadcast_transaction(&self, tx: &StacksTransaction);
}

pub type PegInOp = burn_ops::PegInOp;
pub type PegOutRequestOp = burn_ops::PegOutRequestOp;

pub struct LocalhostStacksNode {}

impl StacksNode for LocalhostStacksNode {
    fn get_peg_in_ops(&self, _block_height: u64) -> Vec<PegInOp> {
        todo!()
    }

    fn get_peg_out_request_ops(&self, _block_height: u64) -> Vec<PegOutRequestOp> {
        todo!()
    }

    fn burn_block_height(&self) -> u64 {
        todo!()
    }

    fn next_nonce(&self, _addr: StacksAddress) {
        todo!()
    }

    fn broadcast_transaction(&self, _tx: &StacksTransaction) {
        todo!()
    }
}
