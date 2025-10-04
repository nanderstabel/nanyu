use crate::cqrs_utils::outbound_adapter::OutboundAdapter;
use async_trait::async_trait;
use cqrs_es::{
    Aggregate, EventEnvelope, Query, View,
    persist::{ViewContext, ViewRepository},
};
use std::{marker::PhantomData, sync::Arc};

/// A generic projector that can create and update either single-instance or collection views.
pub struct Projector<R, V, A>
where
    R: ViewRepository<V, A>,
    V: View<A>,
    A: Aggregate,
{
    view_repository: Arc<R>,
    adapters: Vec<Box<dyn OutboundAdapter<V, A>>>,
    /// If set, the projector manages a collection view.
    is_collection: bool,
    _phantom: PhantomData<(V, A)>,
}

impl<R, V, A> Projector<R, V, A>
where
    R: ViewRepository<V, A>,
    V: View<A>,
    A: Aggregate,
{
    /// Creates a new projector for a collection view with a fixed ID.
    pub fn for_collection(view_repository: Arc<R>) -> Self {
        Self {
            view_repository,
            adapters: Vec::new(),
            is_collection: true,
            _phantom: PhantomData,
        }
    }

    /// Creates a new projector for individual aggregate views.
    pub fn for_individual(view_repository: Arc<R>) -> Self {
        Self {
            view_repository,
            adapters: Vec::new(),
            is_collection: false,
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
impl<R, V, A> Query<A> for Projector<R, V, A>
where
    R: ViewRepository<V, A> + Send + Sync,
    V: View<A> + Default + Clone,
    A: Aggregate + Send + Sync,
{
    async fn dispatch(&self, _aggregate_id: &str, events: &[EventEnvelope<A>]) {
        if events.is_empty() {
            return;
        }

        // Use the aggregate ID as the view ID for individual views,
        // or a fixed ID for collection views.
        let view_id = self
            .is_collection
            .then(|| {
                format!(
                    "{aggregate_type}-collection",
                    aggregate_type = A::aggregate_type()
                )
            })
            .unwrap_or(format!(
                "{aggregate_type}",
                aggregate_type = A::aggregate_type()
            ));

        let (mut view, view_context) = match self
            .view_repository
            .load_with_context(&view_id)
            .await
            .unwrap()
        {
            None => (V::default(), ViewContext::new(view_id.clone(), 0)),
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
            adapter.on_update(&view, &view_id, events).await;
        }
    }
}
