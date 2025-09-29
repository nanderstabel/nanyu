use std::sync::Arc;

use cqrs_es::{CqrsFramework, EventStore, persist::ViewRepository};

use learning_domain::scheduled_review::{
    aggregate::{ScheduledReview, ScheduledReviewCollection},
    command::ScheduledReviewCommand,
};

use crate::services::{Service, ServiceBuilder};

pub struct LearningService<ES, VR1, VR2>
where
    ES: EventStore<ScheduledReview>,
    VR1: ViewRepository<ScheduledReview, ScheduledReview>,
    VR2: ViewRepository<ScheduledReviewCollection, ScheduledReview>,
{
    pub cqrs: CqrsFramework<ScheduledReview, ES>,
    pub scheduled_review_view_repository: Arc<VR1>,
    pub scheduled_review_collection_view_repository: Arc<VR2>,
}

impl<ES, VR1, VR2> LearningService<ES, VR1, VR2>
where
    ES: EventStore<ScheduledReview> + 'static,
    VR1: ViewRepository<ScheduledReview, ScheduledReview> + 'static,
    VR2: ViewRepository<ScheduledReviewCollection, ScheduledReview> + 'static,
{
    // `new` now accepts its dependencies instead of creating them.
    pub fn new(
        cqrs: CqrsFramework<ScheduledReview, ES>,
        scheduled_review_view_repository: Arc<VR1>,
        scheduled_review_collection_view_repository: Arc<VR2>,
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

impl<ES, VR1, VR2> Service for LearningService<ES, VR1, VR2>
where
    ES: EventStore<ScheduledReview>,
    VR1: ViewRepository<ScheduledReview, ScheduledReview>,
    VR2: ViewRepository<ScheduledReviewCollection, ScheduledReview>,
{
    type Aggregate = ScheduledReview;
    type IndividualView = ScheduledReview;
    type CollectionView = ScheduledReviewCollection;
    type EventStore = ES;
    type IndividualRepo = VR1;
    type CollectionRepo = VR2;

    fn new(
        cqrs: CqrsFramework<Self::Aggregate, Self::EventStore>,
        individual_repo: Arc<Self::IndividualRepo>,
        collection_repo: Arc<Self::CollectionRepo>,
    ) -> Self {
        Self {
            cqrs,
            scheduled_review_view_repository: individual_repo,
            scheduled_review_collection_view_repository: collection_repo,
        }
    }
}
