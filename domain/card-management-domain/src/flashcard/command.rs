pub enum FlashcardCommand {
    CreateFlashcard {
        id: String,
        question: String,
        answer: String,
    },
    DeleteFlashcard {
        id: String,
    },
}
