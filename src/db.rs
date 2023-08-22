//use crate::{schema::postgres::*, models::*};
use crate::{schema::postgres, models::*};

use serde_json::Value;
use indexmap::IndexMap;
use diesel::prelude::*;

// Database Cluster URLs
pub const DB_SHARED_URL: &str = include_str!("url.shared");
pub const DB_TARGET_URL: &str = include_str!("url.target");

// Data
const DATA_MIME_JSON: &str = "data/db.json";

pub type DataMap<'a> = IndexMap<&'a str, Value>;

pub enum DatabaseClusterType {
    SHARED,
    TARGET
}

pub fn connect_db_cluster(cluster_type: DatabaseClusterType) -> PgConnection {
    let url = match cluster_type {
        DatabaseClusterType::SHARED => DB_SHARED_URL,
        DatabaseClusterType::TARGET => DB_TARGET_URL
    };
    PgConnection::establish(url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", url))
}

// MIMEType
use std::fs;
use diesel::prelude::*;
//use super::{DATA_MIME_JSON, DataMap, mime_types::mime_types::dsl::*, MIMEType};
//use ::mime_types::dsl::*;
//use mime_types::{mime_types::dsl::*, MIMEType};
use postgres::mime_types::mime_types::dsl::*;

macro_rules! batch_insert {
    ($con:expr, $table_name:expr, $table:expr, $records:expr) => {
        diesel::insert_into($table)
            .values($records)
            .execute($con)
            .expect(&format!("Could not fill {} table", $table_name));
        
    };
}
//pub fn batch_insert<T, V, VT>(con: &mut PgConnection, table_name: &str, table: T, records: &Vec<V>)
//where
    //T: diesel::Table + diesel::query_builder::QueryId,
    //V: diesel::Insertable<T> + diesel::query_source::Table
//{
    //diesel::insert_into(table)
        //.values(records)
        //.execute(con)
        //.expect(&format!("Could not fill {} table", table_name));
//}

pub fn pop_mime_types(con: &mut PgConnection) {
    // Retrieve our media type data
    let conts = fs::read_to_string(DATA_MIME_JSON)
        .expect("Could not find db.json");
    let data: DataMap = serde_json::from_str(&conts)
    .expect("Could not deserialize data");

    // Serialize it to the database
    let mut media_types = vec![];
    let mut index = 1;
    for (k, _) in data {
        let media_type =  MIMEType { id: index, mime_type: k.to_string() };
        media_types.push(media_type);
        index += 1;
    }
    batch_insert!(con, "mime_types", mime_types, media_types);
}
