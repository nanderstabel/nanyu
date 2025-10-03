use async_trait::async_trait;
use card_management_domain::flashcard::{aggregate::Flashcard, event::FlashcardEvent};
use cqrs_es::{EventEnvelope, EventStore, Query, persist::ViewRepository};
use learning_domain::scheduled_review::aggregate::ScheduledReview;

use std::sync::Arc;

use crate::services::learning_service::LearningService;

pub struct CardManagementLearningIntegration<ES>
where
    ES: EventStore<ScheduledReview>,
    ES::AC: Send + Sync,
{
    learning_service: Arc<LearningService<ES>>,
}

impl<ES> CardManagementLearningIntegration<ES>
where
    ES: EventStore<ScheduledReview>,
    ES::AC: Send + Sync,
{
    pub fn new(learning_service: Arc<LearningService<ES>>) -> Self {
        Self { learning_service }
    }
}

#[async_trait]
impl<ES> Query<Flashcard> for CardManagementLearningIntegration<ES>
where
    ES: EventStore<ScheduledReview> + 'static,
    ES::AC: Send + Sync,
{
    async fn dispatch(&self, _aggregate_id: &str, events: &[EventEnvelope<Flashcard>]) {
        for event in events {
            match &event.payload {
                FlashcardEvent::FlashcardCreated { id, .. } => {
                    self.learning_service.create_review_for_card(id).await;
                }
                FlashcardEvent::FlashcardDeleted { id } => {
                    self.learning_service.delete_review_for_card(id).await;
                }
            }
        }
    }
}
