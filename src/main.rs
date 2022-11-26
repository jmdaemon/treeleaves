#![allow(dead_code, unused_variables)]
use rusqlite::{Connection, Result};

#[derive(Debug)]
struct ImageFile {
    id: i32,
    filename: String,
    //tags: Vec<String>,
    tags: String,
    creation_date: String,
    last_modified: String,
    sha1: String,
    md5: String,
}

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;

    const SQL_CREATE_IMAGES_TABLE_TAGS_BLOG: &str = "\
    CREATE TABLE images ( \
        id    INTEGER PRIMARY KEY, \
        filename TEXT NOT NULL, \
        tags BLOB, \
        creation_date TEXT NOT NULL, \
        last_modified TEXT NOT NULL, \
        sha1 TEXT NOT NULL, \
        md5 TEXT NOT NULL,
    )";

    const SQL_CREATE_IMAGES_TABLE_TAGS_STRING: &str = "
    CREATE TABLE images (
        id    INTEGER PRIMARY KEY,
        filename TEXT NOT NULL,
        tags TEXT NOT NULL,
        creation_date TEXT NOT NULL,
        last_modified TEXT NOT NULL,
        sha1 TEXT NOT NULL,
        md5 TEXT NOT NULL
    )";

    conn.execute(
        SQL_CREATE_IMAGES_TABLE_TAGS_STRING, (), // empty list of parameters
    )?;

    let test_image = ImageFile {
        id: 0,
        filename: "~/File.png".to_string(),
        //tags: vec!("cute".to_string(), "animated".to_string()),
        tags: "cute; animated".to_string(),
        creation_date: "2022-11-24T00:00:00".to_string(),
        last_modified: "2022-11-24T00:00:00".to_string(),
        sha1: "b7fcec7f68cb90f18c39988f69c632d94e829d0c".to_string(),
        md5: "fd2f82b6722681764075fce841a31b6f".to_string(),
    };

    conn.execute(
        "INSERT INTO images (filename, tags, creation_date, last_modified, sha1, md5) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        (&test_image.filename, &test_image.tags, &test_image.creation_date,
         &test_image.last_modified, &test_image.sha1, &test_image.md5),
    )?;

    let mut stmt = conn.prepare("SELECT id, filename, tags, creation_date, last_modified, sha1, md5 FROM images")?;
    let image_iter = stmt.query_map([], |row| {
        Ok(ImageFile {
            id: row.get(0)?,
            filename: row.get(1)?,
            tags: row.get(2)?,
            creation_date: row.get(3)?,
            last_modified: row.get(4)?,
            sha1: row.get(5)?,
            md5: row.get(6)?,
        })
    })?;

    for image_file in image_iter {
        println!("Found image {:?}", image_file.unwrap());
    }
    Ok(())
}
