use crate::{
    cqrs_utils::collection::Collection,
    services::{Service, ServiceBuilder},
};
use card_management_domain::flashcard::aggregate::Flashcard;
use cqrs_es::{CqrsFramework, EventStore, persist::ViewRepository};
use std::sync::Arc;

pub struct CardManagementService<ES>
where
    ES: EventStore<Flashcard>,
{
    pub cqrs: CqrsFramework<Flashcard, ES>,
    pub flashcard_view_repository: Arc<dyn ViewRepository<Flashcard, Flashcard>>,
    pub flashcard_collection_view_repository:
        Arc<dyn ViewRepository<Collection<Flashcard>, Flashcard>>,
}

impl<ES> CardManagementService<ES>
where
    ES: EventStore<Flashcard> + 'static,
{
    pub fn new(
        cqrs: CqrsFramework<Flashcard, ES>,
        flashcard_view_repository: Arc<dyn ViewRepository<Flashcard, Flashcard>>,
        flashcard_collection_view_repository: Arc<
            dyn ViewRepository<Collection<Flashcard>, Flashcard>,
        >,
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

impl<ES> Service for CardManagementService<ES>
where
    ES: EventStore<Flashcard>,
{
    type Aggregate = Flashcard;
    type View = Flashcard;
    type EventStore = ES;

    fn new(
        cqrs: CqrsFramework<Self::Aggregate, Self::EventStore>,
        individual_repo: Arc<dyn ViewRepository<Self::View, Self::Aggregate>>,
        collection_repo: Arc<dyn ViewRepository<Collection<Self::Aggregate>, Self::Aggregate>>,
    ) -> Self {
        Self {
            cqrs,
            flashcard_view_repository: individual_repo,
            flashcard_collection_view_repository: collection_repo,
        }
    }
}
