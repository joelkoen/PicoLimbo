use crate::client::SharedClient;
use crate::server::NamedPacket;
use async_trait::async_trait;
use minecraft_protocol::prelude::DecodePacket;
use std::future::Future;
use std::marker::PhantomData;
use std::sync::Arc;

#[async_trait]
pub trait Handler: Send + Sync {
    async fn handle(&self, client: SharedClient, raw_packet: NamedPacket);
}

pub struct ListenerHandler<T, F> {
    listener_fn: Arc<F>,
    _marker: PhantomData<T>,
}

impl<T, F> ListenerHandler<T, F> {
    pub fn new(listener_fn: F) -> Self {
        Self {
            listener_fn: Arc::new(listener_fn),
            _marker: PhantomData,
        }
    }
}

#[async_trait]
impl<T, F, Fut> Handler for ListenerHandler<T, F>
where
    T: DecodePacket + Send + Sync + 'static,
    F: Fn(SharedClient, T) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send + 'static,
{
    async fn handle(&self, client: SharedClient, raw_packet: NamedPacket) {
        let packet = async {
            let client = client.lock().await;
            T::decode(&raw_packet.data, client.protocol_version().version_number()).unwrap()
        }
        .await;
        (self.listener_fn)(client, packet).await;
    }
}
