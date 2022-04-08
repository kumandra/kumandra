[![Substrate version](https://img.shields.io/badge/Substrate-3.0.0-blue?logo=Parity%20Substrate)](https://substrate.dev/) [![GitHub license](https://img.shields.io/badge/license-GPL3%2FApache2-blue)](#LICENSE)

# Kumandra 
A free and fair blockchain for all.
Kumandra is decentralize storage for Selendra network as parachain.


### License

Kumandra is implement from [Subspace](https://github.com/subspace/subspace) under Subspace license


#### Run Node 

This will run a kumandra-node in one terminal and a kumandra-farmer farming in a second terminal.
The node will send slot notification challenges to the farmer.
If the farmer finds a valid solution it will reply, and the node will produce a new block.

```bash
# Get source code
git clone https://github.com/kumandra/kumandra-node.git
cd kumandra-node

# Build and run Node (first terminal)
cargo run --bin kumandra-node -- --dev --tmp

# wait for the client to start before continuing...

# Run Farmer (second terminal)
cargo run --bin kumandra-farmer -- farm --plot-size 10G
```