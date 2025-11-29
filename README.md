# Tauri + React + Typescript

This template should help get you started developing with Tauri, React and Typescript in Vite.

## diesel setups

- cargo install diesel_cli --no-default-features --features sqlite

- `cd src-tauri`
- automatically create table: `diesel migration generate create_items`
  this will generate up.sql and down.sql , write your queries here. Include in get_migration() in migrations.rs.
- generate rust schema: `diesel migration run` will generate rust schemas and automatically write in schema.rs
