use futures::StreamExt;
use parity_scale_codec::Encode;
use kc_consensus::{ArchivedSegmentNotification, KumandraLink};
use sp_core::traits::SpawnEssentialNamed;
use sp_runtime::traits::Block as BlockT;
use kumandra_networking::{CreationError, PUB_SUB_ARCHIVING_TOPIC};
use tracing::{error, info, trace};

/// Start an archiver that will listen for archived segments and send it to DSN network using
/// pub-sub protocol.
pub async fn start_dsn_node<Block, Spawner>(
    kumandra_link: &KumandraLink<Block>,
    networking_config: kumandra_networking::Config,
    spawner: Spawner,
) -> Result<(), CreationError>
where
    Block: BlockT,
    Spawner: SpawnEssentialNamed,
{
    trace!(target: "dsn", "Kumandra networking starting.");

    let (node, mut node_runner) = kumandra_networking::create(networking_config).await?;

    info!(target: "dsn", "Kumandra networking initialized: Node ID is {}", node.id());

    spawner.spawn_essential(
        "node-runner",
        Some("kumandra-networking"),
        Box::pin(async move {
            node_runner.run().await;
        }),
    );

    let mut archived_segment_notification_stream = kumandra_link
        .archived_segment_notification_stream()
        .subscribe();

    spawner.spawn_essential(
        "archiver",
        Some("kumandra-networking"),
        Box::pin(async move {
            trace!(target: "dsn", "Kumandra DSN archiver started.");

            while let Some(ArchivedSegmentNotification {
                archived_segment, ..
            }) = archived_segment_notification_stream.next().await
            {
                trace!(target: "dsn", "ArchivedSegmentNotification received");
                let data = archived_segment.encode().to_vec();

                match node.publish(PUB_SUB_ARCHIVING_TOPIC.clone(), data).await {
                    Ok(_) => {
                        trace!(target: "dsn", "Archived segment published.");
                    }
                    Err(err) => {
                        error!(target: "dsn", error = ?err, "Failed to publish archived segment");
                    }
                }
            }
        }),
    );

    Ok(())
}
