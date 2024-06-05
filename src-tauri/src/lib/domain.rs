use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tauri::State;

pub struct App {
    pub db: DatabaseConnection,
}
pub type AppState<'a> = State<'a, Arc<App>>;
