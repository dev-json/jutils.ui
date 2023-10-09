pub struct NoteStruct
{
    pub(self) note_name: String,
    pub(self) note_content: String
}


pub trait Note {
    fn new(note_name: String, note_content: String) -> Self;
}

impl Note for NoteStruct {
    fn new(note_name: String, note_content: String) -> Self {
        Self {
            note_name,
            note_content
        }
    }
}
