use rs_fsrs::{Card, Rating};

use crate::learning_session::value_objects::language::Language;

pub enum LearningSessionCommand {
    // --- Session Lifecycle Commands ---
    /// Creates a new learning session for a user with a specific deck.
    /// The list of card IDs to review is provided from outside the domain.
    StartSession {
        session_id: String,
        deck_id: String,
        cards_to_review: Vec<String>,
        question_languages: Vec<Language>,
        answer_languages: Vec<Language>,
    },

    /// Abandons a session before it is completed.
    AbandonSession,

    // --- Card Interaction Commands ---
    /// The user provides an answer for the current card.
    /// This is the primary interaction during a session.
    AnswerCard {
        rating: Rating,
        card_before_review: Card,
    },
}
