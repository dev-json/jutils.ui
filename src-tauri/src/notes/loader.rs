pub struct Loader
{

}

impl Loader {
    pub fn new() -> Self {
        println!("New loader!");
        Self {}
    }

    #[tauri::command]
    pub fn load_notes(&self){
        println!("Trying to load notes!");
    }
}