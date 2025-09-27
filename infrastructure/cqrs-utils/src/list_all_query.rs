use std::{marker::PhantomData, sync::Arc};

use async_trait::async_trait;
use cqrs_es::{
    Aggregate, EventEnvelope, Query, View,
    persist::{ViewContext, ViewRepository},
};

/// A struct that lists all the instances of an `Aggregate`.
pub struct ListAllQuery<R, V, A>
where
    R: ViewRepository<V, A>,
    V: View<A>,
    A: Aggregate,
{
    view_id: String,
    view_repository: Arc<R>,
    _phantom: PhantomData<(V, A)>,
}

impl<R, V, A> ListAllQuery<R, V, A>
where
    R: ViewRepository<V, A>,
    V: View<A>,
    A: Aggregate,
{
    pub fn new(view_repository: Arc<R>, view_id: &str) -> Self {
        ListAllQuery {
            view_id: view_id.to_string(),
            view_repository,
            _phantom: PhantomData,
        }
    }
}

#[async_trait]
impl<R, V, A> Query<A> for ListAllQuery<R, V, A>
where
    R: ViewRepository<V, A>,
    V: View<A>,
    A: Aggregate,
{
    async fn dispatch(&self, _view_id: &str, events: &[EventEnvelope<A>]) {
        for event in events {
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

            view.update(event);
            self.view_repository
                .update_view(view, view_context)
                .await
                .ok();
        }
    }
}
