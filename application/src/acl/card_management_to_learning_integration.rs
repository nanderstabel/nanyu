use async_trait::async_trait;
use card_management_domain::deck::{aggregate::Deck, event::DeckEvent};
use cqrs_es::{
    EventEnvelope, Query,
    persist::{ViewContext, ViewRepository},
};
use std::sync::Arc;

use learning_domain::{
    Card, learning_session::aggregate::LearningSession, views::reviewable_card::ReviewableCard,
};

/// This is an Anti-Corruption Layer (ACL) that translates events from the
/// CardManagement domain into actions in the Learning domain.
pub struct CardManagementToLearningIntegration {
    reviewable_card_view_repository: Arc<dyn ViewRepository<ReviewableCard, LearningSession>>,
}

impl CardManagementToLearningIntegration {
    pub fn new(
        reviewable_card_view_repository: Arc<dyn ViewRepository<ReviewableCard, LearningSession>>,
    ) -> Self {
        Self {
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
}

#[async_trait]
impl Query<Deck> for CardManagementToLearningIntegration {
    /// This method is called by the CqrsFramework whenever new Deck events are persisted.
    async fn dispatch(&self, _aggregate_id: &str, events: &[EventEnvelope<Deck>]) {
        for event in events {
            match &event.payload {
                // When a flashcard is added to a deck...
                DeckEvent::FlashcardAdded(dto) => {
                    // ...tell the learning service to create a new reviewable card for it.
                    if let Err(e) = self.create_reviewable_card(&dto.id).await {
                        eprintln!("ACL Error: Failed to create reviewable card: {}", e);
                    }
                }
                // When a flashcard is removed from a deck...
                DeckEvent::FlashcardRemoved { flashcard_id } => {
                    // ...tell the learning service to delete its review data.
                    if let Err(e) = self.delete_reviewable_card(flashcard_id).await {
                        eprintln!("ACL Error: Failed to delete reviewable card: {}", e);
                    }
                }
                // We don't need to react to other deck events.
                _ => {}
            }
        }
    }
}
