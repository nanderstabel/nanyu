use std::sync::Arc;

use cqrs_es::{CqrsFramework, EventStore, persist::ViewRepository};

use learning_domain::scheduled_review::{
    aggregate::ScheduledReview, command::ScheduledReviewCommand,
};

use crate::{
    cqrs_utils::collection::Collection,
    services::{Service, ServiceBuilder},
};

pub struct LearningService<ES>
where
    ES: EventStore<ScheduledReview>,
{
    pub cqrs: CqrsFramework<ScheduledReview, ES>,
    pub scheduled_review_view_repository: Arc<dyn ViewRepository<ScheduledReview, ScheduledReview>>,
    pub scheduled_review_collection_view_repository:
        Arc<dyn ViewRepository<Collection<ScheduledReview>, ScheduledReview>>,
}

impl<ES> LearningService<ES>
where
    ES: EventStore<ScheduledReview> + 'static,
{
    // `new` now accepts its dependencies instead of creating them.
    pub fn new(
        cqrs: CqrsFramework<ScheduledReview, ES>,
        scheduled_review_view_repository: Arc<dyn ViewRepository<ScheduledReview, ScheduledReview>>,
        scheduled_review_collection_view_repository: Arc<
            dyn ViewRepository<Collection<ScheduledReview>, ScheduledReview>,
        >,
    ) -> Self {
        Self {
            cqrs,
            scheduled_review_view_repository,
            scheduled_review_collection_view_repository,
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

    pub fn builder() -> ServiceBuilder<Self> {
        ServiceBuilder::new()
    }
}

impl<ES> Service for LearningService<ES>
where
    ES: EventStore<ScheduledReview>,
{
    type Aggregate = ScheduledReview;
    type View = ScheduledReview;
    type EventStore = ES;

    fn new(
        cqrs: CqrsFramework<Self::Aggregate, Self::EventStore>,
        individual_repo: Arc<dyn ViewRepository<Self::View, Self::Aggregate>>,
        collection_repo: Arc<dyn ViewRepository<Collection<Self::Aggregate>, Self::Aggregate>>,
    ) -> Self {
        Self {
            cqrs,
            scheduled_review_view_repository: individual_repo,
            scheduled_review_collection_view_repository: collection_repo,
        }
    }
}
