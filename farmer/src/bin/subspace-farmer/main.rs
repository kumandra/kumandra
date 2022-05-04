mod commands;
mod utils;

use anyhow::Result;
use clap::{Parser, ValueHint};
use env_logger::Env;
use log::info;
use sp_core::crypto::PublicError;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::time::Duration;
use kumandra_core_primitives::PublicKey;
use kumandra_networking::libp2p::Multiaddr;

const BEST_BLOCK_NUMBER_CHECK_INTERVAL: Duration = Duration::from_secs(5);

/// Arguments for farmer
#[derive(Debug, Parser)]
struct FarmingArgs {
    /// Multiaddrs of bootstrap nodes to connect to on startup, multiple are supported
    #[clap(long)]
    bootstrap_nodes: Vec<Multiaddr>,
    /// Custom path for data storage instead of platform-specific default
    #[clap(long, value_hint = ValueHint::FilePath)]
    custom_path: Option<PathBuf>,
    /// Multiaddr to listen on for kumandra networking, for instance `/ip4/0.0.0.0/tcp/0`,
    /// multiple are supported, kumandra networking is disabled when none specified
    #[clap(long)]
    listen_on: Vec<Multiaddr>,
    /// WebSocket RPC URL of the Kumandra node to connect to
    #[clap(long, value_hint = ValueHint::Url, default_value = "ws://127.0.0.1:9944")]
    node_rpc_url: String,
    /// Host and port where built-in WebSocket RPC server should listen for incoming connections
    #[clap(long, short, default_value = "127.0.0.1:9955")]
    ws_server_listen_addr: SocketAddr,
    /// Address for farming rewards
    #[clap(long, parse(try_from_str = parse_reward_address))]
    reward_address: PublicKey,
    /// Maximum plot size in human readable format (e.g. 10G, 2T) or just bytes (e.g. 4096).
    ///
    /// Only `G` and `T` endings are supported.
    #[clap(long, parse(try_from_str = parse_human_readable_size))]
    plot_size: u64,
}

#[derive(Debug, Parser)]
#[clap(about, version)]
enum Command {
    /// Wipes plot and identity
    Wipe {
        /// Use custom path for data storage instead of platform-specific default
        #[clap(long, value_hint = ValueHint::FilePath)]
        custom_path: Option<PathBuf>,
    },
    /// Start a farmer using previously created plot
    Farm(FarmingArgs),
}

fn parse_human_readable_size(s: &str) -> Result<u64, std::num::ParseIntError> {
    const SUFFIXES: &[(&str, u64)] = &[("G", 10u64.pow(9)), ("T", 10u64.pow(12))];

    SUFFIXES
        .iter()
        .find_map(|(suf, mul)| s.strip_suffix(suf).map(|s| (s, mul)))
        .map(|(s, mul)| s.parse::<u64>().map(|num| num * mul))
        .unwrap_or_else(|| s.parse::<u64>())
}

fn parse_reward_address(s: &str) -> Result<PublicKey, PublicError> {
    s.parse::<sp_core::sr25519::Public>()
        .map(|key| PublicKey::from(key.0))
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(Env::new().default_filter_or("info"));

    match Command::parse() {
        Command::Wipe { custom_path } => {
            let path = utils::get_path(custom_path);
            commands::wipe(&path)?;
            info!("Done");
        }
        Command::Farm(args) => {
            commands::farm(args, BEST_BLOCK_NUMBER_CHECK_INTERVAL).await?;
        }
    }
    Ok(())
}