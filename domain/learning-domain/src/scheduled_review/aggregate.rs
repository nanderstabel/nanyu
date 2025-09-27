use std::{collections::HashMap, marker::PhantomData, sync::Arc};

use async_trait::async_trait;
use cqrs_es::{
    Aggregate, EventEnvelope, Query, View,
    persist::{ViewContext, ViewRepository},
};
use serde::{Deserialize, Serialize};

use super::{
    command::ScheduledReviewCommand::{self, *},
    error::ScheduledReviewError::{self, *},
    event::ScheduledReviewEvent::{self, *},
};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ScheduledReview {
    pub id: String,
    pub question: String,
    pub answer: String,
}

impl View<ScheduledReview> for ScheduledReview {
    fn update(&mut self, event: &EventEnvelope<ScheduledReview>) {
        match &event.payload {
            ScheduledReviewCreated { id } => {
                self.id = id.clone();
            }
            ScheduledReviewDeleted { id } => {
                self.id = id.clone();
            }
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct AllScheduledReviews {
    #[serde(flatten)]
    pub scheduled_reviews: HashMap<String, ScheduledReview>,
}

impl View<ScheduledReview> for AllScheduledReviews {
    fn update(&mut self, event: &EventEnvelope<ScheduledReview>) {
        self.scheduled_reviews
            // Get the entry for the aggregate_id
            .entry(event.aggregate_id.clone())
            // or insert a new one if it doesn't exist
            .or_default()
            // update the view with the event
            .update(event);
    }
}

#[async_trait]
impl Aggregate for ScheduledReview {
    type Command = ScheduledReviewCommand;
    type Event = ScheduledReviewEvent;
    type Error = ScheduledReviewError;
    type Services = ();

    fn aggregate_type() -> String {
        "flashcard".to_string()
    }

    async fn handle(
        &self,
        command: Self::Command,
        _service: &Self::Services,
    ) -> Result<Vec<Self::Event>, Self::Error> {
        match command {
            CreateScheduledReview { id } => Ok(vec![ScheduledReviewCreated { id }]),
            DeleteScheduledReview { id } => Ok(vec![ScheduledReviewDeleted { id }]),
        }
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            ScheduledReviewCreated { id } => {
                self.id = id;
            }
            ScheduledReviewDeleted { id } => {
                self.id = id;
            }
        }
    }
}
