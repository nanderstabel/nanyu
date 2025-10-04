use std::sync::Arc;

use cqrs_es::{
    CqrsFramework, EventStore,
    persist::{ViewContext, ViewRepository},
};

use learning_domain::{
    Card, learning_session::aggregate::LearningSession, views::reviewable_card::ReviewableCard,
};

use crate::{
    cqrs_utils::collection::Collection,
    services::{Service, ServiceBuilder},
};

pub struct LearningService<ES>
where
    ES: EventStore<LearningSession>,
{
    pub cqrs: CqrsFramework<LearningSession, ES>,
    pub learning_session_view_repository: Arc<dyn ViewRepository<LearningSession, LearningSession>>,
    pub learning_session_collection_view_repository:
        Arc<dyn ViewRepository<Collection<LearningSession>, LearningSession>>,

    pub reviewable_card_view_repository: Arc<dyn ViewRepository<ReviewableCard, LearningSession>>,
}

impl<ES> LearningService<ES>
where
    ES: EventStore<LearningSession> + 'static,
{
    // `new` now accepts its dependencies instead of creating them.
    pub fn new(
        cqrs: CqrsFramework<LearningSession, ES>,
        learning_session_view_repository: Arc<dyn ViewRepository<LearningSession, LearningSession>>,
        learning_session_collection_view_repository: Arc<
            dyn ViewRepository<Collection<LearningSession>, LearningSession>,
        >,
        reviewable_card_view_repository: Arc<dyn ViewRepository<ReviewableCard, LearningSession>>,
    ) -> Self {
        Self {
            cqrs,
            learning_session_view_repository,
            learning_session_collection_view_repository,
            reviewable_card_view_repository,
        }
    }

    /// Creates the initial learning state for a new flashcard.
    /// This is called by the ACL.
    pub async fn create_reviewable_card(
        &self,
        flashcard_id: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if self
            .reviewable_card_view_repository
            .load(flashcard_id)
            .await?
            .is_some()
        {
            return Ok(()); // Already exists, do nothing.
        }

        let reviewable_card = ReviewableCard {
            flashcard_id: flashcard_id.to_string(),
            fsrs_card: Card::new(), // A new card with no review history.
        };

        self.reviewable_card_view_repository
            .update_view(
                reviewable_card,
                ViewContext::new(flashcard_id.to_string(), 0),
            )
            .await?;

        Ok(())
    }

    /// Removes the learning state when a flashcard is deleted.
    /// This is called by the ACL.
    pub async fn delete_reviewable_card(
        &self,
        flashcard_id: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        todo!();
        Ok(())
    }

    pub fn builder() -> ServiceBuilder<Self> {
        ServiceBuilder::new()
    }
}

impl<ES> Service for LearningService<ES>
where
    ES: EventStore<LearningSession>,
{
    type Aggregate = LearningSession;
    type View = LearningSession;
    type EventStore = ES;

    fn new(
        cqrs: CqrsFramework<Self::Aggregate, Self::EventStore>,
        individual_repo: Arc<dyn ViewRepository<Self::View, Self::Aggregate>>,
        collection_repo: Arc<dyn ViewRepository<Collection<Self::Aggregate>, Self::Aggregate>>,
    ) -> Self {
        Self {
            cqrs,
            learning_session_view_repository: individual_repo,
            learning_session_collection_view_repository: collection_repo,
            reviewable_card_view_repository: todo!(),
        }
    }
}
