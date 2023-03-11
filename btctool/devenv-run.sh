#!/usr/bin/env bash

set -ex

./btctool-cli \
  consolidate \
  --input-address=$INPUT_ADDRESS \
  --output-address=$OUTPUT_ADDRESS \
  --utxo-id-fetch-limit=10 \
  --utxo-id-max-count=1 \
  --utxo-id-max-value=100000 \
  --input-path="m/49'/0'/0'/0/22" \
  --est-fee-sats-per-vbyte=5 \
  --trezor-tx-file=/tmp/trezor_tx.json $@
