import json
from types import SimpleNamespace
import click
import meat
import sign


@click.command('dl', help='Download UTXOs from BitGo API and input transactions from Trezor API')
@click.option('--input-address', type=str, help='Supported: SPENDP2SHWITNESS (P2WPKH)')
@click.option('--outfile', type=str, default='utxo.json', help='File to store the UTXOs. Default: utxo.json')
# @click.option('--utxo-id-use-cache', is_flag=True, default=False, help='Use cached UTXOs. Default: False')
# @click.option('--utxo-id-cache-file', type=str, default='utxo_id_cache.json',
#               help='File to store the cached UTXOs. Default: utxo_id_cache.json')
@click.option('--utxo-fetch-limit', type=int, default=10_000, help='Maximum number of UTXOs to fetch. Default: 10,000')
# @click.option('--utxo-id-max-count', type=int, default=10,
#               help='Maximum number of UTXOs to include in consolidation. Default: 10')
# @click.option('--utxo-id-max-value', type=int, default=100_000,
#               help='Maximum value of UTXOs include in consolidation. Default: 100,000 sats')
# @click.option('--utxo-tx-use-cache', is_flag=True, default=False, help='Use cached transaction. Default: False')
# @click.option('--utxo-tx-cache-file', type=str, default='utxo_tx_cache.json',
#               help='File to store the cached transaction. Default: utxo_tx_cache.json')
def dl(**args):
    click.echo(f'dl - Download UTXOs from BitGo API and input transactions from Trezor API')
    click.echo(f'args: {args}')
    args = SimpleNamespace(**args)
    # click.echo(f{args.input_address=} {args.outfile}')
    assert args.input_address, 'Please specify an address'

    utxo_id_list = meat.load_utxo_id_list(args)
    utxo_dict_list = meat.load_utxo_dict_list(utxo_id_list, args)
    click.echo(f'Writing {len(utxo_dict_list)} UTXOs to {args.outfile}')
    with open(args.outfile, 'w') as f:
        json.dump(utxo_dict_list, f, indent=2)


@click.command('cl', help='Clear the UTXO cache')
@click.option('--utxo-file', type=str, default='utxo.json', help='File to store the UTXOs. Default: utxo.json')
def cl(**args):
    click.echo(f'cl - Clear the contents of the UTXO cache')
    click.echo(f'args: {args}')
    args = SimpleNamespace(**args)


@click.command('ls', help='List the contents of the UTXO cache')
@click.option('--utxo-file', type=str, default='utxo.json', help='File to store the UTXOs. Default: utxo.json')
@click.option('--utxo-filter-query', type=str, default=None, help='Pandas SQL-like query to filter UTXOs. Default: None (select all)')
def ls(**args):
    click.echo(f'ls - List the contents of the UTXO cache')
    click.echo(f'args: {args}')
    args = SimpleNamespace(**args)
    with open(args.utxo_file, 'r') as f:
        utxo_dict_list = json.load(f)
    df_utxo = meat.filter_utxos(utxo_dict_list, args.utxo_filter_query)
    column_names = ', '.join(list(df_utxo.columns))
    click.echo(f'{column_names}')
    for utxo in df_utxo.itertuples():
        click.echo(f'{utxo.index:4,} {utxo.value:12,} {utxo.txid}')


@click.command('tx', help='Create a consolidation transaction using cached UTXOs')
@click.option('--utxo-file', type=str, default='utxo.json', help='File to store the UTXOs. Default: utxo.json')
@click.option('--utxo-filter-query', type=str, default=None, help='Pandas SQL-like query to filter UTXOs. Default: None (select all)')
@click.option('--output-address', type=str, help='Supported: SPENDP2SHWITNESS (P2WPKH)')
@click.option('--input-path', type=str, default="m/49'/0'/0'/0/0",
              help='HD path to use for signing the transaction. Default: m/49\'/0\'/0\'/0/0 (SPENDP2SHWITNESS)')
# @click.option('--output-path', type=str, default="m/49'/0'/0'/0/0",
#               help='HD path to use for signing the transaction. Default: m/49\'/0\'/0\'/0/0 (SPENDP2SHWITNESS)')
@click.option('--est-fee-sats-per-vbyte', type=int, default=10,
              help='Estimated fee per vbyte. Default: 10 sats/vbyte. (This is a very rough estimate.)')
@click.option('--trezor-tx-file', type=str, default='trezor_tx.json',
              help='File to store the Trezor transaction. Default: trezor_tx.json')
# @click.option('--sign', is_flag=True, default=False, help='Sign the transaction with the Trezor. Default: False')
def tx(**args):
    click.echo(f'tx - Create a consolidation transaction using cached UTXOs')
    click.echo(f'args: {args}')
    args = SimpleNamespace(**args)
    assert args.output_address, 'Please specify an output address'

    click.echo(f'Loading UTXOs from {args.utxo_file}')
    with open(args.utxo_file, 'r') as f:
        utxo_dict_list = json.load(f)
    df_utxo = meat.filter_utxos(utxo_dict_list, args.utxo_filter_query)
    utxo_dict_list_filtered = []
    for utxo in df_utxo.itertuples():
        utxo_dict_list_filtered.append(utxo_dict_list[utxo.index])
    trezor_tx = meat.build_trezor_tx(utxo_dict_list_filtered, args)
    click.echo(f'Writing Trezor transaction to {args.trezor_tx_file}')
    with open(args.trezor_tx_file, 'w') as f:
        json.dump(trezor_tx, f, indent=2)


@click.command('sg', help="Sign a transaction with the Trezor")
@click.option('--trezor-tx-file', type=str, default='trezor_tx.json')
@click.option('--signed-tx-file', type=str, default='signed_tx.txt')
def sg(**args):
    click.echo('sg - Sign a transaction with the Trezor')
    click.echo(f'args: {args}')
    args = SimpleNamespace(**args)

    click.echo(f'Loading Trezor transaction from {args.trezor_tx_file}')
    with open(args.trezor_tx_file, 'r') as f:
        trezor_tx = json.load(f)
    click.echo(f'Signing transaction')
    serialized_tx = None
    try:
        serialized_tx = sign.sign(trezor_tx)
        click.echo("Signed Transaction")
        click.echo(serialized_tx.hex())
        click.echo()
    except Exception as e:
        click.echo(f'Error: {e}')

    if serialized_tx:
        click.echo(f'Writing signed transaction to {args.signed_tx_file}')
        with open(args.signed_tx_file, 'w') as f:
            f.write(serialized_tx.hex())
