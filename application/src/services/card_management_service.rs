use crate::{
    cqrs_utils::collection::Collection,
    services::{Service, ServiceBuilder},
};
use card_management_domain::deck::aggregate::Deck;
use cqrs_es::{CqrsFramework, EventStore, persist::ViewRepository};
use std::sync::Arc;

pub struct CardManagementService<ES>
where
    ES: EventStore<Deck>,
{
    pub cqrs: CqrsFramework<Deck, ES>,
    pub deck_view_repository: Arc<dyn ViewRepository<Deck, Deck>>,
    pub deck_collection_view_repository: Arc<dyn ViewRepository<Collection<Deck>, Deck>>,
}

impl<ES> CardManagementService<ES>
where
    ES: EventStore<Deck> + 'static,
{
    pub fn new(
        cqrs: CqrsFramework<Deck, ES>,
        deck_view_repository: Arc<dyn ViewRepository<Deck, Deck>>,
        deck_collection_view_repository: Arc<dyn ViewRepository<Collection<Deck>, Deck>>,
    ) -> Self {
        Self {
            cqrs,
            deck_view_repository,
            deck_collection_view_repository,
        }
    }

    pub fn builder() -> ServiceBuilder<Self> {
        ServiceBuilder::new()
    }
}

impl<ES> Service for CardManagementService<ES>
where
    ES: EventStore<Deck>,
{
    type Aggregate = Deck;
    type View = Deck;
    type EventStore = ES;

    fn new(
        cqrs: CqrsFramework<Self::Aggregate, Self::EventStore>,
        individual_repo: Arc<dyn ViewRepository<Self::View, Self::Aggregate>>,
        collection_repo: Arc<dyn ViewRepository<Collection<Self::Aggregate>, Self::Aggregate>>,
    ) -> Self {
        Self {
            cqrs,
            deck_view_repository: individual_repo,
            deck_collection_view_repository: collection_repo,
        }
    }
}
