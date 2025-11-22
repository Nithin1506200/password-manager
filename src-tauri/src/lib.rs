use crate::{
    db_models::DB_NAME,
    migrations::get_migrations,
    services::{create_profile, list_profile},
};

mod commands;
pub mod common;
mod db_models;
mod encrypt;
pub mod migrations;
pub mod schema;
pub mod services;
use specta_typescript::Typescript;
use tauri::Builder;
use tauri_specta::collect_commands;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
#[specta::specta]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_sql::Builder::new()
                .add_migrations(DB_NAME, get_migrations())
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            create_profile,
            list_profile
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
#[test]
fn specta() {
    use crate::services::{create_profile, list_profile};
    use serde::{Deserialize, Serialize};
    use specta_typescript::Typescript;
    use tauri_specta::{collect_commands, Builder};
    let builder = Builder::<tauri::Wry>::new()
        // Then register them (separated by a comma)
        .commands(collect_commands![greet, create_profile, list_profile]);

    #[cfg(debug_assertions)] // <- Only export on non-release builds
    builder
        .export(Typescript::default(), "../src/bindings.ts")
        .expect("Failed to export typescript bindings");

    tauri::Builder::default()
        // and finally tell Tauri how to invoke them
        .invoke_handler(builder.invoke_handler())
        .setup(move |app| {
            // This is also required if you want to use events
            builder.mount_events(app);

            Ok(())
        });
}
