use std::path::{Path, PathBuf};
use crate::api::{get_user_dir};


pub struct Config
{
    path: str
}

impl Config
{
    pub fn new(path: str) -> Self
    {
        Self { path }
    }

    #[tauri::command]
    pub async fn dir_exists() -> bool {
        if let Some(user_dir) = get_user_dir().await {
            let mut target_path: PathBuf = PathBuf::new();
            target_path.push(user_dir);
            target_path.push("jnotes");
            Path::new(&target_path).is_dir()
        } else {
            false
        }
    }

}

/*
#[tauri::command]
pub async fn dir_exists_2() -> bool {
    if let Some(user_dir) = get_user_dir().await {
        let mut target_path: PathBuf = PathBuf::new();
        target_path.push(user_dir);
        target_path.push("jnotes");
        Path::new(&target_path).is_dir()
    } else {
        false
    }
}
*/

