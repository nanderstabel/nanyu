pub enum DeckCommand {
    // --- Deck Lifecycle Commands ---
    /// Creates a new, empty deck.
    CreateDeck { id: String, name: String },

    /// Deletes an entire deck and all flashcards within it.
    DeleteDeck { id: String },

    /// Renames an existing deck.
    RenameDeck { id: String, new_name: String },

    // --- Flashcard Management Commands (within the Deck) ---
    /// Adds a new flashcard to the deck. The Deck aggregate is responsible
    /// for generating the new flashcard's ID.
    AddFlashcard {
        dutch: String,
        mandarin: String,
        pinyin: String,
        english: String,
    },

    /// Removes a specific flashcard from the deck.
    RemoveFlashcard { flashcard_id: String },

    /// Updates the content of a specific flashcard within the deck.
    UpdateFlashcardContent {
        flashcard_id: String,
        dutch: String,
        mandarin: String,
        pinyin: String,
        english: String,
    },
}
