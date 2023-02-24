use serde_json::Number;
use stackes_js_rs::make_contract_call::{MakeContractCall, SignedContractCallOptions};

#[test]
fn make_contract_call_test() {
    let mut c = MakeContractCall::new("..");
    let input = SignedContractCallOptions {
        contractAddress: "SPBMRFRPPGCDE3F384WCJPK8PQJGZ8K9QKK7F59X".to_string(),
        contractName: "".to_string(),
        functionName: "mint".to_string(),
        functionArgs: Vec::default(),
        fee: Some(serde_json::Value::String("0".to_string())),
        feeEstimateApiUrl: None,
        nonce: None,
        network: None,
        anchorMode: serde_json::Value::Number(3.into()),
        postConditionMode: None,
        postConditions: None,
        validateWithAbi: None,
        sponsored: None,
        senderKey: "0001020304050607080910111213141516171819202122232425262728293031".to_string(),
    };
    let t = c.call(&input);
}
