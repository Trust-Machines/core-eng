use crate::stacks_node;

pub trait PegQueue {
    fn sbtc_op(&mut self) -> Option<SbtcOp>;
    fn poll(&mut self);
}

pub enum SbtcOp {
    PegIn(stacks_node::PegInOp),
    PegOutRequest(stacks_node::PegOutRequestOp),
}
