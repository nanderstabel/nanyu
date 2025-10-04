use async_trait::async_trait;
use cqrs_es::{Aggregate, EventEnvelope, View};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{
    command::DeckCommand::{self, *},
    entities::flashcard::Flashcard,
    error::DeckError::{self, *},
    event::{
        DeckEvent::{self, *},
        FlashcardDto,
    },
};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Deck {
    pub id: String,
    pub name: String,
    pub flashcards: IndexMap<String, Flashcard>,
}

#[async_trait]
impl Aggregate for Deck {
    type Command = DeckCommand;
    type Event = DeckEvent;
    type Error = DeckError;
    type Services = ();

    fn aggregate_type() -> String {
        "deck".to_string()
    }

    async fn handle(
        &self,
        command: Self::Command,
        _service: &Self::Services,
    ) -> Result<Vec<Self::Event>, Self::Error> {
        // --- Validation before processing the command ---
        let has_been_created = !self.id.is_empty();

        match command {
            CreateDeck { id, name } => {
                if has_been_created {
                    return Err(DeckAlreadyExists);
                }
                Ok(vec![DeckCreated { id, name }])
            }

            // For all other commands, the deck must exist first.
            _ if !has_been_created => Err(DeckNotFound),

            DeleteDeck { id } => Ok(vec![DeckDeleted { id }]),
            RenameDeck { id, new_name } => Ok(vec![DeckRenamed { id, new_name }]),

            AddFlashcard {
                dutch,
                mandarin,
                pinyin,
                english,
            } => {
                let flashcard_id = Uuid::new_v4().to_string();

                Ok(vec![FlashcardAdded(FlashcardDto {
                    id: flashcard_id,
                    dutch,
                    mandarin,
                    pinyin,
                    english,
                })])
            }

            RemoveFlashcard { flashcard_id } => {
                if !self.flashcards.contains_key(&flashcard_id) {
                    return Err(FlashcardNotFound(flashcard_id));
                }

                Ok(vec![FlashcardRemoved { flashcard_id }])
            }

            UpdateFlashcardContent {
                flashcard_id,
                dutch,
                mandarin,
                pinyin,
                english,
            } => {
                if !self.flashcards.contains_key(&flashcard_id) {
                    return Err(FlashcardNotFound(flashcard_id));
                }

                Ok(vec![FlashcardContentUpdated(FlashcardDto {
                    id: flashcard_id,
                    dutch,
                    mandarin,
                    pinyin,
                    english,
                })])
            }
        }
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            DeckCreated { id, name } => {
                self.id = id;
                self.name = name;
            }
            DeckRenamed { id, new_name } => {
                self.id = id;
                self.name = new_name;
            }
            DeckDeleted { id } => {
                self.id = id;
            }
            FlashcardAdded(flashcard_dto) => {
                let flashcard: Flashcard = flashcard_dto.into();
                self.flashcards.insert(flashcard.id.clone(), flashcard);
            }
            FlashcardRemoved { flashcard_id } => {
                self.flashcards.shift_remove(&flashcard_id);
            }
            FlashcardContentUpdated(flashcard_dto) => {
                let flashcard: Flashcard = flashcard_dto.into();
                self.flashcards.insert(flashcard.id.clone(), flashcard);
            }
        }
    }
}

impl View<Deck> for Deck {
    fn update(&mut self, event: &EventEnvelope<Deck>) {
        self.apply(event.payload.clone());
    }
}
