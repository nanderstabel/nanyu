use std::sync::Arc;

use cqrs_es::{CqrsFramework, EventStore, persist::ViewRepository};

use card_management_domain::flashcard::aggregate::{AllFlashcards, Flashcard};

pub struct CardManagementService<ES, VR1, VR2>
where
    ES: EventStore<Flashcard>,
    VR1: ViewRepository<Flashcard, Flashcard>,
    VR2: ViewRepository<AllFlashcards, Flashcard>,
{
    pub cqrs: CqrsFramework<Flashcard, ES>,
    pub flashcard_view_repository: Arc<VR1>,
    pub all_flashcards_view_repository: Arc<VR2>,
}

impl<ES, VR1, VR2> CardManagementService<ES, VR1, VR2>
where
    ES: EventStore<Flashcard>,
    VR1: ViewRepository<Flashcard, Flashcard>,
    VR2: ViewRepository<AllFlashcards, Flashcard>,
{
    pub fn new(
        cqrs: CqrsFramework<Flashcard, ES>,
        flashcard_view_repository: Arc<VR1>,
        all_flashcards_view_repository: Arc<VR2>,
    ) -> Self {
        Self {
            cqrs,
            flashcard_view_repository,
            all_flashcards_view_repository,
        }
    }
}
