use application::cqrs_utils::outbound_adapter::OutboundAdapter;
use application::services::learning_service::LearningService;
use application::{
    cqrs_utils::collection_projector::CollectionProjector,
    services::card_management_service::CardManagementService,
};
use cqrs_es::{CqrsFramework, Query, mem_store::MemStore};
use in_memory_store::MemRepository;
use std::sync::Arc;

// Import application services and domain aggregates
use application::acl::card_management_to_learning::CardManagementLearningIntegration;
use card_management_domain::flashcard::aggregate::{Flashcard, FlashcardCollection};
use learning_domain::scheduled_review::aggregate::{ScheduledReview, ScheduledReviewCollection};
