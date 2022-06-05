Kumandra Protocol is a decentralized cloud computing and storage and other competitor that focuses on consumer and user that have unused compute resource that they want to contribute to the Kumandra community and get reward in Kumandra Token. Its native Kumandra tokens are used to pay for transaction fees, host Dapp and buy storage on the Kumandra crypto platform.

## Guide on building the Node
Please go to the [BUILD](./docs/BUILD.md)

## Running Kumandra Testnet
TODO!

## Local Network
You can play around with Kumandra Node by running a small blockchain network using the `run_node.sh` from the scripts folder. The script starts multiple instances of Kumandra Node on your local machine, so please adjust the number of nodes carefully with performance of your system in mind. By default 4 nodes are started.

You can interact with your locally running nodes using RPC (use port 9933 for node0, 9934 for node1 and so on). A more convenient alternative is to attach to it with a polkadot.js wallet app. We recommend using our fork of that app which can be found [here](https://github.com/kumandra/kumandra.app).
