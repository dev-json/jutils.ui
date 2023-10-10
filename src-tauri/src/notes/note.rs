pub struct Note
{
    pub(self) note_name: String,
    pub(self) note_content: String
}

impl Note {
    fn new(note_name: String, note_content: String) -> Self {
        Self {
            note_name,
            note_content
        }
    }
}
