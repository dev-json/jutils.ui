use crate::notes::note::Note;

pub struct Loader
{

}

impl Loader {
    pub fn new() -> Self {
        println!("New loader!");
        Self {}
    }
}

pub fn load_notes(offset: i32, limit: i32) -> Vec<Note> {
    let mut vec = Vec::new();
    println!("trying to load notes: offset: {0} limit {1}", offset, limit);
    vec
}