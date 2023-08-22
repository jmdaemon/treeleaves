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

// Support additional addons for the database tables
// These addons allow you to:
//      1. Create additional shared/target databases or database tables
//      2. View any data found in anyone of the defined databases and database tables
//      3. Modify any data found in said databases and tables

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
