use serde::{Deserialize, Serialize};

use crate::make_contract_call::{
    AnchorMode, Authorization, ChainID, LengthPrefixedList, Payload, PostConditionMode,
    TransactionVersion,
};

// TODO: Find appropriate type that's compatible with stacks.js JSON
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct StacksTransaction {
    pub version: TransactionVersion,
    pub chainId: ChainID,
    pub auth: Authorization,
    pub anchorMode: AnchorMode,
    pub payload: Payload,
    pub postConditionMode: PostConditionMode,
    pub postConditions: LengthPrefixedList,
}
