use crate::{
    nodes::{setup_justification_handler, JustificationParams},
    session_map::{AuthorityProviderImpl, FinalityNotificatorImpl, SessionMapUpdater},
    KumandraConfig,
};
use log::{debug, error};
use sc_client_api::Backend;
use sc_network::ExHashT;
use sp_consensus::SelectChain;
use sp_runtime::traits::Block;

pub async fn run_nonvalidator_node<B, H, C, BE, SC>(kumandra_config: KumandraConfig<B, H, C, SC>)
where
    B: Block,
    H: ExHashT,
    C: crate::ClientForKumandra<B, BE> + Send + Sync + 'static,
    C::Api: kumandra_primitives::KumandraSessionApi<B>,
    BE: Backend<B> + 'static,
    SC: SelectChain<B> + 'static,
{
    let KumandraConfig {
        network,
        client,
        metrics,
        session_period,
        millisecs_per_block,
        justification_rx,
        spawn_handle,
        ..
    } = kumandra_config;
    let map_updater = SessionMapUpdater::<_, _, B>::new(
        AuthorityProviderImpl::new(client.clone()),
        FinalityNotificatorImpl::new(client.clone()),
    );
    let session_authorities = map_updater.readonly_session_map();
    spawn_handle.spawn("kumandra/updater", None, async move {
        debug!(target: "kumandra-party", "SessionMapUpdater has started.");
        map_updater.run(session_period).await
    });
    let (_, handler_task) = setup_justification_handler(JustificationParams {
        justification_rx,
        network,
        client,
        metrics,
        session_period,
        millisecs_per_block,
        session_map: session_authorities,
    });

    debug!(target: "kumandra-party", "JustificationHandler has started.");
    handler_task.await;
    error!(target: "kumandra-party", "JustificationHandler finished.");
}
