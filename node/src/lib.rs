// Copyright (C) 2021 Subspace Labs, Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Kumandra Node library.

mod chain_spec;
mod import_blocks_from_dsn;

pub use crate::import_blocks_from_dsn::ImportBlocksFromDsnCmd;
use clap::Parser;
use sc_cli::SubstrateCli;
use sc_executor::{NativeExecutionDispatch, RuntimeVersion};
use sc_service::ChainSpec;

/// Executor dispatch for Kumandra runtime
pub struct ExecutorDispatch;

impl NativeExecutionDispatch for ExecutorDispatch {
    /// Only enable the benchmarking host functions when we actually want to benchmark.
    #[cfg(feature = "runtime-benchmarks")]
    type ExtendHostFunctions = (
        sp_executor::fraud_proof_ext::fraud_proof::HostFunctions,
        frame_benchmarking::benchmarking::HostFunctions,
    );
    /// Otherwise we only use the default Substrate host functions.
    #[cfg(not(feature = "runtime-benchmarks"))]
    type ExtendHostFunctions = sp_executor::fraud_proof_ext::fraud_proof::HostFunctions;

    fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
        kumandra_runtime::api::dispatch(method, data)
    }

    fn native_version() -> sc_executor::NativeVersion {
        kumandra_runtime::native_version()
    }
}

/// Utilities for working with a node.
#[derive(Debug, clap::Subcommand)]
pub enum Subcommand {
    /// Key management cli utilities
    #[clap(subcommand)]
    Key(sc_cli::KeySubcommand),

    /// Build a chain specification.
    BuildSpec(sc_cli::BuildSpecCmd),

    /// Validate blocks.
    CheckBlock(sc_cli::CheckBlockCmd),

    /// Export blocks.
    ExportBlocks(sc_cli::ExportBlocksCmd),

    /// Export the state of a given block into a chain spec.
    ExportState(sc_cli::ExportStateCmd),

    /// Import blocks.
    ImportBlocks(sc_cli::ImportBlocksCmd),

    /// Import blocks from Kumandra Network DSN.
    ImportBlocksFromDsn(ImportBlocksFromDsnCmd),

    /// Remove the whole chain.
    PurgeChain(sc_cli::PurgeChainCmd),

    /// Revert the chain to a previous state.
    Revert(sc_cli::RevertCmd),

    /// The custom benchmark subcommand benchmarking runtime pallets.
    #[clap(name = "benchmark", about = "Benchmark runtime pallets.")]
    Benchmark(frame_benchmarking_cli::BenchmarkCmd),
}

/// Command used to run a Kumandra node.
#[derive(Debug, Parser)]
pub struct RunCmd {
    /// Base command to run a node.
    #[clap(flatten)]
    pub base: sc_cli::RunCmd,
}

/// Kumandra Cli.
#[derive(Debug, Parser)]
pub struct Cli {
    /// Various utility commands.
    #[clap(subcommand)]
    pub subcommand: Option<Subcommand>,

    /// Run a node.
    #[clap(flatten)]
    pub run: RunCmd,
}

impl SubstrateCli for Cli {
    fn impl_name() -> String {
        "Kumandra".into()
    }

    fn impl_version() -> String {
        env!("SUBSTRATE_CLI_IMPL_VERSION").into()
    }

    fn description() -> String {
        env!("CARGO_PKG_DESCRIPTION").into()
    }

    fn author() -> String {
        env!("CARGO_PKG_AUTHORS").into()
    }

    fn support_url() -> String {
        "https://kumandra.network/support".into()
    }

    fn copyright_start_year() -> i32 {
        2021
    }

    fn load_spec(&self, id: &str) -> Result<Box<dyn ChainSpec>, String> {
        Ok(match id {
            "testnet" => Box::new(chain_spec::kumandra_testnet_config()?),
            "dev" => Box::new(chain_spec::kumandra_development_config()?),
            "" | "local" => Box::new(chain_spec::kumandra_local_testnet_config()?),
            path => Box::new(chain_spec::KumandraChainSpec::from_json_file(
                std::path::PathBuf::from(path),
            )?),
        })
    }

    fn native_runtime_version(_: &Box<dyn ChainSpec>) -> &'static RuntimeVersion {
        &kumandra_runtime::VERSION
    }
}