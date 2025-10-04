use cqrs_es::{EventEnvelope, View};
use rs_fsrs::Card;
use serde::{Deserialize, Serialize};

use crate::learning_session::{aggregate::LearningSession, event::LearningSessionEvent};

/// A persistent view (read model) that stores the FSRS learning state
/// for a single flashcard. This is the system's long-term memory
/// for spaced repetition.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReviewableCard {
    /// The ID of the flashcard this review state belongs to.
    /// This is the primary key for this view.
    pub flashcard_id: String,

    /// The FSRS Card object, which contains all scheduling information
    /// like `due` date, `stability`, `difficulty`, and review history.
    pub fsrs_card: Card,
}

impl View<LearningSession> for ReviewableCard {
    fn update(&mut self, event: &EventEnvelope<LearningSession>) {
        // This view is updated manually by a projection that processes
        // LearningSessionEvent::CardAnswered events.
        // We only implement this to satisfy the ViewRepository trait bounds.
        // The actual update logic happens in the projection.
        if let LearningSessionEvent::CardAnswered {
            card_id,
            updated_card,
            ..
        } = &event.payload
        {
            self.flashcard_id = card_id.clone();
            self.fsrs_card = updated_card.clone();
        }
    }
}
