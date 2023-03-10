use blockstack_lib::vm::Value;
use stacks_coordinator::make_contract_call::{MakeContractCall, SignedContractCallOptions, ANY};

#[test]
fn make_contract_call_test() {
    let mut c = MakeContractCall::new("..").unwrap();
    let input = SignedContractCallOptions::new(
        "SPBMRFRPPGCDE3F384WCJPK8PQJGZ8K9QKK7F59X",
        "",
        "mint",
        &[Value::UInt(42)],
        ANY,
        "0001020304050607080910111213141516171819202122232425262728293031",
    )
    .with_fee(0);
    {
        let input_s = serde_json::to_string(&input).unwrap();
        println!("{input_s}");
    }
    let t = c.call(&input).unwrap();
    let s = serde_json::to_string(&t).unwrap();
    let expected = r#"{"version":0,"chainId":1,"auth":{"authType":4,"spendingCondition":{"fee":"0","hashMode":0,"keyEncoding":1,"nonce":"0","signature":{"data":"007b8e678be460d78e6c0aee43d13d7765694decc9b2bb26ba90cee89a250530cd7cc3514411c85eba326f1ca0bae21f9e9467ed0ff77df57db99a04c1d605440e","type":9},"signer":"12016c066cb72c7098a01564eeadae379a266ec1"}},"anchorMode":3,"payload":{"contractAddress":{"hash160":"174c3f16b418d70de34138c95a68b5e50fa269bc","type":0,"version":22},"contractName":{"content":"","lengthPrefixBytes":1,"maxLengthBytes":128,"type":2},"functionArgs":[{"type":1,"value":"42"}],"functionName":{"content":"mint","lengthPrefixBytes":1,"maxLengthBytes":128,"type":2},"payloadType":2,"type":8},"postConditionMode":2,"postConditions":{"lengthPrefixBytes":4,"type":7,"values":[]}}"#;
    assert_eq!(s, expected);
}
