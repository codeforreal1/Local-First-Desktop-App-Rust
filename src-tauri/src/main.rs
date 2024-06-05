// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sea_orm::Database;
use sqlx::{migrate::MigrateDatabase, Sqlite};
use std::sync::Arc;

use lib::domain::App;
use lib::tauri_commands::{__cmd__add_user, __cmd__get_users, add_user, get_users};

#[tokio::main]
async fn main() {
    let db_url_string = dotenv::var("DATABASE_URL").unwrap();
    let db_url = db_url_string.as_str();
    if !Sqlite::database_exists(db_url).await.unwrap() {
        println!("Database does not exist. Creating one...");
        match Sqlite::create_database(db_url).await {
            Ok(_) => println!("Database created successfully."),
            Err(err) => panic!("Error occurred during database creation: {}", err),
        }
    }

    let db = Database::connect(db_url).await.unwrap();

    let app = App { db };

    let app_state = Arc::new(app);

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![add_user, get_users])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
