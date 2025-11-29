use tauri::{AppHandle, State};
use tauri_plugin_sql::{DbInstances, DbPool};

use crate::db_models::{Profiles, DB_NAME};

/// Helper function to work with the database pool
/// Takes a closure that receives a reference to the DbPool
pub async fn with_db_pool<F, R>(db_instances: &State<'_, DbInstances>, f: F) -> Result<R, ()>
where
    F: FnOnce(&DbPool) -> Result<R, ()>,
{
    let instances = db_instances.0.read().await;
    let db = instances.get(DB_NAME).ok_or(())?;
    f(db)
}

#[tauri::command]
#[specta::specta]
pub async fn create_profile(
    name: String,
    password: String,
    app: tauri::AppHandle, //
) -> Result<(), String> {
    Profiles::create_new(&name, &password, app).await
}
#[tauri::command]
#[specta::specta]
pub async fn list_profile(app: tauri::AppHandle) -> Result<Vec<Profiles>, String> {
    Profiles::list(app).await
}

pub async fn update_profile_password(profile_id: String, password: String) -> Result<(), ()> {
    todo!()
}

pub fn validate_password(profile_id: String, password: String) -> Result<(), ()> {
    todo!()
}

pub fn create_password(
    profile_id: String,
    password: String,
    description: String,
) -> Result<(), ()> {
    todo!()
}

pub fn decrypt(encrypted_password: String) -> Result<(), ()> {
    todo!()
}

pub fn encrypt(password: String, profile_id: String) -> Result<(), ()> {
    todo!()
}
