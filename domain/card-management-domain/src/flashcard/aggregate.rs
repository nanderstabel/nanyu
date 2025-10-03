use async_trait::async_trait;
use cqrs_es::{Aggregate, EventEnvelope, View};
use serde::{Deserialize, Serialize};

use super::{
    command::FlashcardCommand::{self, *},
    error::FlashcardError::{self, *},
    event::FlashcardEvent::{self, *},
};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Flashcard {
    pub id: String,
    pub question: String,
    pub answer: String,
}

impl View<Flashcard> for Flashcard {
    fn update(&mut self, event: &EventEnvelope<Flashcard>) {
        match &event.payload {
            FlashcardCreated {
                id,
                question,
                answer,
            } => {
                self.id = id.clone();
                self.question = question.clone();
                self.answer = answer.clone();
            }
            FlashcardDeleted { id } => {
                self.id = id.clone();
                self.question = String::new();
                self.answer = String::new();
            }
        }
    }
}

#[async_trait]
impl Aggregate for Flashcard {
    type Command = FlashcardCommand;
    type Event = FlashcardEvent;
    type Error = FlashcardError;
    type Services = ();

    fn aggregate_type() -> String {
        "flashcard".to_string()
    }

    async fn handle(
        &self,
        command: Self::Command,
        _service: &Self::Services,
    ) -> Result<Vec<Self::Event>, Self::Error> {
        match command {
            CreateFlashcard {
                id,
                question,
                answer,
            } => Ok(vec![FlashcardCreated {
                id,
                question,
                answer,
            }]),
            DeleteFlashcard { id } => Ok(vec![FlashcardDeleted { id }]),
        }
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            FlashcardCreated {
                id,
                question,
                answer,
            } => {
                self.id = id;
                self.question = question;
                self.answer = answer;
            }
            FlashcardDeleted { id } => {
                self.id = id;
                self.question = String::new();
                self.answer = String::new();
            }
        }
    }
}
