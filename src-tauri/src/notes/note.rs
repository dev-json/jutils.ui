use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Note
{
    pub(self) note_name: String,
    pub(self) note_content: String
}

impl Note {
    pub(crate) fn new(note_name: String, note_content: String) -> Self {
        Self {
            note_name,
            note_content
        }
    }
}
