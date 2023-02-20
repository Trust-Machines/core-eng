use std::io;

use serde::Serialize;
use stacks_coordinator::{
    peg_wallet::StacksWallet,
    stacks_node::{PegInOp, PegOutRequestOp},
};

use crate::Js;

pub struct StacksWalletJs(Js);

impl StacksWallet for StacksWalletJs {
    fn mint(&mut self, op: &stacks_coordinator::stacks_node::PegInOp) -> io::Result<String> {
        self.0.call(&In::Mint(op))
    }

    fn burn(
        &mut self,
        op: &stacks_coordinator::stacks_node::PegOutRequestOp,
    ) -> io::Result<String> {
        self.0.call(&In::Burn(op))
    }

    fn set_wallet_address(
        &mut self,
        address: stacks_coordinator::peg_wallet::PegWalletAddress,
    ) -> io::Result<String> {
        self.0.call(&In::SetWalletAddress(&address))
    }
}

#[derive(Serialize)]
pub enum In<'a> {
    Mint(&'a PegInOp),
    Burn(&'a PegOutRequestOp),
    SetWalletAddress(&'a stacks_coordinator::peg_wallet::PegWalletAddress),
}
