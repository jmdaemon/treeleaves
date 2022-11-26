#![allow(dead_code, unused_variables)]

// Standard Library
use std::fs::read_to_string;
use std::str;
//use std::alloc::System;

// Third Party Libraries
use clap::{arg, Command};
use rusqlite::{Connection, Result};
use walkdir::WalkDir;
use md5::{Md5, Digest};
//use digest::consts::U8;
use digest::consts::U16;
use hex;
//use hex_literal::hex;

#[derive(Debug)]
struct ImageFile {
    id: i32,
    filename: String,
    tags: String,
    creation_date: String,
    last_modified: String,
    sha1: String,
    md5: String,
    source: String,
}

fn create_images_db(dbfname: String) -> Result<Connection, rusqlite::Error> {
    // TODO: Make the database with the given filename
    let conn = Connection::open_in_memory();
    conn
}

fn create_images_db_table(conn: &Connection) -> Result<usize, rusqlite::Error> {
    const SQL_CREATE_IMAGES_TABLE_TAGS_STRING: &str = "
    CREATE TABLE images (
        id    INTEGER PRIMARY KEY,
        filename TEXT NOT NULL,
        tags TEXT,
        creation_date TEXT NOT NULL,
        last_modified TEXT NOT NULL,
        sha1 TEXT NOT NULL,
        md5 TEXT NOT NULL,
        source BLOB
    )";

    conn.execute(
        SQL_CREATE_IMAGES_TABLE_TAGS_STRING, (), // empty list of parameters
    )
}

fn test_image_db_insert(conn: &Connection) -> Result<usize, rusqlite::Error> {
    let test_image = ImageFile {
        id: 0,
        filename: "~/File.png".to_string(),
        //tags: vec!("cute".to_string(), "animated".to_string()),
        tags: "cute; animated".to_string(),
        creation_date: "2022-11-24T00:00:00".to_string(),
        last_modified: "2022-11-24T00:00:00".to_string(),
        sha1: "b7fcec7f68cb90f18c39988f69c632d94e829d0c".to_string(),
        md5: "fd2f82b6722681764075fce841a31b6f".to_string(),
        source: "https://ponerpics.org/images/1048610".to_string()
    };

    conn.execute(
        "INSERT INTO images (filename, tags, creation_date, last_modified, sha1, md5, source) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        (&test_image.filename, &test_image.tags, &test_image.creation_date,
         &test_image.last_modified, &test_image.sha1, &test_image.md5, &test_image.source),
    )
}

// TODO: Return the image_iter directly from this function
fn test_image_db_select(conn: &Connection) {
    let mut stmt = conn.prepare("SELECT id, filename, tags, creation_date, last_modified, sha1, md5, source FROM images").unwrap();
    let image_iter = stmt.query_map([], |row| {
        Ok(ImageFile {
            id: row.get(0)?,
            filename: row.get(1)?,
            tags: row.get(2)?,
            creation_date: row.get(3)?,
            last_modified: row.get(4)?,
            sha1: row.get(5)?,
            md5: row.get(6)?,
            source: row.get(7)?,
        })
    }).unwrap();

    for image_file in image_iter {
        println!("Found image {:?}", image_file.unwrap());
    }
}

//fn md5sum(contents: String) -> md5::Md5::u8, Self::OutputSize> {
//fn md5sum(contents: String) -> md5::digest::generic_array::GenericArray<u8, md5::digest::typenum::UInt<md5::digest::typenum::UInt<md5::digest::typenum::UInt<md5::digest::typenum::UInt<md5::digest::typenum::UInt<md5::digest::typenum::UTerm, md5::digest::typenum::B1>, md5::digest::typenum::B0>, md5::digest::typenum::B0>, md5::digest::typenum::B0>, md5::digest::typenum::B0>> {
//fn md5sum(contents: String) -> md5::digest::generic_array::GenericArray<u8, md5::digest::Digest::OutputSize> {
fn md5sum(contents: String) -> md5::digest::generic_array::GenericArray<u8, U16> {
//fn md5sum(contents: String) -> Vec<u8> {
    // create a Md5 hasher instance
    let mut hasher = Md5::new();

    // process input message
    hasher.update(contents.as_bytes());

    // acquire hash digest in the form of GenericArray,
    // which in this case is equivalent to [u8; 16]
    let result = hasher.finalize();
    //md5::Digest::output_size();
    result
    //result.to_vec()
    //result.to_vec_in(System);
    //result.as_slice()
}

const PROGRAM_NAME: &str        = "treeleaves";
const VERSION: &str             = "0.1.0";
const AUTHOR: &str              = "Joseph Diza. <josephm.diza@gmail.com>";
const PROGRAM_DESCRIPTION: &str = "Tag and search files easily";

fn main() -> Result<()> {

    let matches = Command::new(PROGRAM_NAME)
        .version(VERSION)
        .author(AUTHOR)
        .about(PROGRAM_DESCRIPTION)
        //.arg(arg!(--two <VALUE>).required(true))
        //.arg(arg!(--one <VALUE>).required(true))
        .subcommand(
            Command::new("create")
            .about("Creates the database with the given filename")
            .arg(arg!([FILENAME]).required(true)),
            )
        .subcommand(
            Command::new("populate")
            .about("Populates the database")
            .arg(arg!([FILENAME]).required(true))
            .arg(arg!([WORKING_DIR]).required(true))
            )
        .get_matches();

    match matches.subcommand() {
        Some(("create", sub_matches)) => {
            let dbfname = sub_matches.get_one::<String>("FILENAME");
            let conn = create_images_db(dbfname.unwrap().to_owned())?;
            create_images_db_table(&conn)?;
            test_image_db_insert(&conn)?;
            test_image_db_select(&conn);
        }
        Some(("populate", sub_matches)) => {
            let dbfname = sub_matches.get_one::<String>("FILENAME");
            let cwd = sub_matches.get_one::<String>("WORKING_DIR");
            
            // Walk the directory of files
            // Get the full file path, and process the filename into tags
            // Processing:
            // 1. Add support for booru style file name tags [FEATURE]
            // 2. If not available from a booru, chop the file path into tags, and format appropriately
            // 3. Add the file to the database with the given tags
            let files = WalkDir::new(cwd.unwrap().as_str());
            for entry in files {
                let entry = entry.unwrap();
                if entry.path().is_dir() {
                    continue; // Ignore directories
                }
                // TODO: Check if a file name matches any booru regex pattern
                // TODO: Prompt booru and retrieve html result for tags
                println!("{}", entry.path().display());

                // TODO: Add the file to the database
                //let md5 = md5sum(entry.metadata().or_else
                let conts = read_to_string(entry.path()).unwrap();
                println!("{conts}");
                let md5 = md5sum(conts);
                //println!("{}", str::from_utf8(&md5.to_owned()).unwrap());

                //let md5bytes = &md5.to_owned();
                //let md5str = str::from_utf8(md5bytes).unwrap();
                //println!("{}", md5str);
                //let s = String::from_utf8_lossy(&md5);
                //let s = String::from_utf8(md5).expect("Found invalid UTF-8");
                //let s = String::from_utf8(md5).expect("Found invalid UTF-8");
                //println!("{}", s);
                //println!("{:x?}", md5);
                println!("{}", hex::encode(md5));
            }
        }
        _ => {},
    }
    Ok(())
}
