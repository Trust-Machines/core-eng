use std::io::Error;

use blockstack_lib::{
    burnchains::Txid,
    chainstate::{
        burn::operations::{PegInOp, PegOutRequestOp},
        stacks::address::PoxAddress,
    },
    types::chainstate::{BurnchainHeaderHash, StacksAddress},
    util::{hash::Hash160, secp256k1::MessageSignature},
    vm::types::{PrincipalData, StandardPrincipalData},
};
use stackes_js_rs::{stacks_wallet_js::In, Js};
use stacks_coordinator::peg_wallet::PegWalletAddress;

fn to_value(s: &str) -> Result<serde_json::Value, Error> {
    let x = serde_json::from_str(s)?;
    Ok(x)
}

fn json_call(js: &mut Js, input: &str) -> Result<String, Error> {
    Ok(js
        .call::<_, serde_json::Value>(&to_value(input)?)?
        .to_string())
}

fn test_wrap() -> Result<(), Error> {
    let mut js = Js::new("./js/mirror.ts")?;
    assert_eq!(
        json_call(&mut js, "{\"b\":[],\"a\":2}")?,
        "[{\"a\":2,\"b\":[]}]"
    );
    assert_eq!(json_call(&mut js, "[54,null]")?, "[[54,null]]");
    assert_eq!(json_call(&mut js, "42")?, "[42]");
    assert_eq!(json_call(&mut js, "\"Hello!\"")?, "[\"Hello!\"]");
    assert_eq!(json_call(&mut js, "true")?, "[true]");
    assert_eq!(json_call(&mut js, "null")?, "[null]");
    Ok(())
}

#[test]
fn test() {
    test_wrap().unwrap();
}

fn pox_address() -> PoxAddress {
    PoxAddress::Standard(StacksAddress::new(0, Hash160::from_data(&[0; 20])), None)
}

#[test]
fn mirror_peg_in_op_test() {
    let p = PegInOp {
        recipient: PrincipalData::Standard(StandardPrincipalData(0, [0; 20])),
        peg_wallet_address: pox_address(),
        amount: 0,
        memo: Vec::default(),
        txid: Txid([0; 32]),
        vtxindex: 0,
        block_height: 0,
        burn_header_hash: BurnchainHeaderHash([0; 32]),
    };
    let x = In::Mint(&p);
    let mut js = Js::new("./js/mirror.ts").unwrap();
    let result: serde_json::Value = js.call(&x).unwrap();
    let expected = r#"[{"Mint":{"amount":0,"block_height":0,"burn_header_hash":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"memo":[],"peg_wallet_address":{"Standard":[{"bytes":"944f997c5553a6f3e1028e707c71b5fa0dd3afa7","version":0},null]},"recipient":{"Standard":[0,[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]]},"txid":"0000000000000000000000000000000000000000000000000000000000000000","vtxindex":0}}]"#;
    assert_eq!(serde_json::to_string(&result).unwrap(), expected);
}

#[test]
fn mirror_peg_out_request_op_test() {
    let p = PegOutRequestOp {
        amount: 0,
        recipient: pox_address(),
        signature: MessageSignature([0; 65]),
        peg_wallet_address: pox_address(),
        fulfillment_fee: 0,
        memo: Vec::default(),
        txid: Txid([0; 32]),
        vtxindex: 0,
        block_height: 0,
        burn_header_hash: BurnchainHeaderHash([0; 32]),
    };
    let x = In::Burn(&p);
    let mut js = Js::new("./js/mirror.ts").unwrap();
    let result: serde_json::Value = js.call(&x).unwrap();
    let expected = r#"[{"Burn":{"amount":0,"block_height":0,"burn_header_hash":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"fulfillment_fee":0,"memo":[],"peg_wallet_address":{"Standard":[{"bytes":"944f997c5553a6f3e1028e707c71b5fa0dd3afa7","version":0},null]},"recipient":{"Standard":[{"bytes":"944f997c5553a6f3e1028e707c71b5fa0dd3afa7","version":0},null]},"signature":"0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000","txid":"0000000000000000000000000000000000000000000000000000000000000000","vtxindex":0}}]"#;
    assert_eq!(serde_json::to_string(&result).unwrap(), expected);
}

#[test]
fn stacks_mint() {
    let p = PegInOp {
        recipient: PrincipalData::Standard(StandardPrincipalData(0, [0; 20])),
        peg_wallet_address: pox_address(),
        amount: 0,
        memo: Vec::default(),
        txid: Txid([0; 32]),
        vtxindex: 0,
        block_height: 0,
        burn_header_hash: BurnchainHeaderHash([0; 32]),
    };
    let x = In::Mint(&p);
    let mut js = Js::new("./js/stacks.ts").unwrap();
    let result: String = js.call(&x).unwrap();
    assert_eq!(result, "Mint");
}

#[test]
fn stacks_burn() {
    let p = PegOutRequestOp {
        amount: 0,
        recipient: pox_address(),
        signature: MessageSignature([0; 65]),
        peg_wallet_address: pox_address(),
        fulfillment_fee: 0,
        memo: Vec::default(),
        txid: Txid([0; 32]),
        vtxindex: 0,
        block_height: 0,
        burn_header_hash: BurnchainHeaderHash([0; 32]),
    };
    let x = In::Burn(&p);
    let mut js = Js::new("./js/stacks.ts").unwrap();
    let result: String = js.call(&x).unwrap();
    assert_eq!(result, "Burn");
}

#[test]
fn stacks_set_wallet_address() {
    let p = PegWalletAddress([0; 32]);
    let x = In::SetWalletAddress(&p);
    let mut js = Js::new("./js/stacks.ts").unwrap();
    let result: String = js.call(&x).unwrap();
    assert_eq!(result, "SetWalletAddress");
}
