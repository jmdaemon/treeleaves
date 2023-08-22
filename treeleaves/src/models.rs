use super::schemas::*;

use diesel::prelude::*;
use chrono::{DateTime, Utc, NaiveDate, NaiveTime};
use url::Url;

use std::{path::PathBuf, time::Duration};

//pub type FileID = u64;
pub type FileID = i64;
pub type DateTimestamp = DateTime<Utc>;

//
// Main Tables
//

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = mime_types)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct MIMEType {
    pub id: i32,
    pub mime_type: String,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = files)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct File {
    pub id: FileID,
    pub name: String,
    //pub path: PathBuf,
    pub path: String,
    //pub size: u64,
    pub size: i64,
    //pub mime_type_id: u32,
    pub mime_type_id: i32,
}

pub enum FolderViewType {
    Icon,
    Table,
}

pub struct Folder {
    pub file_id: FileID,
    pub view_type: i32,
}

pub struct Timestamp {
    pub file_id: FileID,
    pub created_at: DateTimestamp,
    pub updated_at: DateTimestamp,
    pub accessed_at: DateTimestamp,
}

//
// Subset Tables
//
pub struct Image {
    pub file_id: FileID,
    pub pixel_count: u16,
    pub width: u16,
    pub height: u16,
    pub aspect_ratio: String,
}

pub struct Video {
    pub file_id: FileID,
    pub duration: Duration,
    pub width: u16,
    pub height: u16,
    pub aspect_ratio: String,
}

pub struct Audio {
    pub file_id: FileID,
    pub duration: Duration,
    pub frequency: f32,
}

//
// Simple Feature Tables
//

pub struct HashType {
    pub id: u16,
    pub name: String,
}

/// The file hash data repesentation for ar given hash algorithm
/// NOTE: `File Hashes` is individually defined according to the following structure:
/// Structure:
/// - `hashes`
///     - `sha1.db`
///     - `sha256.db`
///     - `md5.db`
/// In every database, there will be a `File Hashes` table
/// So we don't need HashTypeID
pub struct FileHash {
    pub file_id: FileID,
    pub hash: String,
}

pub struct Frequency {
    pub file_id: FileID,
    pub times_accessed: u32,
}

//
// Advanced Feature Tables
//

// Aliases
pub struct Aliases {
    pub file_id: FileID,
    pub alias: String,
}

// Feature: Sources
// Sources
pub struct Source {
    pub id: u16,
    pub name: String,
}

pub struct FileSource {
    pub file_id: FileID,
    pub url: Url,
}

// Sources Tags
pub type SourceTagType = u64;

// Generic tag type table definition
pub struct TagType {
    pub id: u64,
}

pub struct FileSourceTags {
    pub file_id: FileID,
    pub tag_id: SourceTagType,
}

// Feature: Photos
pub struct PhotoMetadata {
    pub file_id: FileID,
    pub date_taken: NaiveDate,
    pub time_taken: NaiveTime,
}

pub struct LocationMetadata {
    pub file_id: FileID,
    pub lattitude: f64,
    pub longitude: f64,
}

// Feature: Photo Albums
pub type PhotoAlbumID = u64;

pub struct PhotoAlbum {
    pub id: PhotoAlbumID,
    pub name: String,
}

pub struct AlbumPhoto {
    pub file_id: FileID,
    pub order: u8,
}

// Feature: Songs

// Song Artists
pub type SongArtistID = u64;

pub struct SongArtistsName {
    pub id: SongArtistID,
    pub firstname: String,
    pub middlename: String,
    pub lastname: String,
}

pub struct SongArtist {
    pub file_id: FileID,
    pub artist: SongArtistID,
}

// Song Bands
pub type BandID = u64;

pub struct Band {
    pub id: BandID,
    pub name: String,
}

pub struct SongBand {
    pub file_id: FileID,
    pub band_id: BandID,
}

// Song Albums
pub type MusicAlbumID = u64;

pub struct MusicAlbum {
    pub id: MusicAlbumID,
    pub name: String,
}

/// An album has a number of tracks
/// NOTE:
/// There are two ways to model this relation:
/// 1. Standard Method: Create a linking table "Song Tracks" that contains fields:
///     - file_id
///     - album_id
///     - order
/// 2. Table Generation: Generate a table "Track: `name`" that contains fields:
///     - file_id
///     - order
/// NOTE: We'll have to do table generation anyways, so we'll stick with #2 for now
pub struct Track {
    pub file_id: FileID,
    pub order: u8,
}

// Song Lyrics
pub struct SongLyric {
    pub file_id: FileID,
    pub lyric: String,
}

// Song Genres
pub type MusicGenreID = u16;
pub struct MusicGenre {
    pub id: MusicGenreID,
    pub name: String,
}

// We generate a table for every file with the song genre.
//

/// Song albums are an add-on feature.
/// An album can contain many songs.
/// There are a few ways to model this relationship:
/// 1. Standard Method: Create a linking table `Song Genres` with fields:
///      - file_id
///      - music_genre_id
/// 2. Table Generation: Generate a table "Music Genre: `name`" with fields:
///      - file_id
/// NOTE: We will have a lot of duplicate data if we generate a table
///     since having two file_ids is bigger than two music_genre_ids
/// So we will stick with method #1
pub struct SongGenre {
    pub music_genre_id: MusicGenreID,
    pub file_id: FileID,
}

// Feature: Books
pub struct Book {
    pub file_id: FileID,
    pub title: String,
    pub pages: u16,
}

// Book Publications
pub struct BookPublication {
    pub file_id: FileID,
    pub publication_date: NaiveDate,
}

// Book Publishers
pub type PublisherID = u64;
pub struct LiteraryPublisher {
    pub id: PublisherID,
    pub name: String,
}

pub struct BookPublisher {
    pub file_id: FileID,
    pub publishers_id: PublisherID,
}

// Book Languages
pub type LanguageID = u16;

pub struct Language {
    pub id: LanguageID,
    pub code: String,
    pub name: String,
}

pub struct BookLanguage {
    pub file_id: FileID, 
    pub language_id: LanguageID,
}
// Book Genres
pub type LiteraryGenreID = u16;

pub struct LiteraryGenre {
    pub id: LiteraryGenreID,
    pub name: String,
}

pub struct BookGenre {
    pub file_id: FileID,
    pub literary_genre_id: LiteraryGenreID,
}

// Book Authors
pub type LiteraryAuthorID = u64;

pub struct LiteraryAuthorName {
    pub id: LiteraryAuthorID,
    pub firstname: String,
    pub middlename: String,
    pub lastname: String,
}

pub struct BookAuthors {
    pub file_id: FileID,
    pub author_id: LiteraryAuthorID,
}
