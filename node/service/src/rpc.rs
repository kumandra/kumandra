// Copyright (C) 2022 KOOMPI Inc.
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

//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.

#![warn(missing_docs)]

use jsonrpsee::RpcModule;
use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApiServer};
use sc_client_api::BlockBackend;
use kc_consensus::notification::KumandraNotificationStream;
use kc_consensus::{
    ArchivedSegmentNotification, NewSlotNotification, RewardSigningNotification,
};
use kc_consensus_rpc::{KumandraRpc, KumandraRpcApiServer};
use sc_rpc::SubscriptionTaskExecutor;
use sc_rpc_api::DenyUnsafe;
use sc_transaction_pool_api::TransactionPool;
use sp_api::ProvideRuntimeApi;
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};
use kp_consensus::FarmerPublicKey;
use std::sync::Arc;
use kumandra_runtime_primitives::opaque::Block;
use kumandra_runtime_primitives::{AccountId, Balance, Index};
use substrate_frame_rpc_system::{System, SystemApiServer};

/// Full client dependencies.
pub struct FullDeps<C, P> {
    /// The client instance to use.
    pub client: Arc<C>,
    /// Transaction pool instance.
    pub pool: Arc<P>,
    /// Whether to deny unsafe calls.
    pub deny_unsafe: DenyUnsafe,
    /// Executor to drive the subscription manager in the Grandpa RPC handler.
    pub subscription_executor: SubscriptionTaskExecutor,
    /// A stream with notifications about new slot arrival with ability to send solution back.
    pub new_slot_notification_stream: KumandraNotificationStream<NewSlotNotification>,
    /// A stream with notifications about headers that need to be signed with ability to send
    /// signature back.
    pub reward_signing_notification_stream: KumandraNotificationStream<RewardSigningNotification>,
    /// A stream with notifications about archived segment creation.
    pub archived_segment_notification_stream:
        KumandraNotificationStream<ArchivedSegmentNotification>,
}

/// Instantiate all full RPC extensions.
pub fn create_full<C, P>(
    deps: FullDeps<C, P>,
) -> Result<RpcModule<()>, Box<dyn std::error::Error + Send + Sync>>
where
    C: ProvideRuntimeApi<Block>
        + BlockBackend<Block>
        + HeaderBackend<Block>
        + HeaderMetadata<Block, Error = BlockChainError>
        + Send
        + Sync
        + 'static,
    C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Index>
        + pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>
        + BlockBuilder<Block>
        + kp_consensus::KumandraApi<Block, FarmerPublicKey>,
    P: TransactionPool + 'static,
{
    let mut module = RpcModule::new(());
    let FullDeps {
        client,
        pool,
        deny_unsafe,
        subscription_executor,
        new_slot_notification_stream,
        reward_signing_notification_stream,
        archived_segment_notification_stream,
    } = deps;

    module.merge(System::new(client.clone(), pool, deny_unsafe).into_rpc())?;
    module.merge(TransactionPayment::new(client.clone()).into_rpc())?;

    module.merge(
        KumandraRpc::new(
            client,
            subscription_executor,
            new_slot_notification_stream,
            reward_signing_notification_stream,
            archived_segment_notification_stream,
        )
        .into_rpc(),
    )?;

    Ok(module)
}
