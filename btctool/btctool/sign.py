import click
from trezorlib import btc, protobuf
from trezorlib import messages
from trezorlib.client import get_default_client


def sign(trezor_tx):
    client = get_default_client()
    coin_name = trezor_tx["coin_name"]
    details = trezor_tx.get("details", {})
    inputs = [
        protobuf.dict_to_proto(messages.TxInputType, i) for i in trezor_tx.get("inputs", ())
    ]
    outputs = [
        protobuf.dict_to_proto(messages.TxOutputType, output)
        for output in trezor_tx.get("outputs", ())
    ]
    prev_txes = {
        bytes.fromhex(txid): protobuf.dict_to_proto(messages.TransactionType, tx)
        for txid, tx in trezor_tx.get("prev_txes", {}).items()
    }

    _, serialized_tx = btc.sign_tx(
        client,
        coin_name,
        inputs,
        outputs,
        prev_txes=prev_txes,
        **details,
    )

    return serialized_tx
