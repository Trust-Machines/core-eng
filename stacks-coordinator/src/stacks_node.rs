use blockstack_lib::chainstate::burn::operations as burn_ops;
use blockstack_lib::types::chainstate::BurnchainHeaderHash as BitcoinHeaderHash;
use blockstack_lib::types::chainstate::StacksAddress;

pub trait StacksNode {
    fn get_peg_in_ops(&self, bitcoin_block_hash: BitcoinHeaderHash) -> Vec<PegInOp>;
    fn get_peg_out_request_ops(
        &self,
        bitcoin_block_hash: BitcoinHeaderHash,
    ) -> Vec<PegOutRequestOp>;
    fn next_nonce(&self, addr: StacksAddress);
    fn broadcast_transaction(&self, tx: &StacksTransaction);
}

pub type PegInOp = burn_ops::PegInOp;
pub type PegOutRequestOp = burn_ops::PegOutRequestOp;

// TODO: Find appropriate type
pub type StacksTransaction = String;
