use crate::common::id::ProfileId;
use crate::schema::profiles;
use crate::schema::secrets;
use bcrypt::{hash, DEFAULT_COST};
use chrono::{NaiveDateTime, Utc};
use diesel::debug_query;
use diesel::prelude::*;
use diesel::sqlite::Sqlite;
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::State;
use tauri_plugin_sql::DbInstances;
use tauri_plugin_sql::DbPool;

pub static DB_NAME: &str = "sqlite:password_manager.db";

type DB_INSTANCE<'a> = State<'a, DbInstances>;

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Type)]
#[diesel(table_name = profiles)]
pub struct Profiles {
    #[specta(type = String)]
    pub id: String, // todo make this as proper id
    pub name: String,
    #[specta(type = String)]
    pub created_at: NaiveDateTime,
    pub pass_hash: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = profiles)]
pub struct NewProfile {
    pub id: String,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub pass_hash: String,
}

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
impl Profiles {
    pub async fn create_new(
        name: &String,
        password: &String,
        // db_instance: &State<'_, DbInstances>,
    ) -> Result<(), String> {
        // Generate a new profile ID
        let profile_id = ProfileId::new().to_string();

        // Hash the password
        let password_hash =
            hash(password, DEFAULT_COST).map_err(|e| format!("Failed to hash password: {}", e))?;

        // Get current timestamp
        let now = Utc::now().naive_utc();

        // Create the new profile record
        let new_profile = NewProfile {
            id: profile_id,
            name: name.clone(),
            created_at: now,
            pass_hash: password_hash,
        };

        // Establish Diesel connection to the SQLite database
        // Note: The database file is expected to be at "password_manager.db" based on tauri config

        let mut conn = SqliteConnection::establish(DB_NAME)
            .map_err(|e| format!("Error connecting to database: {}", e))?;
        println!(" connection fghh");

        // Insert the new profile
        let x = diesel::insert_into(profiles::table)
            .values(&new_profile)
            .execute(&mut conn)
            .map_err(|e| format!("Failed to insert profile: {}", e));
        println!(" connection fgrf {:?}", x);
        x?;
        Ok(())
    }

    pub async fn list() -> Result<Vec<Profiles>, String> {
        // Establish Diesel connection to the SQLite database
        let mut conn = SqliteConnection::establish(DB_NAME)
            .map_err(|e| format!("Error connecting to database: {}", e))?;

        // Query all profiles from the database
        let results = profiles::table
            .select(Profiles::as_select())
            .load(&mut conn)
            .map_err(|e| format!("Failed to load profiles: {}", e))?;

        Ok(results)
    }
}
#[test]
fn create_new_test() {
    // Generate a new profile ID
    let profile_id = ProfileId::new().to_string();
    let password = "fasdfasdf";
    let name = "nithin";
    // Hash the password
    let password_hash = hash(password, DEFAULT_COST)
        .map_err(|e| format!("Failed to hash password: {}", e))
        .unwrap();

    // Get current timestamp
    let now = Utc::now().naive_utc();

    // Create the new profile record
    let new_profile = NewProfile {
        id: profile_id,
        name: name.to_string(),
        created_at: now,
        pass_hash: password_hash,
    };
    let state = diesel::insert_into(profiles::table).values(&new_profile);
    let query = debug_query::<Sqlite, _>(&state).to_string();
    println!("query builder{:?}", query);
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = secrets)]
pub struct Secrets {
    pub id: String,
    pub profile_id: String, // FK
    pub created_at: NaiveDateTime,
    pub data: String,
}

// secret
pub struct SecretKeyStore {
    pub version: String, // hash
    pub key: String,
    pub profile_id: String,
}
