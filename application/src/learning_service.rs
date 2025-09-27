use std::sync::Arc;

use cqrs_es::{CqrsFramework, EventStore, persist::ViewRepository};

use learning_domain::scheduled_review::{
    aggregate::{AllScheduledReviews, ScheduledReview},
    command::ScheduledReviewCommand,
};

pub struct LearningService<ES, VR1, VR2>
where
    ES: EventStore<ScheduledReview>,
    VR1: ViewRepository<ScheduledReview, ScheduledReview>,
    VR2: ViewRepository<AllScheduledReviews, ScheduledReview>,
{
    pub cqrs: CqrsFramework<ScheduledReview, ES>,
    pub scheduled_review_view_repository: Arc<VR1>,
    pub all_scheduled_reviews_view_repository: Arc<VR2>,
}

impl<ES, VR1, VR2> LearningService<ES, VR1, VR2>
where
    ES: EventStore<ScheduledReview>,
    VR1: ViewRepository<ScheduledReview, ScheduledReview>,
    VR2: ViewRepository<AllScheduledReviews, ScheduledReview>,
{
    // `new` now accepts its dependencies instead of creating them.
    pub fn new(
        cqrs: CqrsFramework<ScheduledReview, ES>,
        scheduled_review_view_repository: Arc<VR1>,
        all_scheduled_reviews_view_repository: Arc<VR2>,
    ) -> Self {
        Self {
            cqrs,
            scheduled_review_view_repository,
            all_scheduled_reviews_view_repository,
        }
    }

    pub async fn create_review_for_card(&self, card_id: &str) {
        let command = ScheduledReviewCommand::CreateScheduledReview {
            id: card_id.to_string(),
        };

        self.cqrs.execute(card_id, command).await.unwrap();
    }

    pub async fn delete_review_for_card(&self, card_id: &str) {
        // Implement deletion logic if needed
    }
}
