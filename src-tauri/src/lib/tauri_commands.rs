use crate::db::entity::user;
use crate::domain::AppState;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use uuid::Uuid;

#[tauri::command]
pub async fn add_user(app_state: AppState<'_>, name: &str) -> Result<user::Model, String> {
    let user_entity = user::ActiveModel {
        id: Set(Uuid::new_v4().to_string()),
        name: Set(String::from(name)),
    };
    let user_entity: user::Model = user_entity.insert(&app_state.db).await.unwrap();
    Ok(user_entity)
}

#[tauri::command]
pub async fn get_users(app_state: AppState<'_>) -> Result<Vec<user::Model>, String> {
    let users: Vec<user::Model> = user::Entity::find().all(&app_state.db).await.unwrap();
    Ok(users)
}
