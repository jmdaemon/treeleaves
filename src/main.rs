#![allow(dead_code, unused_variables)]

// Standard Library
use std::{fs::read_to_string, vec};
use std::io;
use std::path::Path;
use std::str;
use std::time::SystemTime;

// Third Party Libraries
use chrono::{DateTime, Utc};
use clap::{arg, ArgAction, Command};
use hex;
use lazy_static::lazy_static;
use log::{debug, error, trace, info, warn, LevelFilter};
use md5::Md5;
use pretty_env_logger::formatted_timed_builder;
use regex::Regex;
use rusqlite::{Connection, Result};
use sha1::Sha1;
use walkdir::WalkDir;
use tl;

lazy_static! {
    static ref REGEX_PONY_BOORU: Regex = Regex::new(r"^([0-9]{1,7})(_{0,2})").unwrap();
    //static ref REGEX_4CHAN: Regex = Regex::new(r"([0-9]{1,16}").unwrap();
}

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

impl ImageFile {
    pub fn new(id: i32, filename: String, tags: String,
    creation_date: String, last_modified: String,
    sha1: String, md5: String, source: String) -> Self {
        ImageFile { id, filename, tags, creation_date, last_modified, sha1, md5, source }
    }
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

    conn.execute(SQL_CREATE_IMAGES_TABLE_TAGS_STRING, (),) // Empty list of params
}

fn insert_into_db(conn: &Connection, image_file: ImageFile) -> Result<usize, rusqlite::Error> {
    const SQL_INSERT_IMAGE_TABLE: &str =
        "INSERT INTO images (filename, tags, creation_date, last_modified, sha1, md5, source) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)";
    conn.execute(
        SQL_INSERT_IMAGE_TABLE, (
            &image_file.filename, &image_file.tags,
            &image_file.creation_date, &image_file.last_modified,
            &image_file.sha1, &image_file.md5,
            &image_file.source),
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
    insert_into_db(conn, test_image)
}

fn select_image(conn: &Connection) -> Vec<ImageFile> {
    const SQL_SELECT_IMAGE_FROM_TABLE: &str =
        "SELECT id, filename, tags, creation_date, last_modified, sha1, md5, source FROM images";
    let mut stmt = conn.prepare(SQL_SELECT_IMAGE_FROM_TABLE).unwrap();
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

    let mut result: Vec<ImageFile> = vec![];

    for image_file in image_iter {
        result.push(image_file.unwrap());
    }
    result
}

fn test_image_db_select(conn: &Connection) {
    let image_iter = select_image(conn);
    for image_file in image_iter {
        debug!("Found image {:?}", image_file);
    }
}

fn md5sum(contents: &String) -> String {
    use md5::Digest;
    hex::encode(Md5::digest(contents))
}

fn sha1sum(contents: &String) -> String {
    use sha1::Digest;
    hex::encode(Sha1::digest(contents))
}

fn tag_from_filetree(fp: &String) -> String {
    let bfp = fp.split('/');
    let bfpvec: Vec<&str> = bfp.collect();
    let tags = &bfpvec[1..bfpvec.len() - 1];
    String::from(tags.join(","))
}

fn format_time(systime: SystemTime) -> String {
    //let now: DateTime<Utc> = systime.into();
    //let now = nsystime.into().to_rfc3339();
    //now
    let dt: DateTime<Utc> = systime.into();
    dt.to_rfc3339()
}

fn create_image_entry(conts: &String, index: i32, entry: walkdir::DirEntry) -> ImageFile {
    let fp = String::from(entry.path().to_str().unwrap());
    debug!("fp: {}", fp);

    let tags = get_image_file_tags(entry.path());
    debug!("tags: {:?}", tags);

    let creation_date = format_time(entry.path().metadata().unwrap().created().unwrap());
    let modified_date = format_time(entry.path().metadata().unwrap().modified().unwrap());
    debug!("creation_date: {}", creation_date);
    debug!("modified_date: {}", modified_date);

    let md5 = md5sum(&conts);
    let sha1 = sha1sum(&conts);
    debug!("md5: {}", md5);
    debug!("sha1: {}", sha1);

    let image_file = ImageFile::new(index, fp, tags, creation_date, modified_date, sha1, md5, "".to_string());
    image_file
}

fn read_file_conts(fp: &Path) -> Result<String, io::Error> {
    let conts_res = read_to_string(fp);
    
    // Read file contents, skip files with read errors
    let conts = match conts_res {
        Ok(contents) => Ok(contents),
        Err(error) => {
            // Silently skip errors
            // eprintln!("Error when reading file: {:?}", error);
            //continue;
            Err(error)
        },
    };
    conts
}

fn file_is_empty(fp: &Path) -> bool {
    let file_open = std::fs::File::open(fp);
    match file_open {
        Ok(file) => if file.metadata().unwrap().len() == 0 { true } else { false },
        Err(error) => false
    }
}

fn get_file_id(fp: &str, re: &Regex) -> String {
    let caps = re.captures(fp);
    let mut file_id: String = String::new();
    if caps.is_some() {
        file_id = caps.unwrap().get(0).unwrap().as_str().to_string();
    }
    file_id
}

fn get_file_id_ponybooru(fp: &str) -> String { get_file_id(fp, &REGEX_PONY_BOORU) }

fn get_image_file_tags(fp: &Path) -> String {
    let fname = fp.file_name().unwrap().to_str().unwrap().to_string();
    let fextopt = fp.extension();
    let fext: String;

    if fextopt.is_some() {
        fext = fextopt.unwrap().to_str().unwrap().to_string();
    }

    let ponybooru_file_id = get_file_id_ponybooru(fname.as_str());
    debug!("ponybooru_file_id: {ponybooru_file_id}");
    let mut tags: String = "".to_string();

    if Some(ponybooru_file_id).is_some() {
        let match_found = false;
        // TODO:
        // Query the available boorus for tags
        // Check the file hash
        // If a match is found, retrieve all the tags and format the string from the tags
        // Else if not found, treat as a normal file
        if !match_found {
            tags = tag_from_filetree(&fp.to_str().unwrap().to_string());
        }
    }
    tags
}

fn send_req(url: &String) -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get(url)?.text()?;
    //println!("{:#?}", resp);
    Ok(resp)
}

fn fmt_url_req(url: &String, id: &String) -> String {
    //format!("{url}/{id}")
    format!("{}/{}", url, id)
}

fn fetch_tags_ponybooru(dom: &tl::VDom, parser: &tl::Parser) -> Vec<String> {
    let elems = dom.query_selector(".tag__name").unwrap();
    let mut tagvec: Vec<String> = vec![];
    for elem in elems {
        let tags_list = elem.get(parser).unwrap();
        for tagelem in tags_list.children() {
            let tag = tags_list.inner_text(parser);
            debug!("{}", tag);
            tagvec.push(tag.to_string());
        }
    }
    tagvec

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
        .arg(arg!(-v --verbose "Toggle verbose information").action(ArgAction::SetTrue))
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
        .subcommand(
            Command::new("fetch")
            .about("Fetches tags for a file externally")
            .arg(arg!([FILENAME]).required(true))
            .arg(arg!([FILE]).required(true))
            .arg(arg!([TAG_TYPE]).required(true))
            )
        .get_matches();
    
    let verbose: &bool = matches.get_one("verbose").unwrap();

    if *verbose == true {
        formatted_timed_builder().filter_level(LevelFilter::Trace).init();
    }

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

            // TODO: For the real db we'll replace this with a connect instead
            let conn = create_images_db(dbfname.unwrap().to_owned())?;
            create_images_db_table(&conn)?;
            
            // Walk the directory of files
            // Get the full file path, and process the filename into tags
            // Processing:
            // 1. Add support for booru style file name tags [FEATURE]
            // 2. If not available from a booru, chop the file path into tags, and format appropriately
            // 3. Add the file to the database with the given tags
            let files = WalkDir::new(cwd.unwrap().as_str());
            let mut index = 0;
            for entry in files {
                let entry = entry.unwrap();

                if entry.path().is_dir() || file_is_empty(entry.path()) {
                    continue; // Ignore directories and empty files
                }

                // TODO: Check if a file name matches any booru regex pattern
                // TODO: Prompt booru and retrieve html result for tags
                let conts = read_file_conts(entry.path());
                if conts.is_err() {
                    continue; // Ignore unreadable files
                }
                let conts = conts.unwrap();

                // Let's just assume that we are only left with tags from the file folder hierarchy
                let image_file = create_image_entry(&conts, index, entry);
                insert_into_db(&conn, image_file)?;
                index += 1;
            }
        test_image_db_select(&conn);
        }
        Some(("fetch", sub_matches)) => {
            let dbfname = sub_matches.get_one::<String>("FILENAME").unwrap();
            let file = sub_matches.get_one::<String>("FILE").unwrap();
            let tag_type = sub_matches.get_one::<String>("TAG_TYPE").unwrap();

            // Parse file path
            let fp = Path::new(file);
            let fname = fp.file_name().unwrap().to_str().unwrap().to_string();
            let fextopt = fp.extension();

            // Send http request
            match tag_type.as_str() {
                "ponerpics" => {
                    let url = fmt_url_req(&"https://ponerpics.org/".to_string(), file);
                    let html = send_req(&url).unwrap();
                    debug!("HTML Response:\n{html}");

                    debug!("Parsing html response.");
                    let dom = tl::parse(html.as_str(), tl::ParserOptions::default()).unwrap();
                    let parser = dom.parser();
                    
                    debug!("Retrieving tags.");
                    let tagvec = fetch_tags_ponybooru(&dom, &parser);
                    let tags = tagvec.join(",");
                    println!("{}", tags);
                },
                _ => {
                    // No-op
                }
            }
        }
        _ => {},
    }
    Ok(())
}
