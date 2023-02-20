use std::io::Error;

use serde::Serialize;
use stacks_coordinator::{
    peg_wallet::StacksWallet,
    stacks_node::{PegInOp, PegOutRequestOp, StacksTransaction},
};

use crate::Js;

pub struct FeeWalletJs(Js);

impl StacksWallet for FeeWalletJs {
    fn mint(&mut self, op: &stacks_coordinator::stacks_node::PegInOp) -> String {
        self.0.call(&In::Mint(op)).unwrap()
    }

    fn burn(&mut self, op: &stacks_coordinator::stacks_node::PegOutRequestOp) -> String {
        self.0.call(&In::Burn(op)).unwrap()
    }

    fn set_wallet_address(
        &mut self,
        address: stacks_coordinator::peg_wallet::PegWalletAddress,
    ) -> String {
        todo!()
    }
}

#[derive(Serialize)]
pub enum In<'a> {
    Mint(&'a PegInOp),
    Burn(&'a PegOutRequestOp),
}
