use async_trait::async_trait;
use card_management_domain::flashcard::{aggregate::Flashcard, event::FlashcardEvent};
use cqrs_es::{EventEnvelope, EventStore, Query, persist::ViewRepository};
use learning_domain::scheduled_review::aggregate::{AllScheduledReviews, ScheduledReview};

use std::sync::Arc;

use crate::learning_service::LearningService;

pub struct CardManagementLearningIntegration<ES, VR1, VR2>
where
    ES: EventStore<ScheduledReview>,
    ES::AC: Send + Sync,
    VR1: ViewRepository<ScheduledReview, ScheduledReview>,
    VR2: ViewRepository<AllScheduledReviews, ScheduledReview>,
{
    learning_service: Arc<LearningService<ES, VR1, VR2>>,
}

impl<ES, VR1, VR2> CardManagementLearningIntegration<ES, VR1, VR2>
where
    ES: EventStore<ScheduledReview>,
    ES::AC: Send + Sync,
    VR1: ViewRepository<ScheduledReview, ScheduledReview>,
    VR2: ViewRepository<AllScheduledReviews, ScheduledReview>,
{
    pub fn new(learning_service: Arc<LearningService<ES, VR1, VR2>>) -> Self {
        Self { learning_service }
    }
}

#[async_trait]
impl<ES, VR1, VR2> Query<Flashcard> for CardManagementLearningIntegration<ES, VR1, VR2>
where
    ES: EventStore<ScheduledReview>,
    ES::AC: Send + Sync,
    VR1: ViewRepository<ScheduledReview, ScheduledReview>,
    VR2: ViewRepository<AllScheduledReviews, ScheduledReview>,
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
