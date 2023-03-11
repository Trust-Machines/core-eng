import json

import click
import pandas as pd
from trezorlib.protobuf import to_dict

import tx_utils
import utxo_utils


def filter_utxos(utxo_dict_list, query):
    utxo_value_list = []
    utxo_txid_list = []
    for utxo_dict in utxo_dict_list:
        utxo_value_list.append(utxo_dict['utxo_id']['value'])
        utxo_txid_list.append(utxo_dict['utxo_id']['txid'])
    df_utxo = pd.DataFrame.from_dict(dict(value=utxo_value_list, txid=utxo_txid_list))
    df_utxo.reset_index(inplace=True)
    if query:
        df_utxo = df_utxo.query(query)
    return df_utxo


def load_utxo_id_list(args):
    # if args.utxo_id_use_cache:
    #     click.echo(f'Loading cached utxo-id list from {args.utxo_id_cache_file=}')
    #     with open(args.utxo_id_cache_file, 'r') as f:
    #         utxo_id_list = json.load(f)
    #     return utxo_id_list

    click.echo(f'Fetching utxo-id list from BitGo API {args.utxo_fetch_limit=:,}')
    utxo_id_list = utxo_utils.fetch_utxo_id_list_for_address(args.input_address, args)
    # with open(args.utxo_id_cache_file, 'w') as f:
    #     json.dump(utxo_id_list, f, indent=2)
    # click.echo(f'Found {len(utxo_id_list):,} utxo ids')

    # utxo_id_list.sort(key=lambda u: u['value'])
    # utxo_id_list = utxo_id_list[:args.utxo_id_max_count]
    # utxo_id_list = filter(lambda u: u['value'] < args.utxo_id_max_value, utxo_id_list)
    utxo_id_list = list(utxo_id_list)
    utxos_amount = sum([utxo['value'] for utxo in utxo_id_list])
    # click.echo(
    #     f'Filtered {len(utxo_id_list):,} eligible utxos for consolidation amounting to {utxos_amount:,} sats {args.utxo_id_max_count=:,} {args.utxo_id_max_value=:,}')
    return utxo_id_list


def load_utxo_dict_list(utxo_id_list, args):
    # if args.utxo_tx_use_cache:
    #     click.echo(f'Loading cached utxo-tx list from {args.utxo_tx_cache_file=}')
    #     with open(args.utxo_tx_cache_file, 'r') as f:
    #         utxo_dict_list = json.load(f)
    #         return utxo_dict_list

    # desc = f'Fetching {len(utxo_id_list):,} transactions from the Trezor API'
    # click.echo(f'Fetching {len(utxo_id_list):,} transactions from the Trezor API')
    utxo_dict_list = tx_utils.fetch_utxo_dict_list(utxo_id_list)
    # click.echo(f'Fetched {len(utxo_dict_list):,} transactions from the Trezor API')
    # with open(args.utxo_tx_cache_file, 'w') as f:
    #     json.dump(utxo_dict_list, f, indent=2)
    return utxo_dict_list


def build_trezor_tx(utxo_dict_list, args):
    tx_input_dict_list = [tx_utils.build_tx_input_dict(utxo_dict['utxo_id'], utxo_dict['utxo_tx'], args) for utxo_dict
                          in utxo_dict_list]
    tx_inputs = [tx_input_dict['tx_input'] for tx_input_dict in tx_input_dict_list]
    tx_inputs_amount = sum(tx_input.amount for tx_input in tx_inputs)
    click.echo(
        f'Created {len(utxo_dict_list):,} consolidation transaction inputs with total amount {tx_inputs_amount:,} sats')

    # quick and dirty fee calculation
    total_fee = args.est_fee_sats_per_vbyte * 100 * len(utxo_dict_list)
    tx_output_amount = tx_inputs_amount - total_fee
    tx_outputs = tx_utils.get_outputs(args.output_address, tx_output_amount, args)
    click.echo(
        f'Created the consolidation transaction output with total amount {tx_output_amount:,} sats and total fee {total_fee:,} sats')

    coin = 'Bitcoin'
    version = 2
    lock_time = 0

    trezor_tx = {
        "coin_name": coin,
        "inputs": [to_dict(i, hexlify_bytes=True) for i in tx_inputs],
        "outputs": [to_dict(o, hexlify_bytes=True) for o in tx_outputs],
        "details": {
            "version": version,
            "lock_time": lock_time,
        },
        "prev_txes": {
            tx_input_dict['txhash']: to_dict(tx_input_dict['tx'], hexlify_bytes=True)
            for tx_input_dict in tx_input_dict_list
        },
    }

    return trezor_tx

