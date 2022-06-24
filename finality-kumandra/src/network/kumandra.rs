use crate::{
    crypto::Signature,
    data_io::{KumandraData, KumandraNetworkMessage},
    network::{Data, DataNetwork},
    Hasher,
};
use kumandra_bft::{Network as KumandraNetwork, NetworkData as KumandraNetworkData, SignatureSet};
use log::warn;
use sp_runtime::traits::Block;
use std::marker::PhantomData;

pub type NetworkData<B> =
    KumandraNetworkData<Hasher, KumandraData<B>, Signature, SignatureSet<Signature>>;

impl<B: Block> KumandraNetworkMessage<B> for NetworkData<B> {
    fn included_data(&self) -> Vec<KumandraData<B>> {
        self.included_data()
    }
}

/// A wrapper needed only because of type system theoretical constraints. Sadness.
pub struct NetworkWrapper<D: Data, DN: DataNetwork<D>> {
    inner: DN,
    _phantom: PhantomData<D>,
}

impl<D: Data, DN: DataNetwork<D>> From<DN> for NetworkWrapper<D, DN> {
    fn from(inner: DN) -> Self {
        NetworkWrapper {
            inner,
            _phantom: PhantomData,
        }
    }
}

#[async_trait::async_trait]
impl<D: Data, DN: DataNetwork<D>> KumandraNetwork<D> for NetworkWrapper<D, DN> {
    fn send(&self, data: D, recipient: kumandra_bft::Recipient) {
        if self.inner.send(data, recipient).is_err() {
            warn!(target: "kumandra-network", "Error sending an AlephBFT message to the network.");
        }
    }

    async fn next_event(&mut self) -> Option<D> {
        self.inner.next().await
    }
}
