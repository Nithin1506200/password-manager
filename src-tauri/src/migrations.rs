use tauri_plugin_sql::{Migration, MigrationKind};

pub fn get_migrations() -> Vec<Migration> {
    let init = Migration {
        version: 1,
        description: "create_initial_tables",
        sql: include_str!("../migrations/2025-10-25-183758-0000_create_items/up.sql"),
        kind: MigrationKind::Up,
    };
    vec![init]
}
