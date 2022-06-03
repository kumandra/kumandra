use crate::{
    data_io::DataStore,
    network::{KumandraNetworkData, ReceiverComponent, RequestBlocks},
    party::{AuthoritySubtaskCommon, Task},
};
use kumandra_bft::SpawnHandle;
use futures::channel::oneshot;
use log::debug;
use sc_client_api::{BlockchainEvents, HeaderBackend};
use sp_runtime::traits::Block;

/// Runs the data store within a single session.
pub fn task<B, C, RB, R>(
    subtask_common: AuthoritySubtaskCommon,
    mut data_store: DataStore<B, C, RB, KumandraNetworkData<B>, R>,
) -> Task
where
    B: Block,
    C: HeaderBackend<B> + BlockchainEvents<B> + Send + Sync + 'static,
    RB: RequestBlocks<B> + 'static,
    R: ReceiverComponent<KumandraNetworkData<B>> + 'static,
{
    let AuthoritySubtaskCommon {
        spawn_handle,
        session_id,
    } = subtask_common;
    let (stop, exit) = oneshot::channel();
    let task = {
        async move {
            debug!(target: "kumandra-party", "Running the data store task for {:?}", session_id);
            data_store.run(exit).await;
            debug!(target: "kumandra-party", "Data store task stopped for {:?}", session_id);
        }
    };

    let handle = spawn_handle.spawn_essential("kumandra/consensus_session_data_store", task);
    Task::new(handle, stop)
}
