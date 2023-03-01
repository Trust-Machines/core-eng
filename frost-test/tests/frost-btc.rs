use wtfrost::v1;

//
#[test]
fn frost_dkg_btc() {
    let signers: Vec<v1::Signer> = vec![];

    // DKG

    // Peg-in: spend a P2PKH utxo and lock it into P2TR output script using the frost public aggregate key.

    // Peg-out: spend the output from the Peg-in tx using frost sign.
}
