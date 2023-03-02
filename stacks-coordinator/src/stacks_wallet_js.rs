use crate::{
    make_contract_call::{MakeContractCall, SignedContractCallOptions, ANY},
    peg_wallet::{PegWalletAddress, StacksWallet},
    stacks_node::{PegInOp, PegOutRequestOp},
    stacks_transaction::StacksTransaction,
};

pub struct StacksWalletJs {
    make_contract_call: MakeContractCall,
    contract_address: String,
    sender_key: String,
}

impl StacksWalletJs {
    pub fn new(path: &str, contract_address: String, sender_key: String) -> Self {
        Self {
            make_contract_call: MakeContractCall::new(path),
            contract_address,
            sender_key,
        }
    }
    fn call(&mut self, function_name: String) -> StacksTransaction {
        let input = SignedContractCallOptions {
            contractAddress: self.contract_address.clone(),
            contractName: "".to_string(),
            functionName: function_name,
            functionArgs: Vec::default(),
            fee: Some(0.to_string()),
            feeEstimateApiUrl: None,
            nonce: None,
            network: None,
            anchorMode: ANY,
            postConditionMode: None,
            postConditions: None,
            validateWithAbi: None,
            sponsored: None,
            senderKey: self.sender_key.clone(),
        };
        self.make_contract_call.call(&input)
    }
}

impl StacksWallet for StacksWalletJs {
    fn mint(&mut self, _op: &PegInOp) -> StacksTransaction {
        self.call("mint".to_string())
    }
    fn burn(&mut self, _op: &PegOutRequestOp) -> StacksTransaction {
        self.call("burn".to_string())
    }
    fn set_wallet_address(&mut self, _address: PegWalletAddress) -> StacksTransaction {
        self.call("set_wallet_address".to_string())
    }
}
