// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;

use std::path::PathBuf;
use rand::Rng;
use dirs;
use crate::db::Database;


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn randommessage() -> String {
    let mut rng = rand::thread_rng();
    format!("{} generated by Rust", rng.gen::<u32>())
}

fn main() {


    let default_path: String = {
        let user_home = match dirs::home_dir() {
            Some(path) => Some(path.to_string_lossy().into_owned()),
            None => Some(String::from("./jutils.ui"))
        };
        let mut tmp_path_buf = PathBuf::new();
        tmp_path_buf.push(user_home.unwrap());
        tmp_path_buf.push("jutils.ui");
        tmp_path_buf.to_string_lossy().into_owned()
    };

    std::fs::create_dir_all(PathBuf::from(default_path.clone()).as_path())
        .expect("Unable to create parent directories!");

    let mut database_ins: Database = Database::new((default_path + "\\data.db"));

    database_ins.use_batch(true);
    database_ins.execute(String::from("CREATE TABLE u_settings(id VARCHAR(255) NOT NULL, value VARCHAR(255), PRIMARY KEY (id));").as_str());
    database_ins.execute(format!("INSERT INTO u_settings(id, value) VALUES ({0}, {1});", "\"notes_save_location\"", "\"{HOME-DIR}/notes/\"").as_str());
    database_ins.commit();
    database_ins.use_batch(false);

    println!("{} overall rows updated", database_ins.get_rows_updated());

    println!("Application started!");
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            randommessage
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}
