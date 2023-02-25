use stacks_coordinator::make_contract_call::{MakeContractCall, SignedContractCallOptions, ANY};

#[test]
fn make_contract_call_test() {
    let mut c = MakeContractCall::new("..");
    let input = SignedContractCallOptions {
        contractAddress: "SPBMRFRPPGCDE3F384WCJPK8PQJGZ8K9QKK7F59X".to_string(),
        contractName: "".to_string(),
        functionName: "mint".to_string(),
        functionArgs: Vec::default(),
        fee: Some("0".to_string()),
        feeEstimateApiUrl: None,
        nonce: None,
        network: None,
        anchorMode: ANY,
        postConditionMode: None,
        postConditions: None,
        validateWithAbi: None,
        sponsored: None,
        senderKey: "0001020304050607080910111213141516171819202122232425262728293031".to_string(),
    };
    {
        let input_s = serde_json::to_string(&input).unwrap();
        println!("{input_s}");
    }
    let t = c.call(&input);
    let s = serde_json::to_string(&t).unwrap();
    let expected = r#"{"version":0,"chainId":1,"auth":{"authType":4,"spendingCondition":{"fee":"0","hashMode":0,"keyEncoding":1,"nonce":"0","signature":{"data":"01b0683fa38065ac869ec933f6768c11e10976a996c2612f67fb9fe09ed168cae6449c9c2b0f9330e162820542847ec4847762862b90560bb52a69eaecd7b51f76","type":9},"signer":"12016c066cb72c7098a01564eeadae379a266ec1"}},"anchorMode":3,"payload":{"contractAddress":{"hash160":"174c3f16b418d70de34138c95a68b5e50fa269bc","type":0,"version":22},"contractName":{"content":"","lengthPrefixBytes":1,"maxLengthBytes":128,"type":2},"functionArgs":[],"functionName":{"content":"mint","lengthPrefixBytes":1,"maxLengthBytes":128,"type":2},"payloadType":2,"type":8},"postConditionMode":2,"postConditions":{"lengthPrefixBytes":4,"type":7,"values":[]}}"#;
    assert_eq!(s, expected);
}
