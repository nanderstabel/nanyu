use async_trait::async_trait;
use card_management_domain::deck::{aggregate::Deck, event::DeckEvent};
use cqrs_es::{EventEnvelope, Query};
use std::sync::Arc;

use crate::services::learning_service::LearningService;
use cqrs_es::EventStore;
use learning_domain::learning_session::aggregate::LearningSession;

/// This is an Anti-Corruption Layer (ACL) that translates events from the
/// CardManagement domain into actions in the Learning domain.
pub struct CardManagementToLearningIntegration<ES>
where
    ES: EventStore<LearningSession>,
{
    learning_service: Arc<LearningService<ES>>,
}

impl<ES> CardManagementToLearningIntegration<ES>
where
    ES: EventStore<LearningSession>,
{
    pub fn new(learning_service: Arc<LearningService<ES>>) -> Self {
        Self { learning_service }
    }
}

#[async_trait]
impl<ES> Query<Deck> for CardManagementToLearningIntegration<ES>
where
    ES: EventStore<LearningSession> + 'static,
{
    /// This method is called by the CqrsFramework whenever new Deck events are persisted.
    async fn dispatch(&self, _aggregate_id: &str, events: &[EventEnvelope<Deck>]) {
        for event in events {
            match &event.payload {
                // When a flashcard is added to a deck...
                DeckEvent::FlashcardAdded(dto) => {
                    // ...tell the learning service to create a new reviewable card for it.
                    if let Err(e) = self.learning_service.create_reviewable_card(&dto.id).await {
                        eprintln!("ACL Error: Failed to create reviewable card: {}", e);
                    }
                }
                // When a flashcard is removed from a deck...
                DeckEvent::FlashcardRemoved { flashcard_id } => {
                    // ...tell the learning service to delete its review data.
                    if let Err(e) = self
                        .learning_service
                        .delete_reviewable_card(flashcard_id)
                        .await
                    {
                        eprintln!("ACL Error: Failed to delete reviewable card: {}", e);
                    }
                }
                // We don't need to react to other deck events.
                _ => {}
            }
        }
    }
}
