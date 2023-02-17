use std::io::Error;

use blockstack_lib::{
    burnchains::Txid,
    chainstate::{burn::operations::PegInOp, stacks::address::PoxAddress},
    types::chainstate::{BurnchainHeaderHash, StacksAddress},
    util::hash::Hash160,
    vm::types::{PrincipalData, StandardPrincipalData},
};
use serde_json::{from_str, to_string, Value};
use stackes_js_rs::{fee_wallet_js::In, Js};

fn to_value(s: &str) -> Result<Value, Error> {
    let x = from_str(s)?;
    Ok(x)
}

fn test_wrap() -> Result<(), Error> {
    let mut js = Js::new("./mirror.mjs")?;
    {
        let result = js.call::<_, Value>(&to_value("{\"b\":[],\"a\":2}")?)?;
        assert_eq!(result.to_string(), "[{\"a\":2,\"b\":[]}]");
    }
    {
        let result = js.call::<_, Value>(&to_value("[54,null]")?)?;
        assert_eq!(result.to_string(), "[[54,null]]");
    }
    {
        let result = js.call::<_, Value>(&to_value("42")?)?;
        assert_eq!(result.to_string(), "[42]");
    }
    {
        let result = js.call::<_, Value>(&to_value("\"Hello!\"")?)?;
        assert_eq!(result.to_string(), "[\"Hello!\"]");
    }
    {
        let result = js.call::<_, Value>(&to_value("true")?)?;
        assert_eq!(result.to_string(), "[true]");
    }
    {
        let result = js.call::<_, Value>(&to_value("null")?)?;
        assert_eq!(result.to_string(), "[null]");
    }
    Ok(())
}

#[test]
fn test() {
    test_wrap().unwrap();
}

#[test]
fn in_test() {
    let p = PegInOp {
        recipient: PrincipalData::Standard(StandardPrincipalData(0, [0; 20])),
        peg_wallet_address: PoxAddress::Standard(
            StacksAddress::new(0, Hash160::from_data(&[0; 20])),
            None,
        ),
        amount: 0,
        memo: Vec::default(),
        txid: Txid([0; 32]),
        vtxindex: 0,
        block_height: 0,
        burn_header_hash: BurnchainHeaderHash([0; 32]),
    };
    let x = In::Mint(&p);
    let mut js = Js::new("./mirror.mjs").unwrap();
    let result = js.call::<_, Value>(&x).unwrap();
    let expected = r#"[{"Mint":{"amount":0,"block_height":0,"burn_header_hash":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"memo":[],"peg_wallet_address":{"Standard":[{"bytes":"944f997c5553a6f3e1028e707c71b5fa0dd3afa7","version":0},null]},"recipient":{"Standard":[0,[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]]},"txid":"0000000000000000000000000000000000000000000000000000000000000000","vtxindex":0}}]"#;
    assert_eq!(to_string(&result).unwrap(), expected);
}
