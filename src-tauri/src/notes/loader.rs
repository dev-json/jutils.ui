use crate::notes::note::Note;

pub struct Loader
{
    pub(self) note_list: Vec<Note>
}

impl Loader {
    pub fn new() -> Self {
        println!("Initialized new note_loader()");

        Self {
            note_list: Vec::new()
        }

    }


    pub fn load_notes(&mut self, offset: i32, limit: i32) -> &Vec<Note> {
        println!("trying to load notes: offset: {0} limit {1}", offset, limit);
        &self.note_list.push(
            Note::new("Test-Note-1".to_string(), "Content-1".to_string())
        );
        &self.note_list.push(
            Note::new("Test-Note-2".to_string(), "Content-2".to_string())
        );
        &self.note_list
    }

}
