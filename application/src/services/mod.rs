use std::sync::Arc;

use cqrs_es::{
    Aggregate, CqrsFramework, EventStore, Query, View,
    persist::{GenericQuery, ViewRepository},
};

pub mod card_management_service;
pub mod learning_service;

/// Defines the contract for an application service.
///
/// A Service can be constructed from a CqrsFramework and its associated repositories.
pub trait Service: Sized {
    type Aggregate: Aggregate;
    type IndividualView: View<Self::Aggregate>; // Add this line
    type CollectionView: View<Self::Aggregate>;
    type EventStore: EventStore<Self::Aggregate>;
    type IndividualRepo: ViewRepository<Self::IndividualView, Self::Aggregate>; // Use IndividualView here
    type CollectionRepo: ViewRepository<Self::CollectionView, Self::Aggregate>;

    fn new(
        cqrs: CqrsFramework<Self::Aggregate, Self::EventStore>,
        individual_repo: Arc<Self::IndividualRepo>,
        collection_repo: Arc<Self::CollectionRepo>,
    ) -> Self;
}

/// A generic builder for constructing the core components of an application service.
pub struct ServiceBuilder<S: Service> {
    event_store: Option<S::EventStore>,
    queries: Vec<Box<dyn Query<S::Aggregate>>>,
    individual_repository: Option<Arc<S::IndividualRepo>>,
    collection_repository: Option<Arc<S::CollectionRepo>>,
}

impl<S> ServiceBuilder<S>
where
    S: Service + 'static,
    S::Aggregate: Send + Sync + 'static,
    <<S as Service>::Aggregate as Aggregate>::Services: Default + Send + Sync,
    S::EventStore: Send + Sync + 'static,
    S::IndividualRepo: Send + Sync + 'static,
    S::CollectionRepo: Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self {
            event_store: None,
            queries: vec![],
            individual_repository: None,
            collection_repository: None,
        }
    }

    pub fn with_event_store(mut self, event_store: S::EventStore) -> Self {
        self.event_store = Some(event_store);
        self
    }

    pub fn with_individual_repository(mut self, repository: Arc<S::IndividualRepo>) -> Self {
        self.individual_repository = Some(repository);
        self
    }

    pub fn with_collection_repository(mut self, repository: Arc<S::CollectionRepo>) -> Self {
        self.collection_repository = Some(repository);
        self
    }

    pub fn with_query(mut self, query: Box<dyn Query<S::Aggregate>>) -> Self {
        self.queries.push(query);
        self
    }

    pub fn build(self) -> Result<S, String> {
        let event_store = self.event_store.ok_or("Event store is required")?;
        let individual_repository = self
            .individual_repository
            .ok_or("Individual view repository is required")?;
        let collection_repository = self
            .collection_repository
            .ok_or("Collection view repository is required")?;

        let all_queries = self
            .queries
            .into_iter()
            .chain(
                vec![Box::new(GenericQuery::new(individual_repository.clone()))
                    as Box<dyn Query<S::Aggregate>>]
                .into_iter(),
            )
            .collect();

        let cqrs = CqrsFramework::new(event_store, all_queries, Default::default());

        Ok(S::new(cqrs, individual_repository, collection_repository))
    }
}
