use cqrs_es::{Aggregate, EventEnvelope, View};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::collections::HashMap;

/// A generic collection view that holds a HashMap of individual aggregate views.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(bound(serialize = "A: Serialize", deserialize = "A: DeserializeOwned"))]
pub struct Collection<A>(pub HashMap<String, A>)
where
    A: Aggregate + View<A>;

impl<A> View<A> for Collection<A>
where
    A: Aggregate + View<A> + std::fmt::Debug,
{
    fn update(&mut self, event: &EventEnvelope<A>) {
        self.0
            .entry(event.aggregate_id.clone())
            .or_default()
            .update(event);
    }
}
