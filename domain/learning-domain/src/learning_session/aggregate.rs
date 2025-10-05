use std::collections::VecDeque;

use async_trait::async_trait;
use chrono::Utc;
use cqrs_es::{Aggregate, EventEnvelope, View};
use rs_fsrs::{FSRS, Parameters};
use serde::{Deserialize, Serialize};

use super::{
    command::LearningSessionCommand::{self, *},
    error::LearningSessionError::{self, *},
    event::LearningSessionEvent::{self, *},
    value_objects::{language::Language, session_status::SessionStatus},
};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct LearningSession {
    pub id: String,
    pub deck_id: String,

    // Defines the "front" and "back" of the cards for this session.
    pub question_language: Option<Language>,
    pub answer_language: Option<Language>,

    // The queue of card IDs to be reviewed.
    pub cards_to_review: VecDeque<String>,

    pub current_card_id: Option<String>,
    pub status: SessionStatus,
}

#[async_trait]
impl Aggregate for LearningSession {
    type Command = LearningSessionCommand;
    type Event = LearningSessionEvent;
    type Error = LearningSessionError;
    type Services = ();

    fn aggregate_type() -> String {
        "learning_session".to_string()
    }

    async fn handle(
        &self,
        command: Self::Command,
        _service: &Self::Services,
    ) -> Result<Vec<Self::Event>, Self::Error> {
        // --- Validation before processing the command ---
        let has_been_created = !self.id.is_empty();

        match command {
            StartSession {
                session_id,
                deck_id,
                mut cards_to_review,
                question_language,
                answer_language,
            } => {
                if has_been_created {
                    return Err(SessionAlreadyStarted);
                }

                let mut events: Vec<LearningSessionEvent> = Vec::new();
                events.push(SessionStarted {
                    session_id,
                    deck_id,
                    cards_to_review: cards_to_review.clone(),
                    question_language,
                    answer_language,
                });

                if let Some(first_card_id) = cards_to_review.pop() {
                    events.push(CardPresented {
                        card_id: first_card_id,
                    });
                } else {
                    events.push(SessionCompleted);
                }

                Ok(events)
            }

            _ if !has_been_created => Err(SessionNotFound),
            _ if self.status != SessionStatus::InProgress => Err(SessionNotActive),

            AbandonSession => Ok(vec![SessionAbandoned]),

            AnswerCard {
                rating,
                card_before_review,
            } => {
                let card_id = self.current_card_id.as_ref().ok_or(NoCardToAnswer)?.clone();

                // Use the FSRS scheduler with the correct type name
                let fsrs = FSRS::new(Parameters::default());
                let now = Utc::now();
                let updated_card = fsrs.scheduler(card_before_review, now).review(rating).card;

                let mut events = vec![CardAnswered {
                    card_id,
                    rating,
                    updated_card,
                }];

                let mut remaining_cards = self.cards_to_review.clone();
                if let Some(next_card_id) = remaining_cards.pop_front() {
                    events.push(CardPresented {
                        card_id: next_card_id,
                    });
                } else {
                    events.push(SessionCompleted);
                }

                Ok(events)
            }
        }
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            SessionStarted {
                session_id,
                deck_id,
                cards_to_review,
                question_language,
                answer_language,
            } => {
                self.id = session_id;
                self.deck_id = deck_id;
                self.cards_to_review = cards_to_review.into();
                self.question_language = Some(question_language);
                self.answer_language = Some(answer_language);
                self.status = SessionStatus::InProgress;
            }
            SessionAbandoned | SessionCompleted => {
                self.status = SessionStatus::Completed;
                self.current_card_id = None;
                self.cards_to_review.clear();
            }
            CardPresented { card_id } => {
                self.current_card_id = Some(card_id);
                self.cards_to_review.pop_front();
            }
            CardAnswered { .. } => {}
        }
    }
}

impl View<LearningSession> for LearningSession {
    fn update(&mut self, event: &EventEnvelope<LearningSession>) {
        self.apply(event.payload.clone());
    }
}
