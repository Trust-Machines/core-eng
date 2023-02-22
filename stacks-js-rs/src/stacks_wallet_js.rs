use serde::Serialize;
use stacks_coordinator::{
    peg_wallet::{PegWalletAddress, StacksWallet},
    stacks_node::{PegInOp, PegOutRequestOp},
};

use crate::{rpc::Rpc, Js};

pub struct StacksWalletJs(pub Js);

impl StacksWalletJs {
    fn call(&mut self, input: &In) -> String {
        self.0.call(input).unwrap()
    }
}

impl StacksWallet for StacksWalletJs {
    fn mint(&mut self, op: &PegInOp) -> String {
        self.call(&In::Mint(op))
    }

    fn burn(&mut self, op: &PegOutRequestOp) -> String {
        self.call(&In::Burn(op))
    }

    fn set_wallet_address(&mut self, address: PegWalletAddress) -> String {
        self.call(&In::SetWalletAddress(&address))
    }
}

#[derive(Serialize)]
pub enum In<'a> {
    Mint(&'a PegInOp),
    Burn(&'a PegOutRequestOp),
    SetWalletAddress(&'a PegWalletAddress),
}
