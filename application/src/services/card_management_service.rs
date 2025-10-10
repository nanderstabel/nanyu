use card_management_domain::deck::{aggregate::Deck, command::DeckCommand};
use cqrs_es::{CqrsFramework, EventStore};

pub struct CardManagementService<ES>
where
    ES: EventStore<Deck>,
{
    cqrs: CqrsFramework<Deck, ES>,
}

impl<ES> CardManagementService<ES>
where
    ES: EventStore<Deck> + 'static,
{
    pub fn new(cqrs: CqrsFramework<Deck, ES>) -> Self {
        Self { cqrs }
    }

    pub async fn create_new_deck(
        &self,
        deck_id: Option<String>,
        name: String,
    ) -> Result<String, String> {
        let deck_id = deck_id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let command = DeckCommand::CreateDeck {
            id: deck_id.clone(),
            name,
        };

        self.cqrs
            .execute(&deck_id, command)
            .await
            .map_err(|e| e.to_string())?;

        Ok(deck_id)
    }

    pub async fn add_flashcard_to_deck(
        &self,
        deck_id: String,
        dutch: String,
        mandarin: String,
        pinyin: String,
        english: String,
    ) -> Result<(), String> {
        let command = DeckCommand::AddFlashcard {
            dutch,
            mandarin,
            pinyin,
            english,
        };

        self.cqrs
            .execute(&deck_id, command)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}
