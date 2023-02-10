pub trait BitcoinNode {
    fn broadcast_transaction(&self, tx: &BitcoinTransaction);
}

pub type BitcoinTransaction = String;
