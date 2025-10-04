#[derive(Debug, thiserror::Error)]
pub enum LearningSessionError {
    #[error("Learning session has already been started.")]
    SessionAlreadyStarted,
    #[error("No learning session found.")]
    SessionNotFound,
    #[error("Learning session is not active.")]
    SessionNotActive,
    #[error("No card is currently presented to answer.")]
    NoCardToAnswer,
}
