use dirs;

#[tauri::command]
pub async fn get_user_dir() -> Option<String> {
     match dirs::home_dir() {
        Some(path) => Some(path.to_string_lossy().into_owned()),
        None => {
            println!("Error: Unable to determine user's home directory.");
            None
        }
    }
}