# Kumandra Node

Reference implementation of Kumandra Farmer

## Overview
**Notes:** The code is un-audited and not production ready, use it at your own risk.

Kumandra is a proof-of-storage blockchain that trying to solve wasted computing power and storage.

## Some Notes on Plotting

### Time to Plot

Plotting time is roughly linear with respect to number of cores and clock speed of the host system. On average, it takes ~ 1 minute to create a 1GB plot or 18 hours to to create a 1TB plot, though these numbers will depend on the system used. This is largely independent of the storage media used (i.e. HDD, SATA SSD, NVME SSD) as it is largely a CPU-bound task.

### Storage Overhead

In addition to the plot a small Binary Search Tree (BST) is also stored on disk using RocksDB, which has roughly 1% of the storage size.
Due to current implementation two of such databases might be stored at once, though this will improve in the future.
There are also some supplementary database mappings.

So creating a 1GB plot should actually consume about 1.03 GB of storage.
Plot size parameter specified in farming command accounts for this overhead, so you don't need to worry about implementation details.

## Running

It is recommended to follow general farming instructions that explain how to run both farmer and node together.

## Build from source

Rust toolchain is expected to be installed for anything in this repository to compile, but there are some extra dependencies for farmer specifically.

RocksDB on Linux needs LLVM/Clang:
```bash
sudo apt-get install llvm clang
```

For OpenCL support before enabling `opencl` feature make sure to follow documentation of `sloth256-189` library: <https://docs.rs/sloth256-189/latest/sloth256_189/opencl/index.html>.

Then build the farmer using Cargo:
```
cargo build --profile production --bin kumandra-farmer
target/production/kumandra-farmer --version
```

## Usage
Commands here assume you installed native binary, but you can also easily adapt them to using with Docker.

Use `--help` to find out all available commands and their options:
```
target/production/kumandra-farmer --help
```

### Start the farmer
```
target/production/kumandra-farmer farm --reward-address st... --plot-size 100G
```

`st...` should be replaced with the reward address taken from Polkadot.js wallet (or similar) and `100G` replaced with desired plot size.

This will connect to local node and will try to solve on every slot notification, while also plotting all existing and new history of the blockchain in parallel.

*NOTE: You need to have a `kumandra-node` running before starting farmer, otherwise it will not be able to start*

By default, farmer data are written to `kumandra-farmer` subdirectory of the OS-specific users local data directory.

```
Linux
$XDG_DATA_HOME or                   /home/alice/.local/share
$HOME/.local/share 

macOS
$HOME/Library/Application Support   /Users/Alice/Library/Application Support

Windows
{FOLDERID_LocalAppData}             C:\Users\Alice\AppData\Local
```

### Wipe the plot
```
target/production/kumandra-farmer wipe
```

This would wipe plots in the OS-specific users local data directory.

## Architecture

The farmer typically runs two processes in parallel: plotting and farming.

### Plotting

Think of it as the following pipeline:

1. [Farmer receives new blocks from the blockchain](src/archiving.rs)
2. [Archives each of them](src/archiving.rs)
3. [Encodes each archived piece by applying the time-asymmetric SLOTH permutation as `encode(genesis_piece, farmer_public_key_hash, plot_index)`](src/single_plot_farm)
4. [Each encoding is written to the disk](src/single_plot_farm.rs)
3. [A commitment, or tag, to each encoding is created as `hmac(encoding, salt)` and stored within a binary search tree (BST)](src/single_plot_farm).

This process currently takes ~ 36 hours per TiB on a quad-core machine, but for 1 GiB plotting should take between a few seconds and a few minutes.

### [Farming](src/farming.rs)

1. Connect to a client and subscribe to `slot_notifications` via JSON-RPC.
2. Given a global challenge as `hash(randomness || slot_index)` and `SOLUTION_RANGE`.
3. Derive local challenge as `hash(global_challenge || farmer_public_key_hash)`.
4. Query the BST for the nearest tag to the local challenge.
5. If it within `SOLUTION_RANGE` return a `SOLUTION` else return `None`
6. All the above can and will happen in parallel to plotting process, so it is possible to participate right away
