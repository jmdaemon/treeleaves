use std::{
    convert::Into,
    time::SystemTime,
    path::Path, fs::Metadata, borrow::Cow,
};

// Retrieve data for all the database tables

use md5::Md5;
use mime::Mime;
use sha1::Sha1;
use indexmap::IndexMap;

use chrono::prelude::*;
use serde_json::Value;

pub const DEFFAULT_MIME_TYPE: Mime = mime::APPLICATION_OCTET_STREAM;

pub const DATA_MIME_JSON: &str = "data/db.json";

pub type DataMap<'a> = IndexMap<&'a str, Value>;

pub fn md5sum(conts: &str) -> String {
    use md5::Digest;
    hex::encode(Md5::digest(conts))
}

pub fn sha1sum(conts: &str) -> String {
    use sha1::Digest;
    hex::encode(Sha1::digest(conts))
}

pub fn timestamp(systime: SystemTime) -> String {
    Into::<DateTime<Utc>>::into(systime).to_rfc3339()
}

pub fn metadata(path: &Path) -> Metadata {
    path.metadata()
        .unwrap_or_else(|_| panic!("Could not retrieve metadata for {}", path.to_string_lossy()))
}

pub fn creation_date(metadata: &Metadata) -> String {
    timestamp(metadata.created()
        .expect("Could not access creation date"))
}

pub fn modified_date(metadata: &Metadata) -> String {
    timestamp(metadata.created()
        .expect("Could not access modification date"))
}

pub fn size(metadata: &Metadata) -> u64 {
    metadata.len()
}

// `path` must exist on disk
pub fn get_mime_type(path: &Path) -> Mime {
    mime_guess::from_path(path).first().unwrap_or(DEFFAULT_MIME_TYPE)
}

pub fn file_hierarchy<'a>(path: &'a Path, root: &Path) -> Vec<Cow<'a, str>> {
    let mut hierarchy = vec![];
    for ancestor in path.ancestors() {
        if ancestor == root { break; }
        hierarchy.push(ancestor.to_string_lossy());
    }
    hierarchy
}

pub fn tags_from_file_hierarchy(path: &Path, root: &Path) -> String {
    file_hierarchy(path, root).join(", ")
}
