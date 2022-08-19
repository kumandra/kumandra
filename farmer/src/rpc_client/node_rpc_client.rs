use crate::rpc_client::{Error as RpcError, RpcClient};
use async_trait::async_trait;
use futures::{Stream, StreamExt};
use jsonrpsee::core::client::{ClientT, SubscriptionClientT};
use jsonrpsee::core::Error as JsonError;
use jsonrpsee::rpc_params;
use jsonrpsee::ws_client::{WsClient, WsClientBuilder};
use std::pin::Pin;
use std::sync::Arc;
use kumandra_archiving::archiver::ArchivedSegment;
use kumandra_core_primitives::{RecordsRoot, SegmentIndex};
use kumandra_rpc_primitives::{
    FarmerProtocolInfo, RewardSignatureResponse, RewardSigningInfo, SlotInfo, SolutionResponse,
};

// Defines max_concurrent_requests constant in the node rpc client.
// It must be set for large plots.
const WS_PRC_MAX_CONCURRENT_REQUESTS: usize = 1_000_000;

/// `WsClient` wrapper.
#[derive(Clone, Debug)]
pub struct NodeRpcClient {
    client: Arc<WsClient>,
}

impl NodeRpcClient {
    /// Create a new instance of [`RpcClient`].
    pub async fn new(url: &str) -> Result<Self, JsonError> {
        let client = Arc::new(
            WsClientBuilder::default()
                .max_concurrent_requests(WS_PRC_MAX_CONCURRENT_REQUESTS)
                .build(url)
                .await?,
        );
        Ok(Self { client })
    }
}

#[async_trait]
impl RpcClient for NodeRpcClient {
    async fn farmer_protocol_info(&self) -> Result<FarmerProtocolInfo, RpcError> {
        Ok(self
            .client
            .request("kumandra_getFarmerProtocolInfo", rpc_params![])
            .await?)
    }

    async fn subscribe_slot_info(
        &self,
    ) -> Result<Pin<Box<dyn Stream<Item = SlotInfo> + Send + 'static>>, RpcError> {
        let subscription = self
            .client
            .subscribe(
                "kumandra_subscribeSlotInfo",
                rpc_params![],
                "kumandra_unsubscribeSlotInfo",
            )
            .await?;

        Ok(Box::pin(subscription.filter_map(
            |slot_info_result| async move { slot_info_result.ok() },
        )))
    }

    async fn submit_solution_response(
        &self,
        solution_response: SolutionResponse,
    ) -> Result<(), RpcError> {
        Ok(self
            .client
            .request(
                "kumandra_submitSolutionResponse",
                rpc_params![&solution_response],
            )
            .await?)
    }

    async fn subscribe_reward_signing(
        &self,
    ) -> Result<Pin<Box<dyn Stream<Item = RewardSigningInfo> + Send + 'static>>, RpcError> {
        let subscription = self
            .client
            .subscribe(
                "kumandra_subscribeRewardSigning",
                rpc_params![],
                "kumandra_unsubscribeRewardSigning",
            )
            .await?;

        Ok(Box::pin(subscription.filter_map(
            |reward_signing_info_result| async move { reward_signing_info_result.ok() },
        )))
    }

    /// Submit a block signature
    async fn submit_reward_signature(
        &self,
        reward_signature: RewardSignatureResponse,
    ) -> Result<(), RpcError> {
        Ok(self
            .client
            .request(
                "kumandra_submitRewardSignature",
                rpc_params![&reward_signature],
            )
            .await?)
    }

    async fn subscribe_archived_segments(
        &self,
    ) -> Result<Pin<Box<dyn Stream<Item = ArchivedSegment> + Send + 'static>>, RpcError> {
        let subscription = self
            .client
            .subscribe(
                "kumandra_subscribeArchivedSegment",
                rpc_params![],
                "kumandra_unsubscribeArchivedSegment",
            )
            .await?;

        Ok(Box::pin(subscription.filter_map(
            |archived_segment_result| async move { archived_segment_result.ok() },
        )))
    }

    async fn acknowledge_archived_segment(
        &self,
        segment_index: SegmentIndex,
    ) -> Result<(), RpcError> {
        Ok(self
            .client
            .request(
                "kumandra_acknowledgeArchivedSegment",
                rpc_params![&segment_index],
            )
            .await?)
    }

    async fn records_roots(
        &self,
        segment_indexes: Vec<SegmentIndex>,
    ) -> Result<Vec<Option<RecordsRoot>>, RpcError> {
        Ok(self
            .client
            .request("kumandra_recordsRoots", rpc_params![&segment_indexes])
            .await?)
    }
}
