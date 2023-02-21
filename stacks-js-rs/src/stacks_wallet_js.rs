use std::io;

use serde::Serialize;
use stacks_coordinator::{
    peg_wallet::StacksWallet,
    stacks_node::{PegInOp, PegOutRequestOp},
};

use crate::{rpc::Rpc, Js};

pub struct StacksWalletJs(Js);

impl StacksWallet for StacksWalletJs {
    fn mint(&mut self, op: &stacks_coordinator::stacks_node::PegInOp) -> String {
        self.0.call(&In::Mint(op)).unwrap()
    }

    fn burn(
        &mut self,
        op: &stacks_coordinator::stacks_node::PegOutRequestOp,
    ) -> String {
        self.0.call(&In::Burn(op)).unwrap()
    }

    fn set_wallet_address(
        &mut self,
        address: stacks_coordinator::peg_wallet::PegWalletAddress,
    ) -> String {
        self.0.call(&In::SetWalletAddress(&address)).unwrap()
    }
}

#[derive(Serialize)]
pub enum In<'a> {
    Mint(&'a PegInOp),
    Burn(&'a PegOutRequestOp),
    SetWalletAddress(&'a stacks_coordinator::peg_wallet::PegWalletAddress),
}
