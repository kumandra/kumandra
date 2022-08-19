//! A farming process, that is interruptable (via dropping it)
//! and possible to wait on (custom `wait` method)
#[cfg(test)]
mod tests;

use crate::commitments::Commitments;
use crate::identity::Identity;
use crate::plot::Plot;
use crate::rpc_client::RpcClient;
use crate::single_disk_farm::SingleDiskSemaphore;
use crate::single_plot_farm::SinglePlotFarmId;
use crate::utils::{CallOnDrop, JoinOnDrop};
use futures::future::{Fuse, FusedFuture};
use futures::{FutureExt, StreamExt};
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::thread;
use std::time::Instant;
use kumandra_core_primitives::{PublicKey, Salt, Solution};
use kumandra_rpc_primitives::{
    RewardSignatureResponse, RewardSigningInfo, SlotInfo, SolutionResponse,
};
use kumandra_verification::is_within_solution_range;
use thiserror::Error;
use tracing::{debug, error, info, info_span, trace, warn, Instrument};

const TAGS_SEARCH_LIMIT: usize = 10;

#[derive(Debug, Error)]
pub enum FarmingError {
    #[error("jsonrpsee error: {0}")]
    RpcError(Box<dyn std::error::Error + Send + Sync>),
    #[error("Plot read error: {0}")]
    PlotRead(std::io::Error),
}

type FarmingFut = Pin<Box<dyn Future<Output = Result<(), FarmingError>> + Send>>;

/// `Farming` structure is an abstraction of the farming process for a single replica plot farming.
///
/// Farming instance can be stopped by dropping or it is possible to wait for it to exit on its own.
///
/// At high level it receives a new challenge from the consensus and tries to find solution for it
/// in its `Commitments` database.
#[must_use = "doesn't do anything unless `.wait()` method is called"]
pub struct Farming {
    farming_fut: Fuse<FarmingFut>,
}

/// Assumes `plot`, `commitment`, `client` and `identity` are already initialized
impl Farming {
    /// Create new farming instance
    pub fn create<T: RpcClient + Sync + Send + 'static>(
        single_plot_farm_id: SinglePlotFarmId,
        plot: Plot,
        commitments: Commitments,
        client: T,
        single_disk_semaphore: SingleDiskSemaphore,
        identity: Identity,
        reward_address: PublicKey,
    ) -> Self {
        let farming_fut: FarmingFut = Box::pin(
            async move {
                subscribe_to_slot_info(
                    single_plot_farm_id,
                    &client,
                    &plot,
                    &commitments,
                    single_disk_semaphore,
                    &identity,
                    reward_address,
                )
                .await
            }
            .in_current_span(),
        );

        Farming {
            farming_fut: farming_fut.fuse(),
        }
    }

    /// Waits for the background farming to finish
    pub async fn wait(&mut self) -> Result<(), FarmingError> {
        if self.farming_fut.is_terminated() {
            return Ok(());
        }
        (&mut self.farming_fut).await
    }
}

/// Salts will change, this struct allows to keep track of them
#[derive(Default)]
struct Salts {
    current: Option<Salt>,
    next: Option<Salt>,
}

/// Subscribes to slots, and tries to find a solution for them
#[allow(clippy::too_many_arguments)]
async fn subscribe_to_slot_info<T: RpcClient>(
    single_plot_farm_id: SinglePlotFarmId,
    client: &T,
    plot: &Plot,
    commitments: &Commitments,
    single_disk_semaphore: SingleDiskSemaphore,
    identity: &Identity,
    reward_address: PublicKey,
) -> Result<(), FarmingError> {
    info!("Subscribing to slot info notifications");
    let mut slot_info_notifications = client
        .subscribe_slot_info()
        .await
        .map_err(FarmingError::RpcError)?;

    let mut reward_signing_info_notifications = client
        .subscribe_reward_signing()
        .await
        .map_err(FarmingError::RpcError)?;

    let reward_signing_fut = {
        let identity = identity.clone();
        let client = client.clone();

        async move {
            while let Some(RewardSigningInfo { hash, public_key }) =
                reward_signing_info_notifications.next().await
            {
                // Multiple plots might have solved, only sign with correct one
                if identity.public_key().to_bytes() != public_key {
                    continue;
                }

                let signature = identity.sign_reward_hash(&hash);

                match client
                    .submit_reward_signature(RewardSignatureResponse {
                        hash,
                        signature: Some(signature.to_bytes().into()),
                    })
                    .await
                {
                    Ok(_) => {
                        info!("Successfully signed reward hash 0x{}", hex::encode(hash));
                    }
                    Err(error) => {
                        warn!(
                            %error,
                            "Failed to send signature for reward hash 0x{}",
                            hex::encode(hash),
                        );
                    }
                }
            }

            Ok(())
        }
    };

    let notification_handling_fut = async move {
        let mut salts = Salts::default();
        let dropped = Arc::new(AtomicBool::new(false));
        // It is important that `join_handles` comes before `_drop_guard` or else recommitments will
        // not stop until fully done on drop
        let mut join_handles = Vec::with_capacity(2);
        let _drop_guard = CallOnDrop::new({
            let dropped = Arc::clone(&dropped);

            move || {
                dropped.store(true, Ordering::SeqCst);
            }
        });

        while let Some(slot_info) = slot_info_notifications.next().await {
            debug!(?slot_info, "New slot");

            update_commitments(
                single_plot_farm_id,
                plot,
                commitments,
                &mut salts,
                &slot_info,
                &single_disk_semaphore,
                &mut join_handles,
                &dropped,
            );

            let maybe_solution_handle = tokio::task::spawn_blocking({
                let identity = identity.clone();
                let commitments = commitments.clone();
                let plot = plot.clone();

                move || {
                    let (local_challenge, target) =
                        identity.derive_local_challenge_and_target(slot_info.global_challenge);

                    // Try to first find a block authoring solution, then if not found try to find a
                    // vote
                    let voting_tags = commitments.find_by_range(
                        target,
                        slot_info.voting_solution_range,
                        slot_info.salt,
                        TAGS_SEARCH_LIMIT,
                    );

                    let maybe_tag = if voting_tags.len() < TAGS_SEARCH_LIMIT {
                        // We found all tags within voting solution range
                        voting_tags.into_iter().next()
                    } else {
                        let (tag, piece_offset) = voting_tags
                            .into_iter()
                            .next()
                            .expect("Due to if condition vector is not empty; qed");

                        if is_within_solution_range(target, tag, slot_info.solution_range) {
                            // Found a tag within solution range for blocks
                            Some((tag, piece_offset))
                        } else {
                            // There might be something that is within solution range for blocks
                            commitments
                                .find_by_range(
                                    target,
                                    slot_info.solution_range,
                                    slot_info.salt,
                                    TAGS_SEARCH_LIMIT,
                                )
                                .into_iter()
                                .next()
                                .or(Some((tag, piece_offset)))
                        }
                    };

                    match maybe_tag {
                        Some((tag, piece_offset)) => {
                            let (encoding, piece_index) = plot
                                .read_piece_with_index(piece_offset)
                                .map_err(FarmingError::PlotRead)?;
                            let solution = Solution {
                                public_key: identity.public_key().to_bytes().into(),
                                reward_address,
                                piece_index,
                                encoding,
                                tag_signature: identity.create_tag_signature(tag),
                                local_challenge,
                                tag,
                            };
                            debug!("Solution found");
                            trace!(?solution, "Solution found");

                            Ok(Some(solution))
                        }
                        None => {
                            debug!("Solution not found");
                            Ok(None)
                        }
                    }
                }
            });

            let maybe_solution = maybe_solution_handle.await.unwrap()?;

            client
                .submit_solution_response(SolutionResponse {
                    slot_number: slot_info.slot_number,
                    maybe_solution,
                })
                .await
                .map_err(FarmingError::RpcError)?;
        }

        Ok(())
    };

    futures::future::select(
        Box::pin(reward_signing_fut),
        Box::pin(notification_handling_fut),
    )
    .await
    .factor_first()
    .0?;

    Ok(())
}

/// Compare salts in `slot_info` to those known from `salts` and start update plot commitments
/// accordingly if necessary (in background)
#[allow(clippy::too_many_arguments)]
fn update_commitments(
    single_plot_farm_id: SinglePlotFarmId,
    plot: &Plot,
    commitments: &Commitments,
    salts: &mut Salts,
    slot_info: &SlotInfo,
    single_disk_semaphore: &SingleDiskSemaphore,
    join_handles: &mut Vec<JoinOnDrop>,
    must_stop: &Arc<AtomicBool>,
) {
    let mut current_recommitment_done_receiver = None;
    // Check if current salt has changed
    if salts.current != Some(slot_info.salt) {
        salts.current.replace(slot_info.salt);

        // If previous `salts.next` is not the same as current (expected behavior), need to
        // re-commit
        if salts.next != Some(slot_info.salt) {
            let (current_recommitment_done_sender, receiver) = mpsc::channel::<()>();

            current_recommitment_done_receiver.replace(receiver);

            let salt = slot_info.salt;
            let plot = plot.clone();
            let commitments = commitments.clone();
            let single_disk_semaphore = single_disk_semaphore.clone();
            let span = info_span!("recommit", new_salt = %hex::encode(salt));
            let must_stop = Arc::clone(must_stop);

            let result = thread::Builder::new()
                .name(format!(
                    "recommit-{}-{single_plot_farm_id}",
                    hex::encode(salt)
                ))
                .spawn(move || {
                    let _single_disk_semaphore_guard = single_disk_semaphore.acquire();
                    let _span_guard = span.enter();

                    let started = Instant::now();
                    info!("Salt updated, recommitting in background");

                    if let Err(error) = commitments.create(salt, plot, &must_stop) {
                        error!(%error, "Failed to create commitment");
                    } else {
                        info!(
                            took_seconds = started.elapsed().as_secs_f32(),
                            "Finished recommitment",
                        );
                    }

                    // We don't care if anyone is listening on the other side
                    let _ = current_recommitment_done_sender.send(());
                });

            match result {
                Ok(join_handle) => {
                    join_handles.drain_filter(|join_handle| join_handle.is_finished());
                    join_handles.push(JoinOnDrop::new(join_handle));
                }
                Err(error) => {
                    error!(%error, "Failed to spawn recommitment thread")
                }
            }
        }
    }

    if let Some(new_next_salt) = slot_info.next_salt {
        if salts.next != Some(new_next_salt) {
            salts.next.replace(new_next_salt);

            let plot = plot.clone();
            let commitments = commitments.clone();
            let single_disk_semaphore = single_disk_semaphore.clone();
            let span = info_span!("recommit", next_salt = %hex::encode(new_next_salt));
            let must_stop = Arc::clone(must_stop);

            let result = thread::Builder::new()
                .name(format!(
                    "recommit-{}-{single_plot_farm_id}",
                    hex::encode(new_next_salt)
                ))
                .spawn(move || {
                    // Wait for current recommitment to finish if it is in progress
                    if let Some(receiver) = current_recommitment_done_receiver {
                        // Do not care about result here either
                        let _ = receiver.recv();
                    }

                    let _single_disk_semaphore_guard = single_disk_semaphore.acquire();
                    let _span_guard = span.enter();

                    let started = Instant::now();
                    info!("Salt will be updated, recommitting in background");
                    if let Err(error) = commitments.create(new_next_salt, plot, &must_stop) {
                        error!(
                            %error,
                            "Recommitting salt in background failed",
                        );
                        return;
                    }
                    info!(
                        took_seconds = started.elapsed().as_secs_f32(),
                        "Finished recommitment in background",
                    );
                });

            match result {
                Ok(join_handle) => {
                    join_handles.drain_filter(|join_handle| join_handle.is_finished());
                    join_handles.push(JoinOnDrop::new(join_handle));
                }
                Err(error) => {
                    error!(%error, "Failed to spawn recommitment thread")
                }
            }
        }
    }
}
