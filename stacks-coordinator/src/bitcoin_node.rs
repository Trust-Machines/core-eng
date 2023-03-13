pub trait BitcoinNode {
    fn broadcast_transaction(&self, tx: &BitcoinTransaction);
}

pub type BitcoinTransaction = bitcoin::Transaction;

pub struct LocalhostBitcoinNode {}

impl BitcoinNode for LocalhostBitcoinNode {
    fn broadcast_transaction(&self, _tx: &BitcoinTransaction) {
        todo!()
    }
}
