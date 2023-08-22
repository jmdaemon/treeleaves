use std::path::Path;

use crate::database::{SharedConnection, TargetConnection};

pub mod alias;
pub mod band;
pub mod book;
pub mod book_author;
pub mod book_genre;
pub mod book_language;
pub mod book_publication;
pub mod frequency;
pub mod hash;
pub mod photo;
pub mod photo_album;
pub mod song;
pub mod song_album;
pub mod song_genre;
pub mod song_lyric;
pub mod source;
pub mod tag;

// Support additional metadata plugins for the database tables
// These plugins allow you to:
//      1. Create additional shared and target database tables
//      2. Manage data in tables of any of the target/shared databases

// Creation Traits
pub trait SharedExt {
    /** Create and populate a shared database table **/
    fn mk_shared_table(con: &mut SharedConnection);

    /** Delete a shared database table **/
    fn rm_shared_table(con: &mut SharedConnection);
}

pub trait TargetExt {
    /** Create and populate a target database table **/
    fn mk_target_table(con: &mut TargetConnection, dir: &Path);

    /** Delete a target database table **/
    fn rm_target_table(con: &mut TargetConnection, dir: &Path);
}

// Runtime Traits
// TODO:
//      Define interface for
//          1. Querying files for plugins
//          2. 

pub trait SearchFilesExt {
    /** **/
    fn search_for();
}


pub trait TreeleavesPlugin: SharedExt + TargetExt {
}
