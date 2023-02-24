use std::path::Path;

use serde::Serialize;
use stacks_coordinator::{
    peg_wallet::{PegWalletAddress, StacksWallet},
    stacks_node::{PegInOp, PegOutRequestOp},
};
use yarpc::{js::Js, rpc::Rpc};

pub struct StacksWalletJs {
    js: Js,
}

impl StacksWalletJs {
    pub fn new(path: &str) -> Self {
        let file_name = Path::new(path).join("stacks-js-rs/js/dispatch.ts");
        Self {
            js: Js::new(file_name.to_str().unwrap()).unwrap(),
        }
    }
    fn call(&mut self, input: &In) -> String {
        self.js.call(input).unwrap()
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
enum In<'a> {
    Mint(&'a PegInOp),
    Burn(&'a PegOutRequestOp),
    SetWalletAddress(&'a PegWalletAddress),
}
