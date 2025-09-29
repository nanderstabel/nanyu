use async_trait::async_trait;
use cqrs_es::{Aggregate, EventEnvelope, View};

#[async_trait]
pub trait OutboundAdapter<V, A>: Send + Sync
where
    V: View<A>,
    A: Aggregate,
{
    async fn on_update(&self, view: &V, view_id: &str, events: &[EventEnvelope<A>]);
}
