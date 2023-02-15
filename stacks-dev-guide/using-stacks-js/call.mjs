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
