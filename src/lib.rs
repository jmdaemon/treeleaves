//#![feature(trivial_bounds)]

// Declare modules
pub mod models;
pub mod schema;

// Re-export schemas
pub mod schemas {
    // Compile for sqlite3
    pub use super::schema::sqlite3::audio::*;
    pub use super::schema::sqlite3::files::*;
    pub use super::schema::sqlite3::images::*;
    pub use super::schema::sqlite3::main::*;
    pub use super::schema::sqlite3::mime_types::*;
    pub use super::schema::sqlite3::videos::*;
}

use diesel::prelude::*;
use models::MIMEType;

pub fn create_mime_type(conn: &mut SqliteConnection, mime_type: &str) -> MIMEType {
    MIMEType { id: 0, mime_type: "".to_owned()}
}

/*
pub fn create_post(conn: &mut SqliteConnection, title: &str, body: &str) -> Post {
    use crate::schema::posts;

    let new_post = NewPost { title, body };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .returning(Post::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}
*/

/*

use rusqlite::Connection;

// Structure:
// Given the following:
// - User provides a directory [dir] for treeleaves to begin file tracking.
// - User provides name for database [my_files.db]
//
// Then we will:
// - Create directory [dir]/.treeleaves
// - Create [dir]/my_files.db
//      - This table contains the `files` table.
// - Create []

// TODO: Generate SQL table create statements from struct

// TODO: Efficiently recurse through to add all files
#[derive(Debug)]
pub struct File {
    file_id: i32,
    file_path: String,
}

// TODO: Use mime file types
pub enum MimeType {
}

pub struct FileType {
    mimetype: MimeType,
}

#[derive(Debug)]
pub struct Source {
    source_id: i32,
    file_id: i32,
    site_url: String,
}
// Encode variants as databases based on website sources
// e.g ponerpics.db, pixiv.db, deviantart.db

// Used to calculate SHA1, MD5, SHA256, hashes
#[derive(Debug)]
pub struct Hash {
    hash_id: i32,
    file_id: i32,
    hash: String,
}
// Encode variants as databases based on hash sources?
// files/sha_1.db, files/md5.db
// Could also potentially just force files to all have a hash but that could
// be too costly for some applications

pub struct SHA1(Hash);
pub struct MD5(Hash);

#[derive(Debug)]
pub struct Tags {
    tags_id: i32,
    file_id: i32,
    source_id: i32,
    tags: String,
}

#[derive(Debug)]
pub struct Emote {
}
// Encode variants as databases based on emote sources
// e.g local_emotes.db, rewatch_emotes.db


fn create_sql_table(query: &str, conn: &Connection) -> Result<usize, rusqlite::Error> {
    conn.execute(query, (),)
}
*/
