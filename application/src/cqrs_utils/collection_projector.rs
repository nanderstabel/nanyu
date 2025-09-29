use crate::cqrs_utils::outbound_adapter::OutboundAdapter;
use async_trait::async_trait;
use cqrs_es::{
    Aggregate, EventEnvelope, Query, View,
    persist::{ViewContext, ViewRepository},
};
use std::{marker::PhantomData, sync::Arc};

/// A struct that lists all the instances of an `Aggregate`.
pub struct CollectionProjector<R, V, A>
where
    R: ViewRepository<V, A>,
    V: View<A>,
    A: Aggregate,
{
    view_id: String,
    view_repository: Arc<R>,
    adapters: Vec<Box<dyn OutboundAdapter<V, A>>>,
    _phantom: PhantomData<(V, A)>,
}

impl<R, V, A> CollectionProjector<R, V, A>
where
    R: ViewRepository<V, A>,
    V: View<A>,
    A: Aggregate,
{
    pub fn new(view_repository: Arc<R>, view_id: &str) -> Self {
        CollectionProjector {
            view_id: view_id.to_string(),
            view_repository,
            adapters: Vec::new(),
            _phantom: PhantomData,
        }
    }

    /// Attaches an adapter to be notified of updates.
    pub fn with_adapter(mut self, adapter: Box<dyn OutboundAdapter<V, A>>) -> Self {
        self.adapters.push(adapter);
        self
    }
}

#[async_trait]
impl<R, V, A> Query<A> for CollectionProjector<R, V, A>
where
    R: ViewRepository<V, A>,
    V: View<A> + Clone,
    A: Aggregate,
{
    async fn dispatch(&self, _view_id: &str, events: &[EventEnvelope<A>]) {
        let (mut view, view_context) = match self
            .view_repository
            .load_with_context(&self.view_id)
            .await
            .unwrap()
        {
            None => {
                let view_context = ViewContext::new(self.view_id.clone(), 0);
                (Default::default(), view_context)
            }
            Some((view, context)) => (view, context),
        };

        for event in events {
            view.update(event);
        }

        self.view_repository
            .update_view(view.clone(), view_context)
            .await
            .ok();

        for adapter in &self.adapters {
            // FIXME: send and forget?
            adapter.on_update(&view, &self.view_id, events).await;
        }
    }
}
