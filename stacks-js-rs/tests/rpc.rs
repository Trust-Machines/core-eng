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
use stackes_js_rs::{In, StacksWalletJs};
use stacks_coordinator::peg_wallet::{PegWalletAddress, StacksWallet};
use yarpc::{js::Js, rpc::Rpc};

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
    let mut js = Js::new("../yarpc/js/mirror.ts").unwrap();
    let result: serde_json::Value = js.call(&x).unwrap();
    let expected = r#"{"Mint":{"amount":0,"block_height":0,"burn_header_hash":"0000000000000000000000000000000000000000000000000000000000000000","memo":"","peg_wallet_address":"1EXCN4m6mNL88QzPwksBnpVqr5F1dC4SGa","recipient":"S0000000000000000000002AA028H","txid":"0000000000000000000000000000000000000000000000000000000000000000","vtxindex":0}}"#;
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
    let mut js = Js::new("../yarpc/js/mirror.ts").unwrap();
    let result: serde_json::Value = js.call(&x).unwrap();
    let expected = r#"{"Burn":{"amount":0,"block_height":0,"burn_header_hash":"0000000000000000000000000000000000000000000000000000000000000000","fulfillment_fee":0,"memo":"","peg_wallet_address":"1EXCN4m6mNL88QzPwksBnpVqr5F1dC4SGa","recipient":"1EXCN4m6mNL88QzPwksBnpVqr5F1dC4SGa","signature":"0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000","txid":"0000000000000000000000000000000000000000000000000000000000000000","vtxindex":0}}"#;
    assert_eq!(serde_json::to_string(&result).unwrap(), expected);
}

#[test]
fn mirror_set_wallet_address_test() {
    let p = PegWalletAddress([0; 32]);
    let x = In::SetWalletAddress(&p);
    let mut js = Js::new("../yarpc/js/mirror.ts").unwrap();
    let result: serde_json::Value = js.call(&x).unwrap();
    let expected =
        r#"{"SetWalletAddress":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]}"#;
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

#[test]
fn stacks_mint_test() {
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
    let mut wallet = StacksWalletJs::new(Js::new("./js/stacks.ts").unwrap());
    let result = wallet.mint(&p);
    assert_eq!(result, "Mint");
}

#[test]
fn stacks_burn_test() {
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
    let mut wallet = StacksWalletJs::new(Js::new("./js/stacks.ts").unwrap());
    let result = wallet.burn(&p);
    assert_eq!(result, "Burn");
}

#[test]
fn stacks_set_wallet_address_test() {
    let p = PegWalletAddress([0; 32]);
    let mut wallet = StacksWalletJs::new(Js::new("./js/stacks.ts").unwrap());
    let result = wallet.set_wallet_address(p);
    assert_eq!(result, "SetWalletAddress");
}
