import { AnchorMode, ClarityVersion, PostConditionMode, broadcastTransaction, makeContractDeploy} from '@stacks/transactions';
import { StacksTestnet } from '@stacks/network';
import { readFileSync } from 'fs';

const PRIVATE_KEY = '0e871d2b16423e98ababa1495d740613d3a4f039d4386577653a559d786af3a401';
const TESTNET_2_1_URL = "https://2-1-api.testnet.hiro.so"
const NETWORK = new StacksTestnet({url: TESTNET_2_1_URL});

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

let result = await broadcastTransaction(transaction, NETWORK);
console.log("Result: ", result);
