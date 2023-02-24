pub type ClarityValue = serde_json::Value;

pub type PostCondition = serde_json::Value;

pub type IntegerType = serde_json::Value;

pub type StacksNetworkNameOrStacksNetwork = serde_json::Value;

pub type BooleanOrClarityAbi = serde_json::Value;

#[allow(non_snake_case)]
pub struct SignedContractCallOptions {
    contractAddress: String,
    contractName: String,
    functionName: String,
    functionArgs: Vec<ClarityValue>,
    fee: Option<IntegerType>,
    feeEstimateApiUrl: Option<String>,
    nonce: Option<IntegerType>,
    network: Option<StacksNetworkNameOrStacksNetwork>,
    anchorMode: AnchorMode,
    postConditionMode: Option<PostConditionMode>,
    postConditions: Option<PostCondition>,
    validateWithAbi: Option<BooleanOrClarityAbi>,
    sponsored: Option<bool>,
}

pub type TransactionVersion = serde_json::Number;

pub type ChainID = serde_json::Number;

type Authorization = serde_json::Value;

type AnchorMode = serde_json::Value;

type Payload = serde_json::Value;

type PostConditionMode = serde_json::Value;

type LengthPrefixedList = serde_json::Value;

pub struct StacksTransaction {
    pub version: TransactionVersion,
    pub chainId: ChainID,
    pub auth: Authorization,
    pub anchorMode: AnchorMode,
    pub payload: Payload,
    pub postConditionMode: PostConditionMode,
    pub postConditions: LengthPrefixedList,
}
