use application::services::card_management_service::CardManagementService;
use cqrs_es::EventStore;

pub struct Seeder;

impl Seeder {
    pub async fn seed_data<ES>(
        card_management_service: &CardManagementService<ES>,
    ) -> Result<(), String>
    where
        ES: EventStore<card_management_domain::deck::aggregate::Deck> + 'static,
    {
        let json = include_str!("../seeds/bommel-de-loodhervormer-1.json");
        let flashcards: Vec<serde_json::Value> =
            serde_json::from_str(json).map_err(|e| e.to_string())?;

        let deck_id = "bommel-de-loodhervormer-1.json".to_string();

        card_management_service
            .create_new_deck(
                Some(deck_id.to_string()),
                "Bommel - De Loodhervormer".to_string(),
            )
            .await?;

        for flashcard in flashcards {
            card_management_service
                .add_flashcard_to_deck(
                    deck_id.clone(),
                    flashcard["dutch"].as_str().unwrap().to_string(),
                    flashcard["mandarin"].as_str().unwrap().to_string(),
                    flashcard["pinyin"].as_str().unwrap().to_string(),
                    flashcard["english"].as_str().unwrap().to_string(),
                )
                .await?;
        }

        Ok(())
    }
}
