use cqrs_es::DomainEvent;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, strum::Display)]
pub enum ScheduledReviewEvent {
    ScheduledReviewCreated { id: String },
    ScheduledReviewDeleted { id: String },
}

impl DomainEvent for ScheduledReviewEvent {
    fn event_type(&self) -> String {
        self.to_string()
    }

    fn event_version(&self) -> String {
        "1".to_string()
    }
}
