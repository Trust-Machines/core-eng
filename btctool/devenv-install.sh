#!/usr/bin/env bash

set -ex

script_path="$(dirname "$0")"
script_path="$(readlink -f "$script_path")"
cd "$script_path/btctool"

# gmp needed by python library fastecdsa which is used by bitcoinlib.
brew install gmp
export CFLAGS=-I/opt/homebrew/opt/gmp/include
export LDFLAGS=-L/opt/homebrew/opt/gmp/lib
poetry install

