use cqrs_es::{CqrsFramework, mem_store::MemStore};
use cqrs_utils::list_all_query::ListAllQuery;
use in_memory_store::MemRepository;
use std::sync::Arc;

// Import application services and domain aggregates
use application::{
    acl::card_management_to_learning::CardManagementLearningIntegration,
    card_management_service::CardManagementService, learning_service::LearningService,
};
use card_management_domain::flashcard::aggregate::{AllFlashcards, Flashcard};
use learning_domain::scheduled_review::aggregate::{AllScheduledReviews, ScheduledReview};

// A struct to hold the fully constructed services
pub struct AppServices {
    pub learning_service: Arc<
        LearningService<
            MemStore<ScheduledReview>,
            MemRepository<ScheduledReview, ScheduledReview>,
            MemRepository<AllScheduledReviews, ScheduledReview>,
        >,
    >,
    pub card_management_service: Arc<
        CardManagementService<
            MemStore<Flashcard>,
            MemRepository<Flashcard, Flashcard>,
            MemRepository<AllFlashcards, Flashcard>,
        >,
    >,
}

pub struct AppBuilder;

impl AppBuilder {
    pub fn new() -> Self {
        Self
    }

    pub fn build(&self) -> AppServices {
        let scheduled_review_repo = Arc::new(MemRepository::new());
        let all_scheduled_reviews_repo = Arc::new(MemRepository::new());
        let all_reviews_query =
            ListAllQuery::new(all_scheduled_reviews_repo.clone(), "all_scheduled_reviews");
        let learning_cqrs = CqrsFramework::new(
            MemStore::default(), // Using MemStore for this example
            vec![Box::new(all_reviews_query)],
            (),
        );
        let learning_service = Arc::new(LearningService::new(
            learning_cqrs,
            scheduled_review_repo,
            all_scheduled_reviews_repo,
        ));

        let flashcard_repo = Arc::new(MemRepository::new());
        let all_flashcards_repo = Arc::new(MemRepository::new());
        let all_flashcards_query = ListAllQuery::new(all_flashcards_repo.clone(), "all_flashcards");

        // Create the ACL instance, giving it the learning_service
        let card_to_learning_integration =
            CardManagementLearningIntegration::new(learning_service.clone());

        let card_management_cqrs = CqrsFramework::new(
            MemStore::default(), // Using MemStore for this example
            vec![
                Box::new(all_flashcards_query),
                Box::new(card_to_learning_integration),
            ],
            (),
        );
        let card_management_service = Arc::new(CardManagementService::new(
            card_management_cqrs,
            flashcard_repo,
            all_flashcards_repo,
        ));

        // Return all constructed services
        AppServices {
            learning_service,
            card_management_service,
        }
    }
}
