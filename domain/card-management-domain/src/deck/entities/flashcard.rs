use serde::{Deserialize, Serialize};

use crate::deck::event::FlashcardDto;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Flashcard {
    pub id: String,
    pub dutch: String,
    pub mandarin: String,
    pub pinyin: String,
    pub english: String,
}

impl From<FlashcardDto> for Flashcard {
    fn from(dto: FlashcardDto) -> Self {
        Self {
            id: dto.id,
            dutch: dto.dutch,
            mandarin: dto.mandarin,
            pinyin: dto.pinyin,
            english: dto.english,
        }
    }
}
