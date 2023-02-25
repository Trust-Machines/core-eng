use std::path::Path;

use crate::make_contract_call::{MakeContractCall, SignedContractCallOptions, StacksTransaction};
use stacks_coordinator::{
    peg_wallet::{PegWalletAddress, StacksWallet},
    stacks_node::{PegInOp, PegOutRequestOp},
};

pub struct StacksWalletJs {
    make_contract_call: MakeContractCall,
    sender_key: String,
}

impl StacksWalletJs {
    pub fn new(path: &str, sender_key: String) -> Self {
        let file_name = Path::new(path).join("stacks-js-rs/js/dispatch.ts");
        Self {
            make_contract_call: MakeContractCall::new(file_name.to_str().unwrap()),
            sender_key,
        }
    }
    fn call(&mut self, input: &SignedContractCallOptions) -> StacksTransaction {
        self.make_contract_call.call(&input)
    }
}

impl StacksWallet for StacksWalletJs {
    fn mint(&mut self, op: &PegInOp) -> String {
        let input = SignedContractCallOptions {
            contractAddress: todo!(),
            contractName: todo!(),
            functionName: todo!(),
            functionArgs: todo!(),
            fee: todo!(),
            feeEstimateApiUrl: todo!(),
            nonce: todo!(),
            network: todo!(),
            anchorMode: todo!(),
            postConditionMode: todo!(),
            postConditions: todo!(),
            validateWithAbi: todo!(),
            sponsored: todo!(),
            senderKey: self.sender_key,
        };
        let x = self.call(&input);
        serde_json::to_string(&x).unwrap()
    }
    fn burn(&mut self, op: &PegOutRequestOp) -> String {
        let input = SignedContractCallOptions {
            contractAddress: todo!(),
            contractName: todo!(),
            functionName: todo!(),
            functionArgs: todo!(),
            fee: todo!(),
            feeEstimateApiUrl: todo!(),
            nonce: todo!(),
            network: todo!(),
            anchorMode: todo!(),
            postConditionMode: todo!(),
            postConditions: todo!(),
            validateWithAbi: todo!(),
            sponsored: todo!(),
            senderKey: self.sender_key,
        };
        let x = self.call(&input);
        serde_json::to_string(&x).unwrap()
    }
    fn set_wallet_address(&mut self, address: PegWalletAddress) -> String {
        let input = SignedContractCallOptions {
            contractAddress: todo!(),
            contractName: todo!(),
            functionName: todo!(),
            functionArgs: todo!(),
            fee: todo!(),
            feeEstimateApiUrl: todo!(),
            nonce: todo!(),
            network: todo!(),
            anchorMode: todo!(),
            postConditionMode: todo!(),
            postConditions: todo!(),
            validateWithAbi: todo!(),
            sponsored: todo!(),
            senderKey: self.sender_key,
        };
        let x = self.call(&input);
        serde_json::to_string(&x).unwrap()
    }
}
