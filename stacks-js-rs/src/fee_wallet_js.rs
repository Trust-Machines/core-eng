use std::io::Error;

use serde::Serialize;
use stacks_coordinator::stacks_node::{PegInOp, PegOutRequestOp, StacksTransaction};

use crate::Js;

pub struct FeeWalletJs(Js);

impl FeeWalletJs {
    fn mint_sbtc(&mut self, op: &PegInOp) -> Result<StacksTransaction, Error> {
        self.0.call(&In::Mint(op))
    }
    fn burn_sbtc(&mut self, op: &PegOutRequestOp) -> Result<StacksTransaction, Error> {
        self.0.call(&In::Burn(op))
    }
}

#[derive(Serialize)]
pub enum In<'a> {
    Mint(&'a PegInOp),
    Burn(&'a PegOutRequestOp),
}
