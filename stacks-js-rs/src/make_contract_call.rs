use serde::{Deserialize, Serialize};
use yarpc::{js::Js, rpc::Rpc};

pub type ClarityValue = serde_json::Value;

pub type PostCondition = serde_json::Value;

pub type IntegerType = serde_json::Value;

pub type StacksNetworkNameOrStacksNetwork = serde_json::Value;

pub type BooleanOrClarityAbi = serde_json::Value;

#[allow(non_snake_case)]
#[derive(Serialize)]
pub struct SignedContractCallOptions {
    pub contractAddress: String,
    pub contractName: String,
    pub functionName: String,
    pub functionArgs: Vec<ClarityValue>,
    pub fee: Option<IntegerType>,
    pub feeEstimateApiUrl: Option<String>,
    pub nonce: Option<IntegerType>,
    pub network: Option<StacksNetworkNameOrStacksNetwork>,
    pub anchorMode: AnchorMode,
    pub postConditionMode: Option<PostConditionMode>,
    pub postConditions: Option<PostCondition>,
    pub validateWithAbi: Option<BooleanOrClarityAbi>,
    pub sponsored: Option<bool>,
}

pub type TransactionVersion = serde_json::Number;

pub type ChainID = serde_json::Number;

pub type Authorization = serde_json::Value;

type AnchorMode = serde_json::Value;

type Payload = serde_json::Value;

type PostConditionMode = serde_json::Value;

type LengthPrefixedList = serde_json::Value;

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct StacksTransaction {
    pub version: TransactionVersion,
    pub chainId: ChainID,
    pub auth: Authorization,
    pub anchorMode: AnchorMode,
    pub payload: Payload,
    pub postConditionMode: PostConditionMode,
    pub postConditions: LengthPrefixedList,
}

pub struct MakeContractCall(Js);

impl MakeContractCall {
    pub fn call(&mut self, input: &SignedContractCallOptions) -> StacksTransaction {
        self.0.call(input).unwrap()
    }
}
