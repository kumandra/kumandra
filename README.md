[![Substrate version](https://img.shields.io/badge/Substrate-3.0.0-blue?logo=Parity%20Substrate)](https://substrate.dev/) [![GitHub license](https://img.shields.io/badge/license-GPL3%2FApache2-blue)](#LICENSE)


<a href='https://web3.foundation/'><img width='205' alt='web3f_grants_badge.png' src='https://github.com/heyworld88/gitskills/blob/main/web3f_grants_badge.png'></a>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<a href='https://builders.parity.io/'><img width='240' src='https://github.com/heyworld88/gitskills/blob/main/sbp_grants_badge.png'></a>

  
## Getting Started


### Install Guide

Follow [Setup](https://github.com/kumandra/kumandra-node/tree/main/docs/setup.md) to guide you install the Kumandra development.

### Build Node

The `cargo run` command will perform an initial build. Use the following command to build the node without launching it:

```
# Fetch the code
git clone https://github.com/kumandra/kumandra-node.git
cd kumandra-node

# Build the node (The first build will be long (~30min))
cargo build --release
```

## Run The Kumandra Node


After the node has finished compiling, you can follow these steps below to run it. 

### Generate Keys

If you already have keys for Substrate using the [SS58 address encoding format](https://docs.substrate.io/v3/advanced/ss58/), please see the next section.

Begin by compiling and installing the utility ([instructions and more info here](https://substrate.dev/docs/en/knowledgebase/integrate/subkey)). 

Generate a mnemonic (Secret phrase) and see the `sr25519` key and address associated with it.

```
# subkey command
subkey generate --scheme sr25519
```

Now see the `ed25519` key and address associated with the same mnemonic (secret phrase).

```
# subkey command
subkey inspect --scheme ed25519 "SECRET PHRASE YOU JUST GENERATED"
```

We recommend that you record the above outputs and keep mnemonic in safe.

### Run Testnet

Launch node on the kumandra-testnet with:

```
# start
./target/release/kumandra-node --base-path /tmp/kumandra --chain kumandra-testnet
```

Then you can add an account with:

```
# create key file
vim secretKey.txt

# add secret phrase for the node in the file
YOUR ACCOUNT'S SECRET PHRASE
```

```
# add key to node
./target/release/kumandra-node key insert --base-path /tmp/kumandra --chain kumandra-testnet --scheme Sr25519  --key-type babe --suri /root/secretKey.txt

./target/release/kumandra-node key insert --base-path /tmp/kumandra --chain kumandra-testnet --scheme Ed25519  --key-type gran --suri /root/secretKey.txt
```

Now you can launch node again:

```
# start
./target/release/kumandra-node --base-path /tmp/kumandra --chain kumandra-testnet
```

### Run in Docker

Install [Docker](https://docs.docker.com/get-docker/) first, and run the following command to start a node on the kumandra-testnet:

```
docker pull kumandra/kumandra-testnet:0.2.0
docker run -itd --name=kumandra --network=host kumandra/kumandra_testnet:0.2.0 && docker logs -f kumandra
```

## Storage Mining

Kumandra supports to obtain incentives by contributing idle storage with [storage mining tool](https://github.com/kumandra/kumandra-mining-tool), and click [here](https://github.com/kumandra/kumandra-node/tree/main/docs/designs-of-storage-mining.md) to learn more.

## Run Tests


Kumandra has Rust unit tests, and can be run locally.

```
# Run all the Rust unit tests
cargo test --release
```

## Run Tests with Benchmarks


Kumandra has Rust unit tests with benckmarks also. Currently, testing this feature in docker is not supported. Please execute belows after clone this repo.

```
# Run unit tests with benchmarks
cargo test -p pallet-sminer --features runtime-benchmarks
cargo test -p pallet-segment-book --features runtime-benchmarks
cargo test -p pallet-file-bank --features runtime-benchmarks
```

## Module Documentation


* [Files Bank](https://github.com/kumandra/kumandra-node/tree/main/pallets/file-bank)
* [Segment Book](https://github.com/kumandra/kumandra-node/tree/main/pallets/segment-book)
* [Sminer](https://github.com/kumandra/kumandra-node/tree/main/pallets/sminer)

## Contribute

Please follow the contributions guidelines as outlined in [`docs/CONTRIBUTING.adoc`](https://github.com/kumandra/kumandra-node/tree/main/docs/CONTRIBUTING.adoc). In all communications and contributions, this project follows the [Contributor Covenant Code of Conduct](https://github.com/paritytech/substrate/blob/master/docs/CODE_OF_CONDUCT.md).


### License

Kumandra is implement from [CESSProject](https://github.com/CESSProject/cess) under license [Apache 2.0 License](LICENSE-GPL3).
