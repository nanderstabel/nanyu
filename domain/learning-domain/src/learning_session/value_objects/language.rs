use serde::{Deserialize, Serialize};

/// Represents the different languages or text representations available on a flashcard.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Language {
    Dutch,
    Mandarin,
    Pinyin,
    English,
}
