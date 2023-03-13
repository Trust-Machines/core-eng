use bitcoin::hashes::hex::{FromHex, ToHex};
use bitcoin::hashes::Hash;
use bitcoin::Script;
use serde::Serialize;

use crate::bitcoin_node;
use crate::bitcoin_node::BitcoinTransaction;
use crate::error::Result;
use crate::stacks_node;
use crate::stacks_node::{PegInOp, PegOutRequestOp};
use crate::stacks_transaction::StacksTransaction;

pub trait StacksWallet {
    fn mint(&mut self, op: &stacks_node::PegInOp) -> Result<StacksTransaction>;
    fn burn(&mut self, op: &stacks_node::PegOutRequestOp) -> Result<StacksTransaction>;
    fn set_wallet_address(&mut self, address: PegWalletAddress) -> Result<StacksTransaction>;
}

pub trait BitcoinWallet {
    fn fulfill_peg_out(
        &self,
        op: &stacks_node::PegOutRequestOp,
    ) -> Result<bitcoin_node::BitcoinTransaction>;
}

pub trait PegWallet {
    type StacksWallet: StacksWallet;
    type BitcoinWallet: BitcoinWallet;
    fn stacks_mut(&mut self) -> &mut Self::StacksWallet;
    fn bitcoin_mut(&mut self) -> &mut Self::BitcoinWallet;
}

// TODO: Representation
// Should correspond to a [u8; 32] - perhaps reuse a FROST type?
#[derive(Serialize)]
pub struct PegWalletAddress(pub [u8; 32]);

pub struct WrapPegWallet {}

impl PegWallet for WrapPegWallet {
    type StacksWallet = FileStacksWallet;
    type BitcoinWallet = FileBitcoinWallet;

    fn stacks_mut(&mut self) -> &mut Self::StacksWallet {
        todo!()
    }

    fn bitcoin_mut(&mut self) -> &mut Self::BitcoinWallet {
        todo!()
    }
}

pub struct FileStacksWallet {}

impl StacksWallet for FileStacksWallet {
    fn mint(&mut self, _op: &PegInOp) -> Result<StacksTransaction> {
        todo!()
    }

    fn burn(&mut self, _op: &PegOutRequestOp) -> Result<StacksTransaction> {
        todo!()
    }

    fn set_wallet_address(&mut self, _address: PegWalletAddress) -> Result<StacksTransaction> {
        todo!()
    }
}

pub struct FileBitcoinWallet {}

impl BitcoinWallet for FileBitcoinWallet {
    fn fulfill_peg_out(&self, op: &PegOutRequestOp) -> Result<BitcoinTransaction> {
        let bitcoin_txid = bitcoin::Txid::from_slice(op.txid.as_bytes())?;
        let utxo = bitcoin::OutPoint {
            txid: bitcoin_txid,
            vout: op.vtxindex,
        };
        let peg_out_input = bitcoin::TxIn {
            previous_output: utxo,
            script_sig: Default::default(),
            sequence: Default::default(),
            witness: Default::default(),
        };
        //let p2wpk = bitcoin::Script::new_v0_p2wpkh(&user_address.wpubkey_hash().unwrap());
        let peg_out_output_stx = op.recipient.to_bitcoin_tx_out(op.amount);
        let peg_out_script = Script::from_hex(&peg_out_output_stx.script_pubkey.to_hex())?;
        let peg_out_output = bitcoin::TxOut {
            value: peg_out_output_stx.value,
            script_pubkey: peg_out_script,
        };
        Ok(bitcoin::blockdata::transaction::Transaction {
            version: 0,
            lock_time: bitcoin::PackedLockTime(0),
            input: vec![peg_out_input],
            output: vec![peg_out_output],
        })
    }
}

#[cfg(test)]
mod tests {
    use blockstack_lib::burnchains::Txid;
    use blockstack_lib::chainstate::stacks::address::{PoxAddress, PoxAddressType20};
    use blockstack_lib::types::chainstate::BurnchainHeaderHash;
    use blockstack_lib::util::secp256k1::MessageSignature;

    use crate::peg_wallet::{BitcoinWallet, FileBitcoinWallet};
    use crate::stacks_node::PegOutRequestOp;

    #[test]
    fn fufill_peg_out() {
        let wallet = FileBitcoinWallet {};
        let recipient = PoxAddress::Addr20(true, PoxAddressType20::P2WPKH, [0x01; 20]);
        let peg_wallet_address = PoxAddress::Addr20(true, PoxAddressType20::P2WPKH, [0x01; 20]);
        let req_op = PegOutRequestOp {
            amount: 1000,
            recipient: recipient,
            signature: MessageSignature([0x00; 65]),
            peg_wallet_address: peg_wallet_address,
            fulfillment_fee: 0,
            memo: vec![],
            txid: Txid([0x04; 32]),
            vtxindex: 0,
            block_height: 0,
            burn_header_hash: BurnchainHeaderHash([0x00; 32]),
        };
        let btc_tx = wallet.fulfill_peg_out(&req_op).unwrap();
        assert_eq!(btc_tx.output[0].value, 1000)
    }
}
