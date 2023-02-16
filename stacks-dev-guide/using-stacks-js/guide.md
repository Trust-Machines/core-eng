# How to upload and run a smart contract with [stacks.js](https://github.com/hirosystems/stacks.js)
The goal of this tutorial is to showcase how to use `stacks.js` to upload a smart contract to the stacks 2.1 testnet.

The complete API:s of all `stacks.js` packages are documented [here](https://stacks.js.org/).

## Prerequisites
To follow this tutorial, you need [Node.js](https://nodejs.org/en/) installed.
We also need to install the Stacks.js transactions and networks packages
in our working directory:

```sh
npm install @stacks/transactions
npm install @stacks/network
```

## Step 1: Prepare your smart contract
This tutorial will assume the following contract lives in a file `hello-world.clar` in our working directory.
Replace file with whichever contract you wish to deploy.
```clarity
(define-public (greet (name (string-ascii 20)))
  (concat (concat "Hello, " name) "!")
)
```
If you don't have a contract prepared, feel free to copy the above one or develop your own. To get familiar with smart
contract development, I recommend reading the [clarity book](https://book.clarity-lang.org).

## Step 2: Prepare your private key
To deploy your smart contract, you need a private key to an stx address with enough tokens to cover the fee to upload your contract.
You can use the [blockstack-cli](../using-blockstack-cli.md) to generate a key and request tokens from the testnet faucet.
The rest of this tutorial will use this key `0e871d2b16423e98ababa1495d740613d3a4f039d4386577653a559d786af3a401` which already has
funds from the 2.1 testnet faucet. Feel free to reuse it.

## Step 3: Write the script to deploy the contract
Create a file `deploy.mjs` and begin with adding the following imports that we'll need.

```js
import { AnchorMode, ClarityVersion, PostConditionMode, broadcastTransaction, makeContractDeploy} from '@stacks/transactions';
import { StacksTestnet } from '@stacks/network';
import { readFileSync } from 'fs';
```

Continue by adding the constants below. Feel free to replace the private key and network URL depending on your needs.

```js
const PRIVATE_KEY = '0e871d2b16423e98ababa1495d740613d3a4f039d4386577653a559d786af3a401';
const TESTNET_2_1_URL = "https://2-1-api.testnet.hiro.so"
const NETWORK = new StacksTestnet({url: TESTNET_2_1_URL});
```

We are now ready to create the transaction.
```js
const txOptions = {
  contractName: 'hello-world',
  codeBody: readFileSync('./hello-world.clar').toString(),
  senderKey: PRIVATE_KEY,
  clarityVersion: ClarityVersion.Clarity1,
  network: NETWORK,
  anchorMode: AnchorMode.OnChainOnly,
  postConditionMode: PostConditionMode.Allow,
  fee: 800,
};

const transaction = await makeContractDeploy(txOptions);
```

The final step once the transaction is created is to broadcast it
to the network.

```js
let result = await broadcastTransaction(transaction, network);
console.log("Result: ", result);
```

## Step 4: Deploy the contract
We are now ready to actually deploy the contract.

```sh
node deploy.mjs  
```

Make sure to observe the result of broadcasting the transaction.
The broadcast can fail for many reasons. Common ones include

- The contract already exists. This should happen if you try to deploy `hello-world` with the key in this tutorial.
  To circumvent the issue, you may rename the contract before publishing.
- Errors in the smart contract.
- The fee is too low.
- The stx account has insufficient funds.

If the deployment is successful, you should be able to
observe the contract in the [stacks explorer](https://explorer.stacks.co/?chain=mainnet).
Make sure to select the right network in the explorer.

## Step 5: Make a contract call

Calling the contract also entails creating and broadcasting a transaction. The main difference
is that we are building the transaction with `makeContractCall` instead of `makeContractDeploy`.
Add the following script to a file named `call.mjs`

```js
import { AnchorMode, PostConditionMode, TransactionVersion, broadcastTransaction, makeContractCall, getAddressFromPrivateKey, stringAsciiCV} from '@stacks/transactions';
import { StacksTestnet } from '@stacks/network';

const PRIVATE_KEY = '0e871d2b16423e98ababa1495d740613d3a4f039d4386577653a559d786af3a401';
const STX_ADDRESS = getAddressFromPrivateKey(PRIVATE_KEY, TransactionVersion.Testnet);
const TESTNET_2_1_URL = "https://2-1-api.testnet.hiro.so"
const NETWORK = new StacksTestnet({url: TESTNET_2_1_URL});

const txOptions = {
  senderKey: PRIVATE_KEY,
  contractAddress: STX_ADDRESS,
  contractName: "hello-world",
  functionName: "greet",
  functionArgs: [stringAsciiCV("Hans Gruber")],
  network: NETWORK,
  anchorMode: AnchorMode.OnChainOnly,
  postConditionMode: PostConditionMode.Allow,
  fee: 800,
};

const transaction = await makeContractCall(txOptions);

let result = await broadcastTransaction(transaction, NETWORK);
console.log("Result: ", result);
```

and call it with
```sh
node call.mjs
```

You should be able to see the result of the function invocation in the [stacks explorer](https://explorer.stacks.co/?chain=mainnet).
If you have been quick, you might get the transaction rejected due to `NoSuchContract`. Don't worry about this, the conctract deployment
transaction may take a while to be mined. You can see pending transactions if you search for the address you deployed your contract with in
the explorer. Here's the [link for the example key address](https://explorer.stacks.co/address/STJ8F4BTN3YFG60TRTEPCG6QZXAM2A8EMDKCYDF7?chain=testnet&api=https://2-1-api.testnet.hiro.so).

# Step 6: Are we confirmed yet?
Transactions typically take a while to be confirmed on the blockchain. The easiest way to poll the status of a transaction is sending a POST request
to a stacks node directly using the [blockchain API](https://docs.hiro.so/api). Here's an example using curl:

```
curl -sL https://2-1-api.testnet.hiro.so/extended/v1/tx/$TXID |  jq .tx_status
```

# Step 7: Take a break 🧘
You have now completed this tutorial. Good job! Reward yourself with a cold shower or anything else that increases your dopamine levels.
High dopamine levels support your brain in forming memories, and will make you remember this tutorial better.
