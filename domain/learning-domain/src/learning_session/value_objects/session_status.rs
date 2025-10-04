use serde::{Deserialize, Serialize};

/// The status of the learning session.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum SessionStatus {
    #[default]
    NotStarted,
    InProgress,
    Completed,
}
