import multiprocessing
import time

import requests
import tqdm
from trezorlib import btc, messages, tools

import utils


# the following script type mapping is only valid for single-sig Trezor-generated utxos
BITCOIN_CORE_INPUT_TYPES = {
    "pubkeyhash": messages.InputScriptType.SPENDADDRESS,
    "scripthash": messages.InputScriptType.SPENDP2SHWITNESS,
    "witness_v0_keyhash": messages.InputScriptType.SPENDWITNESS,
    "witness_v1_taproot": messages.InputScriptType.SPENDTAPROOT,
}


class RateLimitException(Exception):
    pass


def get_outputs(address, amount, args):
    outputs = []
    address_n = None
    script_type = messages.OutputScriptType.PAYTOADDRESS

    outputs.append(
        messages.TxOutputType(
            amount=amount,
            address_n=address_n,
            address=address,
            script_type=script_type,
        )
    )

    return outputs


def build_tx_input_dict(utxo_id, utxo_tx, args):
    utxo_txid = bytes.fromhex(utxo_id['txid'])
    address_n = tools.parse_path(args.input_path)
    utxo_index = int(utxo_id['output_n'])
    amount = utxo_id['value']
    reported_type = utxo_tx["vout"][utxo_index]["scriptPubKey"]["type"]
    # print(utxo_tx["vout"][utxo_index])
    # print(utxo_id['value'] == utxo_tx["vout"][utxo_index]['value'])
    # raise SystemExit
    # # import json
    # click.echo(json.dumps(tx_dict, indent=4))
    # click.echo((address_n, args.spend_path))
    # click.echo(json.dumps(utxo_id, indent=2))
    # click.echo(json.dumps(utxo_tx["vout"][utxo_index], indent=2))
    # click.echo({'asm': utxo_tx["vout"][utxo_index]["scriptPubKey"]["asm"], 'type': reported_type})
    # raise Exception
    script_type = BITCOIN_CORE_INPUT_TYPES[reported_type]
    sequence = 0xFFFFFFFD

    tx_input = messages.TxInputType(
        address_n=address_n,
        prev_hash=utxo_txid,
        prev_index=utxo_index,
        amount=amount,
        script_type=script_type,
        sequence=sequence,
    )

    txhash = utxo_txid.hex()
    tx = btc.from_json(utxo_tx)
    return {'tx_input': tx_input, 'txhash': txhash, 'tx': tx}


def fetch_utxo_dict(utxo_id, request_url):
    utxo_txid = bytes.fromhex(utxo_id['txid'])
    tx_hash = utxo_txid.hex()
    request_url = f'{request_url}/{tx_hash}'
    session = requests.Session()
    session.headers.update({"User-Agent": "trezorlib"})
    response = session.get(request_url, timeout=1)
    if not response.ok:
        # raise Exception(tx_url, r.content)
        # click.echo(f'Got HTTP status code {r.status_code} for {tx_url=}')
        raise RateLimitException(request_url, response.content)

    utxo_tx = response.json(parse_float=str)
    # for vo in utxo_tx['vout']:
    #     vo['value'] = int(decimal.Decimal(vo['value']) * 100_000_000)
    return {'utxo_id': utxo_id, 'utxo_tx': utxo_tx}


@utils.ignore_keyboard_interrupt
def request_worker(queue, result_list, error_list, worker_index):
    request_url = f'https://btc{worker_index}.trezor.io/api/tx-specific'
    # request_url = f'https://www.bitgo.com/api/v1/tx'

    min_timeout_secs = 0.5
    max_timeout_secs = 0.5

    timeout_secs = min_timeout_secs
    while True:
        utxo = queue.get()
        try:
            result_list.append(fetch_utxo_dict(utxo, request_url))
            timeout_secs = min_timeout_secs
            time.sleep(timeout_secs)
        # except RateLimitException as ex:
        except Exception as ex:
            queue.put(utxo)
            error_list.append(ex)
            timeout_secs = min(max_timeout_secs, 2 * timeout_secs)
            time.sleep(timeout_secs)
        finally:
            queue.task_done()


def fetch_utxo_dict_list(utxo_id_list):
    manager = multiprocessing.Manager()
    queue = manager.Queue()
    utxo_tx_list = manager.list()
    error_list = manager.list()
    procs = []

    for worker_index in range(1, 6):
        proc = multiprocessing.Process(target=request_worker, args=[queue, utxo_tx_list, error_list, worker_index])
        procs.append(proc)

    for utxo in utxo_id_list:
        queue.put(utxo)

    for proc in procs:
        proc.start()

    result_count = 0
    error_count = 0
    with tqdm.tqdm(total=len(utxo_id_list)) as bar:
        while True:
            queue_empty = queue.empty()
            if queue_empty:
                queue.join()
            new_result_count = len(utxo_tx_list)
            new_error_count = len(error_list)
            if queue_empty or new_result_count > result_count or new_error_count > error_count:
                bar.set_description(
                    f'Fetched {new_result_count:,} transactions from the Trezor API (error retries: {len(error_list):,})')
                bar.update(new_result_count - result_count)
                result_count = new_result_count
                error_count = new_error_count
                time.sleep(0.1)
            if queue_empty:
                break

    bar.close()

    for proc in procs:
        proc.terminate()

    return list(utxo_tx_list)


'''
def sign_tx(trezor_tx):
    trezor_client = client.get_default_client()
    data = trezor_tx
    coin = data["coin_name"]
    details = data.get("details", {})
    inputs = [
        protobuf.dict_to_proto(messages.TxInputType, i) for i in data.get("inputs", ())
    ]
    outputs = [
        protobuf.dict_to_proto(messages.TxOutputType, output)
        for output in data.get("outputs", ())
    ]
    prev_txes = {
        bytes.fromhex(txid): protobuf.dict_to_proto(messages.TransactionType, tx)
        for txid, tx in data.get("prev_txes", {}).items()
    }

    _, serialized_tx = btc.sign_tx(
        trezor_client,
        coin,
        inputs,
        outputs,
        prev_txes=prev_txes,
        **details,
    )

    return serialized_tx.hex()
    
def fetch_inputs_single(utxos):
    inputs = []
    txes = {}
    request_url = f'https://btc1.trezor.io/api/tx-specific'
    while utxos:
        utxo = utxos.pop()
        try:
            response = get_input(utxo, request_url)
            inputs.append(response['tx_input'])
            txes[response['txhash']] = response['tx']
        except RateLimitException:
            utxos.append(utxo)
            time.sleep(0.3)

    # for utxo in utxos:
    #     try:
    #         response = get_input(utxo)
    #     except RateLimitException:
    #         time.sleep(0.3)
    #         response = get_input(utxo)
    #     inputs.append(response['tx_input'])
    #     txes[response['txhash']] = response['tx']
    # return inputs, txes

def get_input2(utxo, request_url):
    utxo_hash = bytes.fromhex(utxo['txid'])
    utxo_index = int(utxo['output_n'])
    txhash = utxo_hash.hex()
    inputs = [trezorlib.messages.TxInputType(
        prev_hash=b'',
        prev_index=0,
        script_sig=b'',
        sequence=0,
                # prev_hash=bytes.fromhex(vin["txid"]),
                # prev_index=vin["vout"],
                # script_sig=bytes.fromhex(vin["scriptSig"]["hex"]),
                # sequence=0,vin["sequence"],
            )]
    bin_outputs = [trezorlib.messages.TxOutputBinType(
        amount=0,
        script_pubkey=b'',
            # amount=int(Decimal(vout["value"]) * (10**8)),
            # script_pubkey=bytes.fromhex(vout["scriptPubKey"]["hex"]),
        )]
    tx = trezorlib.messages.TransactionType(
        # version=json_dict["version"],
        version=1,
        lock_time=0,
        # lock_time=json_dict.get("locktime", 0),
        inputs=inputs, #[make_input(vin) for vin in json_dict["vin"]],
        bin_outputs=bin_outputs, #[make_bin_output(vout) for vout in json_dict["vout"]],
    )

    # from_address = tx_dict["vout"][utxo_index]["scriptPubKey"]["address"]
    # amount = tx.bin_outputs[utxo_index].amount
    amount = utxo['value']
    # echo(f"From address: {from_address} txid:{tx_dict['txid']} amount:{amount}")
    address_n = trezorlib.tools.parse_path("m/44'/0'/0'/0/0")

    # reported_type = tx_dict["vout"][utxo_index]["scriptPubKey"].get("type") # TODO
    # script_type = BITCOIN_CORE_INPUT_TYPES[reported_type]
    script_type = trezorlib.messages.InputScriptType.SPENDADDRESS # TODO
    sequence = 0xFFFFFFFD

    new_input = trezorlib.messages.TxInputType(
        address_n=address_n,
        prev_hash=utxo_hash,
        prev_index=utxo_index,
        amount=amount,
        script_type=script_type,
        sequence=sequence,
    )

    return {'tx_input': new_input, 'tx': tx, 'txhash': txhash}
'''
