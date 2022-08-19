pub mod bench_rpc_client;
#[cfg(test)]
pub mod mock_rpc_client;
pub(crate) mod node_rpc_client;

use async_trait::async_trait;
use futures::Stream;
use std::pin::Pin;
use kumandra_archiving::archiver::ArchivedSegment;
use kumandra_core_primitives::{RecordsRoot, SegmentIndex};
use kumandra_rpc_primitives::{
    FarmerProtocolInfo, RewardSignatureResponse, RewardSigningInfo, SlotInfo, SolutionResponse,
};

/// To become error type agnostic
pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

/// Abstraction of the Remote Procedure Call Client
#[async_trait]
pub trait RpcClient: Clone + Send + Sync + 'static {
    /// Get farmer metadata
    async fn farmer_protocol_info(&self) -> Result<FarmerProtocolInfo, Error>;

    /// Subscribe to slot
    async fn subscribe_slot_info(
        &self,
    ) -> Result<Pin<Box<dyn Stream<Item = SlotInfo> + Send + 'static>>, Error>;

    /// Submit a slot solution
    async fn submit_solution_response(
        &self,
        solution_response: SolutionResponse,
    ) -> Result<(), Error>;

    /// Subscribe to block signing request
    async fn subscribe_reward_signing(
        &self,
    ) -> Result<Pin<Box<dyn Stream<Item = RewardSigningInfo> + Send + 'static>>, Error>;

    /// Submit a block signature
    async fn submit_reward_signature(
        &self,
        reward_signature: RewardSignatureResponse,
    ) -> Result<(), Error>;

    /// Subscribe to archived segments
    async fn subscribe_archived_segments(
        &self,
    ) -> Result<Pin<Box<dyn Stream<Item = ArchivedSegment> + Send + 'static>>, Error>;

    /// Acknowledge receiving of archived segments
    async fn acknowledge_archived_segment(&self, segment_index: SegmentIndex) -> Result<(), Error>;

    /// Get records roots for the segments
    async fn records_roots(
        &self,
        segment_indexes: Vec<SegmentIndex>,
    ) -> Result<Vec<Option<RecordsRoot>>, Error>;
}
