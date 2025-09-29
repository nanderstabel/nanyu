use crate::services::{Service, ServiceBuilder};
use card_management_domain::flashcard::aggregate::{Flashcard, FlashcardCollection};
use cqrs_es::{CqrsFramework, EventStore, persist::ViewRepository};
use std::sync::Arc;

pub struct CardManagementService<ES, VR1, VR2>
where
    ES: EventStore<Flashcard>,
    VR1: ViewRepository<Flashcard, Flashcard>,
    VR2: ViewRepository<FlashcardCollection, Flashcard>,
{
    pub cqrs: CqrsFramework<Flashcard, ES>,
    pub flashcard_view_repository: Arc<VR1>,
    pub flashcard_collection_view_repository: Arc<VR2>,
}

impl<ES, VR1, VR2> CardManagementService<ES, VR1, VR2>
where
    ES: EventStore<Flashcard> + 'static,
    VR1: ViewRepository<Flashcard, Flashcard> + 'static,
    VR2: ViewRepository<FlashcardCollection, Flashcard> + 'static,
{
    pub fn new(
        cqrs: CqrsFramework<Flashcard, ES>,
        flashcard_view_repository: Arc<VR1>,
        flashcard_collection_view_repository: Arc<VR2>,
    ) -> Self {
        Self {
            cqrs,
            flashcard_view_repository,
            flashcard_collection_view_repository,
        }
    }

    pub fn builder() -> ServiceBuilder<Self> {
        ServiceBuilder::new()
    }
}

impl<ES, VR1, VR2> Service for CardManagementService<ES, VR1, VR2>
where
    ES: EventStore<Flashcard>,
    VR1: ViewRepository<Flashcard, Flashcard>,
    VR2: ViewRepository<FlashcardCollection, Flashcard>,
{
    type Aggregate = Flashcard;
    type IndividualView = Flashcard;
    type CollectionView = FlashcardCollection;
    type EventStore = ES;
    type IndividualRepo = VR1;
    type CollectionRepo = VR2;

    fn new(
        cqrs: CqrsFramework<Self::Aggregate, Self::EventStore>,
        individual_repo: Arc<Self::IndividualRepo>,
        collection_repo: Arc<Self::CollectionRepo>,
    ) -> Self {
        Self {
            cqrs,
            flashcard_view_repository: individual_repo,
            flashcard_collection_view_repository: collection_repo,
        }
    }
}
