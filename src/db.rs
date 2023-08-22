use crate::data::{metadata, size, get_mime_type};
use crate::{schema::postgres, models::*};

use std::{fs, path::Path};

use serde_json::Value;
use indexmap::IndexMap;
use diesel::prelude::*;

use walkdir::WalkDir;
use anyhow::Result;

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

macro_rules! batch_insert {
    ($con:expr, $table_name:expr, $table:expr, $records:expr) => {
        diesel::insert_into($table)
            .values($records)
            .execute($con)
            .expect(&format!("Could not fill {} table", $table_name));
        
    };
}

// Tables

// MIMEType
//use postgres::mime_types::mime_types::dsl::*;

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
    use postgres::mime_types::mime_types::dsl::mime_types;
    batch_insert!(con, "mime_types", mime_types, media_types);
}

//use postgres::files::files::dsl::*;

//pub fn pop_files(con: &mut PgConnection, dir: &Path) {
//pub fn pop_files(con: &mut PgConnection, dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
pub fn pop_files(con: &mut PgConnection, dir: &Path) -> Result<()> {
    let treeleaves_root = WalkDir::new(dir);
    let mut id = 1;
    let mut files = vec![];
    for file_entry in treeleaves_root {
        //let fpath = file_entry.unwrap().path();
        //let metadata = metadata(fpath);

        //

        let file_entry = file_entry?;
        let path = file_entry.path();
        let metadata = metadata(path);

        //let (_name) = (fpath.file_name().unwrap());
        let (name, size, mime_type) = (path.file_name().unwrap(), size(&metadata), get_mime_type(path));
        use postgres::mime_types::mime_types::dsl::{mime_types, mime_type as media_type};
        let mime_type_id = mime_types
            .filter(media_type.eq(mime_type.to_string()))
            .first::<(i32, String)>(con)?
            .0;

        let file = File { id,
            name: name.to_string_lossy().to_string(),
            path: path.to_string_lossy().to_string(),
            size: size.try_into()?, mime_type_id
        };
        files.push(file);
        id += 1;
    }
    use postgres::files::files::table as tbl_files;
    batch_insert!(con, "files", tbl_files, files);
    Ok(())
}
