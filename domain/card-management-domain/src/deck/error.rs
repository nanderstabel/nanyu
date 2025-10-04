#[derive(Debug, thiserror::Error)]
pub enum DeckError {
    #[error("Invalid deck name provided.")]
    DeckAlreadyExists,
    #[error("Deck not found.")]
    DeckNotFound,

    #[error("Flashcard with ID `{0}` does not exist in the deck.")]
    FlashcardNotFound(String),
}
