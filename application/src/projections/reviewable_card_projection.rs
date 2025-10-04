use std::sync::Arc;

use async_trait::async_trait;
use cqrs_es::{
    EventEnvelope, Query, View as _,
    persist::{ViewContext, ViewRepository},
};
use learning_domain::{
    learning_session::{aggregate::LearningSession, event::LearningSessionEvent},
    views::reviewable_card::ReviewableCard,
};

/// This projection is responsible for updating the `ReviewableCard` read model.
/// It listens to events from the `LearningSession` aggregate.
pub struct ReviewableCardProjection {
    repo: Arc<dyn ViewRepository<ReviewableCard, LearningSession>>,
}

impl ReviewableCardProjection {
    pub fn new(repo: Arc<dyn ViewRepository<ReviewableCard, LearningSession>>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl Query<LearningSession> for ReviewableCardProjection {
    /// This `dispatch` method is called by the CqrsFramework whenever new
    /// `LearningSession` events are persisted.
    async fn dispatch(&self, aggregate_id: &str, events: &[EventEnvelope<LearningSession>]) {
        for event in events {
            // We only care about the `CardAnswered` event for this projection.
            if let LearningSessionEvent::CardAnswered { card_id, .. } = &event.payload {
                // 1. Load the current state of the view, or create a new default one.
                let mut view = self
                    .repo
                    .load(card_id)
                    .await
                    .unwrap_or_default()
                    .unwrap_or_default();

                // 2. Apply the event to the view. This calls the `update` method
                //    you just wrote in `reviewable_card_view.rs`.
                view.update(event);

                // 3. Save the updated view back to the repository using the correct ID.
                self.repo
                    .update_view(view, ViewContext::new(aggregate_id.to_string(), 0))
                    .await
                    .unwrap();
            }
        }
    }
}
