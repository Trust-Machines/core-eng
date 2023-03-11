# btctool

## Instructions using the standalone binary

- Download the latest btctool release (No Docker or Trezor CLI required to be installed)
  - Manually (latest): https://github.com/Trust-Machines/core-eng/releases/latest 
  - Using curl (version 0.0.1) 
    ```commandline
    curl -LO https://github.com/Trust-Machines/core-eng/releases/download/v0.0.1/btctool.bin
    ```

- Fetch UTXOs using BitGo and Trezor APIs
  ```commandline
  ./btctool.bin dl --input-address 1111111111111111111114oLvT2
  ```
- List all UTXOs (ordered by BitGo API response)
  ```commandline
  ./btctool.bin ls
  ```
- Query for specific UTXOs. Documentation for [query language](https://pandas.pydata.org/docs/reference/api/pandas.DataFrame.query.html).
  ```commandline
  ./btctool.bin ls --utxo-filter-query "1<=index<=5"
  ```
- Generate the consolidation transaction
  ```commandline
  ./btctool.bin tx \
    --output-address 1111111111111111111114oLvT2 \
    --input-path "m/49'/0'/0'/0/22" \
    --utxo-filter-query "1<=index<=5"
  ```
- Sign the transaction
  - using the Trezor
    ```commandline
    ./btctool.bin sg
    ```
  - or the Trezor CLI:
    - Instructions to install the Trezor CLI: https://trezor.io/learn/a/trezorctl-on-macos
    - Sign the transaction:
    ```commandline
    trezorctl btc sign-tx trezor_tx.json
    ```

- Verify the signed transaction using coinb.in
  - Verify the signed transaction using a tool like https://coinb.in/#verify
    ```commandline
    TODO
    ```
- Broadcast the signed transaction using coinb.in
  - Broadcast the signed transaction using a tool like https://coinb.in/#broadcast
    ```commandline
    TODO
    ```

## Instructions using local dev environment

- Install brew (Mac package manager)
  - https://brew.sh/

- Install GitHub tool and Python
  ```commandline
  brew install gh
  brew install python@3.10 
  ```
  
- Install poetry (Python package manager)
  - https://python-poetry.org/docs/
  
- Clone the repo and install the dependencies
  ```commandline
  gh repo clone Trust-Machines/core-eng -- -b btctool
  cd core-eng/btctool
  sh devenv-install.sh
  ```

- Run btctool using examples from previous section. For example: 
  ```commandline
  ./btctool-cli dl --input-address 1111111111111111111114oLvT2
  ```

## Building and Publishing the Docker image

- Login to Docker
  ```commandline 
  docker login
  ```
- Build and push the docker image
  ```commandline
   docker-deploy.sh
  ```

- Pull the docker image
    ```commandline
    docker pull igorsyl/btctool:latest
    ```
- Run the docker image
  ```commandline
        docker run -it -v /tmp:/tmp igorsyl/btctool:latest \
  ```

### Trezor Firmware

- Checkout the trezor-firmware repository patched to sign arbitrary transaction inputs
  ```commandline 
  git clone --recurse-submodules https://github.com/Trust-Machines/trezor-firmware
  git checkout igor-patch 
  gh pr checkout https://github.com/Trust-Machines/trezor-firmware/pull/1
  ```

- Build the trezor emulator - https://docs.trezor.io/trezor-firmware/core/build/emulator.html
    - Mac: `brew install scons sdl2 sdl2_image pkg-config llvm`
  ```commandline
  cd trezor-firmware
  rustup default nightly
  rustup update
  poetry install
  poetry shell
  cd core
  make build_unix
  ```
- Run the emulator
  ```commandline
  emu.py --disable-animation --erase --slip0014 
  ```

### References

- https://api.bitgo.com/docs/#tag/Overview
- https://github.com/trezor/trezor-firmware/tree/master/python/tools
- https://docs.trezor.io/trezor-suite/
- https://github.com/trezor/trezor-firmware/tree/master/python/
- https://github.com/trezor/trezor-firmware/blob/master/common/protob/messages-bitcoin.proto
- https://github.com/trezor/trezor-firmware/blob/master/python/docs/transaction-format.md
- curl "https://www.bitgo.com/api/v1/address/1111111111111111111114oLvT2/unspents?limit=1&skip=158272" | jq # found
  155_000
- curl "https://www.bitgo.com/api/v1/address/14CEjTd5ci3228J45GdnGeUKLSSeCWUQxK/unspents?limit=5000&skip=0" | jq
- curl "https://www.bitgo.com/api/v1/tx/e20185c66e904c3589f341e0303208d8806ad4bcbb0b6b79c62562626fdfa39c" | jq
- curl "https://www.bitgo.com/api/v1/tx/e20185c66e904c3589f341e0303208d8806ad4bcbb0b6b79c62562626fdfa39c" | jq
- curl -A
  trezorlib "https://btc1.trezor.io/api/tx-specific/e20185c66e904c3589f341e0303208d8806ad4bcbb0b6b79c62562626fdfa39c" |
  jq
