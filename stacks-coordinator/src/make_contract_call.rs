use std::path::Path;

use serde::{Deserialize, Serialize};
use yarpc::{dispatch_command::DispatchCommand, js::Js, rpc::Rpc};

use crate::stacks_transaction::StacksTransaction;

pub type ClarityValue = serde_json::Value;

pub type PostCondition = serde_json::Value;

// number | string | bigint | Uint8Array | BN;
pub type IntegerType = String;

pub type StacksNetworkNameOrStacksNetwork = serde_json::Value;

pub type BooleanOrClarityAbi = serde_json::Value;

#[allow(non_snake_case)]
#[derive(Serialize)]
pub struct SignedContractCallOptions {
    pub contractAddress: String,

    pub contractName: String,

    pub functionName: String,

    pub functionArgs: Vec<ClarityValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<IntegerType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub feeEstimateApiUrl: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<IntegerType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<StacksNetworkNameOrStacksNetwork>,

    pub anchorMode: AnchorMode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub postConditionMode: Option<PostConditionMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub postConditions: Option<PostCondition>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub validateWithAbi: Option<BooleanOrClarityAbi>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sponsored: Option<bool>,

    pub senderKey: String,
}

pub type TransactionVersion = serde_json::Number;

pub type ChainID = serde_json::Number;

pub type Authorization = serde_json::Value;

pub type AnchorMode = u8;

pub const ON_CHAIN_ONLY: AnchorMode = 1;
pub const OFF_CHAIN_ONLY: AnchorMode = 2;
pub const ANY: AnchorMode = 3;

pub type Payload = serde_json::Value;

pub type PostConditionMode = serde_json::Value;

pub type LengthPrefixedList = serde_json::Value;

pub struct MakeContractCall(Js);

impl MakeContractCall {
    pub fn call(&mut self, input: &SignedContractCallOptions) -> StacksTransaction {
        self.0
            .call(&DispatchCommand("makeContractCall".to_string(), input))
            .unwrap()
    }
    pub fn new(path: &str) -> Self {
        let file_name = Path::new(path).join("yarpc/js/stacks/transactions.ts");
        Self(Js::new(file_name.to_str().unwrap()).unwrap())
    }
}
