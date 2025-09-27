use cqrs_es::DomainEvent;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, strum::Display)]
pub enum FlashcardEvent {
    FlashcardCreated {
        id: String,
        question: String,
        answer: String,
    },
    FlashcardDeleted {
        id: String,
    },
}

impl DomainEvent for FlashcardEvent {
    fn event_type(&self) -> String {
        self.to_string()
    }

    fn event_version(&self) -> String {
        "1".to_string()
    }
}
