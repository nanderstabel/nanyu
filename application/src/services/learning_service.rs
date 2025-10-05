use std::sync::Arc;

use card_management_domain::deck::aggregate::Deck;
use cqrs_es::{CqrsFramework, EventStore, persist::ViewRepository};

use learning_domain::{
    Rating,
    learning_session::{
        aggregate::LearningSession, command::LearningSessionCommand,
        value_objects::language::Language,
    },
    views::reviewable_card::ReviewableCard,
};

pub struct LearningService<ES>
where
    ES: EventStore<LearningSession>,
{
    cqrs: CqrsFramework<LearningSession, ES>,
    deck_repo: Arc<dyn ViewRepository<Deck, Deck>>,
    reviewable_card_repo: Arc<dyn ViewRepository<ReviewableCard, LearningSession>>,
    learning_session_repo: Arc<dyn ViewRepository<LearningSession, LearningSession>>,
}

impl<ES> LearningService<ES>
where
    ES: EventStore<LearningSession> + 'static,
{
    // `new` now accepts its dependencies instead of creating them.
    pub fn new(
        cqrs: CqrsFramework<LearningSession, ES>,
        deck_repo: Arc<dyn ViewRepository<Deck, Deck>>,
        reviewable_card_repo: Arc<dyn ViewRepository<ReviewableCard, LearningSession>>,
        learning_session_repo: Arc<dyn ViewRepository<LearningSession, LearningSession>>,
    ) -> Self {
        Self {
            cqrs,
            deck_repo,
            reviewable_card_repo,
            learning_session_repo,
        }
    }

    // This new method contains all the business logic
    pub async fn start_session_for_deck(
        &self,
        deck_id: String,
        question_language: Language,
        answer_language: Language,
    ) -> Result<String, String> {
        println!(
            "Starting session for deck_id: {}, question_language: {:?}, answer_language: {:?}",
            deck_id, question_language, answer_language
        );

        // 1. Load the deck to get the flashcard IDs
        let deck = self
            .deck_repo
            .load(&deck_id)
            .await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Deck not found".to_string())?;

        // 2. For each flashcard, load its reviewable state and collect the ID
        let mut cards_to_review = Vec::new();
        for flashcard_id in deck.flashcards.keys() {
            // We check if a reviewable card exists for this ID.
            // If it does, we add its ID to the list of cards for the session.
            if self
                .reviewable_card_repo
                .load(flashcard_id)
                .await
                .map_err(|e| e.to_string())?
                .is_some()
            {
                cards_to_review.push(flashcard_id.clone());
            }
        }

        if cards_to_review.is_empty() {
            return Err("This deck has no cards to review.".to_string());
        }

        // 3. Generate a new, unique ID for this session
        let session_id = uuid::Uuid::new_v4().to_string();

        // 4. Create the command
        let command = LearningSessionCommand::StartSession {
            session_id: session_id.clone(),
            deck_id,
            cards_to_review, // This is now correctly a Vec<String>
            question_language,
            answer_language,
        };

        // 5. Execute the command
        self.cqrs
            .execute(&session_id, command)
            .await
            .map_err(|e| e.to_string())?;

        // 6. Return the new session_id
        Ok(session_id)
    }

    // New method to handle answering a card
    pub async fn answer_current_card(
        &self,
        session_id: String,
        rating: Rating,
    ) -> Result<(), String> {
        // 1. Load the session view to find the current card ID
        let session_view = self
            .learning_session_repo
            .load(&session_id)
            .await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Learning session not found".to_string())?;

        let card_id = session_view
            .current_card_id
            .ok_or_else(|| "No current card in session to answer".to_string())?;

        // 2. Load the reviewable card view to get its FSRS state
        let reviewable_card = self
            .reviewable_card_repo
            .load(&card_id)
            .await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Reviewable card data not found".to_string())?;

        // 3. Construct the full command with all required data
        let command = LearningSessionCommand::AnswerCard {
            rating,
            card_before_review: reviewable_card.fsrs_card,
        };

        // 4. Execute the command
        self.cqrs
            .execute(&session_id, command)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}
