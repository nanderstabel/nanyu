use cqrs_es::DomainEvent;
use rs_fsrs::{Card, Rating};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::learning_session::value_objects::{answer_quality::AnswerQuality, language::Language};

/// A DTO that captures the result of a card review.
/// This is the primary output of the learning domain.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ReviewResult {
    pub card_id: String,
    pub quality: AnswerQuality,
    pub new_ease_factor: f32,
    pub new_interval_days: u32,
    pub next_review_date: OffsetDateTime,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, strum::Display)]
pub enum LearningSessionEvent {
    // --- Session Lifecycle Events ---
    /// A new learning session was started with an initial set of cards.
    SessionStarted {
        session_id: String,
        deck_id: String,
        cards_to_review: Vec<String>,
        question_languages: Vec<Language>,
        answer_languages: Vec<Language>,
    },

    /// A session was abandoned before completion.
    SessionAbandoned,

    /// A session was successfully completed after all cards were reviewed.
    SessionCompleted,

    // --- Card Interaction Events ---
    /// A card was presented to the user for review.
    CardPresented { card_id: String },

    /// A card was answered by the user, and the new review schedule was calculated.
    CardAnswered {
        card_id: String,
        rating: Rating,
        updated_card: Card,
    },
}

impl DomainEvent for LearningSessionEvent {
    fn event_type(&self) -> String {
        self.to_string()
    }

    fn event_version(&self) -> String {
        "1".to_string()
    }
}
