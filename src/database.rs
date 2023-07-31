use std::{path::PathBuf, time::Duration};

use chrono::{DateTime, Utc, NaiveDate, NaiveTime};

pub type FileID = u64;
pub type DateTimestamp = DateTime<Utc>;

// Main Tables
pub struct File {
    pub id: FileID,
    pub name: String,
    pub path: PathBuf,
    pub mime_type: String,
    pub size: i64,
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

// Subset Tables
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

// Simple Feature Tables
/*
pub enum HashType {
    SHA1,
    SHA256,
    MD5,
}
*/

pub struct HashType {
    pub id: u16,
    pub name: String,
}

pub struct FileHash {
    pub file_id: FileID,
    pub hash: String,
}

pub struct Frequency {
    pub file_id: FileID,
    pub times_accessed: u32,
}

// Advanced Feature Tables

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
pub struct TagType<T> {
    pub id: u64,
}

pub struct FileSourceTags {
    pub file_id: FileID,
    pub tag_id: SourceTagType,
}

// Feature: Photos
pub struct PhotoMetadata {
    pub file_id: FileID,
    //pub taken_at: DateTimestamp,
    pub date_taken: NaiveDate,
    pub time_taken: NaiveTime,
}

pub struct  LocationMetadata {
    pub file_id: FileID,
    pub lattitude: f64,
    pub longitude: f64,
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
pub type AlbumID = u64;

// TODO: This relationship needs work
// Is the relationship a many-to-many relationship?

// An album
pub struct Album {
    pub id: AlbumID,
    pub name: String,
}

// We want to have an album available without having to encode the album id
// So instead we generate a database of Albums with the Album Name, and link
// back to the files

// A song can be a track
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

// Song albums are an add-on feature.
// An album can contain many songs.
// There are a few ways to model this relationship:
// 1. Create a linking table SongGenres with:
//      - file_id
//      - music_genre_id

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

// Book Publishers
pub type PublisherID = u64;
pub struct LiteraryPublisher {
    pub id: PublisherID,
    pub name: String,
}

//pub struct BookPublications {
    //pub file_id: FileID,
    //pub publication_date: NaiveDate,
//}

// TODO: This will cause an issue if there are multiple publishers
pub struct BookPublisher {
    pub file_id: FileID,
    pub publishers_id: PublisherID,
    pub publication_date: NaiveDate,
}

// Book Languages
pub type LanguageID = u16;
pub struct Language {
    pub id: LanguageID,
    pub code: String,
    pub name: String,
}

// TODO: It's probably redundant to include an identifier for BookLanguage
pub struct BookLanguage {
    pub id: u64,
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
