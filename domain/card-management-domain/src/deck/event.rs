use cqrs_es::DomainEvent;
use serde::{Deserialize, Serialize};

/// A Data Transfer Object that captures the full state of a flashcard for use in events.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct FlashcardDto {
    pub id: String,
    pub dutch: String,
    pub mandarin: String,
    pub pinyin: String,
    pub english: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, strum::Display)]
pub enum DeckEvent {
    // --- Deck Lifecycle Events ---
    DeckCreated {
        id: String,
        name: String,
    },
    DeckRenamed {
        id: String,
        new_name: String,
    },
    DeckDeleted {
        id: String,
    },

    // --- Flashcard Management Events ---
    /// A new flashcard was added to the deck.
    FlashcardAdded(FlashcardDto),

    /// A flashcard was removed from the deck.
    FlashcardRemoved {
        flashcard_id: String,
    },

    /// The content of a flashcard was updated.
    FlashcardContentUpdated(FlashcardDto),
}

impl DomainEvent for DeckEvent {
    fn event_type(&self) -> String {
        self.to_string()
    }

    fn event_version(&self) -> String {
        "1".to_string()
    }
}
