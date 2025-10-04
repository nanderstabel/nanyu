use serde::{Deserialize, Serialize};

/// Represents the user's self-assessed quality of an answer.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AnswerQuality {
    Again,
    Hard,
    Good,
    Easy,
}
