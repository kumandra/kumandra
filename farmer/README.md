<div align="center">
  <h1><code>kumandra-farmer</code></h1>
  <strong>Farmer for the <a href="https://kumandra.network/">Kumandra Network Blockchain</a></strong>
</div>

## Overview
**Notes:** The code is un-audited and not production ready, use it at your own risk.

This repo is an implementation of a Farmer for [Kumandra Network Blockchain](https://kumandra.network).

Kumandra is a proof-of-storage blockchain that resolves the farmer's dilemma, to learn more read our [white paper](https://kumandra.org/whitepaper).

## Some Notes on Plotting

### Time to Plot

Plotting time is roughly linear with respect to number of cores and clock speed of the host system. On average, it takes ~ 1 minute to create a 1GB plot or 18 hours to to create a 1TB plot, though these numbers will depend on the system used. This is largely independent of the storage media used (i.e. HDD, SATA SSD, NVME SSD) as it is largely a CPU-bound task.

### Storage Overhead

In addition, the plot, a small Binary Search Tree (BST) is also stored on disk using RocksDB. This adds roughly 1% storage overhead. So creating a 1GB plot will actually consume about 1.01 GB of storage. 

## Run with Docker
The simplest way to use kumandra-farmer is to use container image:
```bash
docker volume create kumandra-farmer
docker run --rm -it --mount source=kumandra-farmer,target=/var/kumandra kumandralabs/kumandra-farmer --help
```

`kumandra-farmer` is the volume where plot and identity will be stored, it only needs to be created once.

## Install and Run Manually
Instead of Docker you can also install kumandra-farmer natively by compiling it using cargo.

RocksDB on Linux needs LLVM/Clang:
```bash
sudo apt-get install llvm clang
```

Then install the framer using Cargo:
```
cargo install kumandra-farmer
```

## Usage
Commands here assume you installed native binary, but you can also easily adapt them to using with Docker.

Use `--help` to find out all available commands and their options:
```
kumandra-farmer --help
```

### Start the farmer
```
kumandra-farmer farm
```

This will connect to local node and will try to solve on every slot notification, while also plotting all existing and new history of the blockchain in parallel.

*NOTE: You need to have a kumandra-client node running before starting farmer, otherwise it will not be able to start*

By default, plots are written to the OS-specific users local data directory.

```
Linux
$XDG_DATA_HOME or                   /home/alice/.local/share
$HOME/.local/share 

macOS
$HOME/Library/Application Support   /Users/Alice/Library/Application Support

Windows
{FOLDERID_LocalAppData}             C:\Users\Alice\AppData\Local
```

## Architecture

The farmer typically runs two processes in parallel: plotting and farming.

### Plotting

Think of it as the following pipeline:

1. [Farmer receives new blocks from the blockchain](src/archiving.rs)
2. [Archives each of them](src/archiving.rs)
3. [Encodes each archived piece by applying the time-asymmetric SLOTH permutation as `encode(genesis_piece, farmer_public_key_hash, plot_index)`](src/plotting.rs)
4. [Each encoding is written to the disk](src/plotting.rs)
3. [A commitment, or tag, to each encoding is created as `hmac(encoding, salt)` and stored within a binary search tree (BST)](src/plotting.rs).

This process currently takes ~ 36 hours per TiB on a quad-core machine, but for 1 GiB plotting should take between a few seconds and a few minutes.

### [Farming](src/farming.rs)

1. Connect to a client and subscribe to `slot_notifications` via JSON-RPC.
2. Given a global challenge as `hash(randomness || slot_index)` and `SOLUTION_RANGE`.
3. Derive local challenge as `hash(global_challenge || farmer_public_key_hash)`.
4. Query the BST for the nearest tag to the local challenge.
5. If it within `SOLUTION_RANGE` return a `SOLUTION` else return `None`
6. All the above can and will happen in parallel to plotting process, so it is possible to participate right away