use std::fs;

use diesel::prelude::*;
use indexmap::IndexMap;
use serde_json::Value;

fn main() {
    // Load the data file
    const DB_MIME_JSON: &str = "db.json";
    let conts = fs::read_to_string(DB_MIME_JSON)
        .expect("Could not find db.json");
    let data: IndexMap<&str, Value> = serde_json::from_str(&conts)
        .expect("Could not deserialize data");

    // Load the database
    //dotenv().ok();
    //let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    const MIME_DATA_URL: &str = ".env/mime_types";
    let database_url = fs::read_to_string(MIME_DATA_URL)
        .expect("Could not read database url");
    
    let con = SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    // Serialize to the database
    
    for (k, v) in data {
    }
}
