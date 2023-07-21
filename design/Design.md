# Design

Design document and rationale for the Treeleaves database.

## Table of Contents

- [Mission Statement](#mission-statement)
- [Mission Objective](#mission-objective)
- [Terminology](#terminology)
- [Database Analysis](#database-analysis)
    - [Current Database State](#current-database-state)
    - [Data Collection](#data-collection)
    - [Data Presentation](#data-presentation)
- [Data Structure](#data-structure)
    - [Subjects](#subjects)
    - [Characteristics](#characteristics)
    - [Preliminary Field List](#preliminary-field-list)
    - [Calculated Field List](#calculated-field-list)
    - [Preliminary Table List](#preliminary-table-list)
    - [Final Table List](#final-table-list)
    - [Tables with Associated Fields](#tables-with-associated-fields)
- [Table Level Integrity](#table-level-integrity)
- [Field Level Integrity](#field-level-integrity)

## Mission Statement

The purpose of the Treeleaves database is to maintain the data of files of users to
allow users to both easily and quickly find and access their personal on disk files.
A secondary objective of Treeleaves is to also make these files easily shareable
with others.

## Mission Objective

The objectives of the database are to:

- Maintain information on user files
- Maintain some information on file hash
- Maintain complete information file name
- Maintain some information on file name variants
- Maintain some information on file sources
- Maintain complete information on file tags
- Maintain complete information on file alias
- Maintain complete information on file type
- Maintain complete information on file mime type
- Maintain complete information on file size

- Keep track of duplicate files
- Keep track of current file paths

## Terminology

- Data: The values and records present in the database.
- Information: Data that is processed to be useful or have meaning.
- Subjects: Nouns, specifically nouns that reference a person, place or thing.
- Characteristics: Adjectives, specifically descriptions of aspects of the subjects.

- Paper-based database: A file system that consists of physical forms of data sorted by some coding scheme.
- Human-based database: A biological database based on the memory of one or more entities in any given organization.

- Preliminary Field List: A characteristic of a particular subject
- Calculated Field List: A field that is calculated as a result from the preliminary field list

## Database Analysis

## Current Database State

The database format as it stands currently is essentially a paper-based database consisting mostly
of some organized sorted files in folders.

The database is also a human-based database as the locations of where specific files are found, is only
known by the owner of those files.

### Data Collection

- Files collections are aggregated to by way of being saved to various on disk directories.
- Files with "like" properties are grouped into folders

### Data Presentation

#### Folders

Folders are just a collection or encoding scheme used to organize and sort files.

Folders are useful in being able to group like-objects together. Folders often works similar to informal sets
containing

Users find folders useful in that some folders/directories are frequently navigated to
during a session. Folders can also contain thumbnails representative of the contents stored inside.

Folders can present files in a variety of views. For example, folders can present files in
an icon view that shows the files listed side by side with big thumbnails representing the file contents.

Folders can also present files in a table format with specific columns tailored to files.

#### Images

Images are typically looked up with a file manager. It often takes a lot of time to load image thumbnails,
and there are a lot of files to go through before the user finds the desired image.

On some image boorus, it can be a lot quicker to find images due to the images being tagged very well, indexed,
and easy to search.

#### Photos

Photos are similar to images with a few exceptions. Photos are images taken purely with a camera.
These photos contain metadata such as
- GPS Location metadata
- Camera settings
- Device & hardware information
- Lens information
- Image size
- Pixel dimensions
- Date & time
- The app used to take the picture

#### Songs

Users also often have lots of music they like to listen to.

These songs are made by certain artists, have track titles, are part of an album, are made in a specific year, often as part of
an associated music genre. Many songs also contain lyrics and track numbers specifying their order. These are typically specified
as music tags.

An example of an application that manages such metadata is [audacity](https://manual.audacityteam.org/man/metadata_editor.html).

Songs also come in uncompressed formats that capture studio level sound quality. These are bigger than compressed files,
but offer the best sound.

#### Audio

Some users also have general audio sounds or samples. These don't contain as much metadata, but they do still have an audio frequency and duration.

#### Books

On some book searching websites, books can be searched by title, author(s), publisher, year, language or file type.

Users often have lots of books saved to their personal disks, and it can be a hassle to remember where a book is,
if the user is not constantly trifling through their library.

Users also are able to buy and/or download books online from a variety of web sites and sources.

#### Alias

An alias is a shorthand name for something else. This is typically a reference to another item or thing
of the same type. Shorthands are useful because one can remember easier to recall names than some specific
characteristics of an item. Aliases centralized in a single location allows you to group together like-objects
together, making them more easily found.

Aliases, emotes and alternative file names are also pretty similar in function and in name.

### Data Structure

#### Subjects

- Files
    - Images
    - Photos
    - Audio
    - Songs
    - Documents/Books
    - Videos
    - Comics
- File metadata
- Folders
- Aliases
- User

#### Characteristics

- Hash
- File Name, File Path, File Type File Size
- Source
- Tags

- Alternate File Names
- Alias
- Emote

- Creation Date
- Updated Date
- File Lifespan Dates / File Timestamps

- Times Accessed

- Audio Frequency

- Artist
- Album Name
- Track Title
- Track Number
- Genre

- Author
- Song Title
- Duration
- Genre

- Author
- Publisher
- Publish Date
- Page Count

- File Folder View Type

### Preliminary Field List

1. Determine and list all the characteristics of your subjects
2. Refine the field list by specifying more specific names, and pruning duplicates
3. Ensure the list of characteristics list are characteristics by definition

- File Name Variant

- File Alias

- Local File Tags
- File Tag Aliases

- Song Artist
- Song Album Name
- Song Track Title
- Song Track Number
- Song Genre
- Song Lyrics

- Book Author or Authors
- Book Genre
- Book Publisher
- Book Publish Date (Year)
- Book Language

### Calculated Field List

- File Name
- File Path
- File MIME Type
- File Size

- File Hash Type
- File Hash

- File Creation Date
- File Last Updated Date
- File Last Accessed Date

- Number of Times Accessed (Frequency)
- File Folder Directory

- Image Pixels
- Image Width
- Image Height

- Photo Date

- File Source Site
- File Source URL
- File Source Tags

- Video Width
- Video Height
- Video Duration

- Audio Frequency
- Song Duration

- Document Page Count

- Folder View Type

### Preliminary Table List

The preliminary table list is obtained from grouping the various fields in the preliminary field list.

- Files
- Sources
- Tags
- Hashes
- Timestamps
- Frequency

- File Aliases*

- Folders

- Images
- Photos

- Videos

- Songs
- Audio

- Books

- \*Aliases could technically be a part of a group of alternative names.
    As such it is hard to include these into the current table list since there could be many alias groups/tables.
    For now we will ignore these.

### Final Table List

| Name       | Type | Description                                                                                                                                                                                                                                                                                                                                                                                                         |
|------------|------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Files      | Data | A digital storage medium. Files contain a common set of traits. Files are good at storing information but make for poor databases. Centralizing the files within this database lets us keep track of our files' contents, and where our files are.                                                                                                                                                                  |
| Sources    | Data | The origin of a file. Many websites share files with some purpose-built to allow users to share similar content. Files stored on a server have the added benefit that users can simply share direct url links to such information instead of reuploading the file. This is much less wasteful and allows files to be shareable to more people.                                                                      |
| Hashes     | Data | Hash sums for given files. File hash sums are unique, replicable, and static. Hash sums allow us to make quick file comparisons, and also reinforce security of files if they dont match. Users can benefit from being able to search files quicker using pre-calculated hash sums of files.                                                                                                                        |
| Frequency  | Data | How frequently a file is accessed. Users often find themselves frequently searching for the same files & folders. We can deliver more relevant information for a given user session quicker, by caching frequently search results for recall later.                                                                                                                                                                 |
| Timestamps | Data | The time a file was saved to disk, or was last updated. Datetimes are useful to us since we can find files by how recently they were updated, or created.                                                                                                                                                                                                                                                           |
| Images     | Data | Any digital picture stored on disk. Images convey many meanings that aren't stored in the file itself. We need to track this information to provide better searches.                                                                                                                                                                                                                                                |
| Photos     | Data | An image from a camera of a real-life object. Photos have important metadata such as the date, time, and location the photo was taken. Photos are mini human souvenirs of past memories which hold significant meaning to people. Finding these photos again lets users relive their lives.                                                                                                                         |
| Videos     | Data | A collection of moving images. Videos are essentially bigger images, with a runtime duration. Due to their size, videos are often more valuable. Users won't want to carry too many on-disk lest their storage becomes full. If there are many file entries or videos, it becomes hard to distinguish which was which without watching the video itself.                                                            |
| Audio      | Data | An audible sound file. Audio files have a duration, and set frequency. Sound files are hard to preview since they don't often come with a thumbnail or descriptive name. As such sound files are harder to find in folders. Maintaining information that can hint at what the sound is, lets users navigate their folders with less guesswork.                                                                      |
| Songs      | Data | Vocal music intended to be listened to. Songs are audio files that contain more data about themselves than normal sound files, such as, artist name, track title, album title, and genre. Users that save songs to disk, listen to them repeatedly enabling users to focus on what song they want to listen to, rather than where it can be found.                                                                  |
| Books      | Data | A digital document. Books are archives of information, written for a variety of subjects, and by many different people of different times and publishers. Unlike a library however, this information can get lost in a sea of folders. Users will always have digital books to maintain. Knowing where to find a book is key to maintaining a well organized library, allowing users more time to focus on reading. |
| Folders    | Data | A directory or collection containing various files and/or folders. Folders are used by users to group together like-things. Directories often consolidate a lot of information in a central place. Repeatedly accessing the same folders can be cached and used as a tool by the user to quickly navigate his or her file system at ease.                                                                           |

### Tables with Associated Fields

| Files     | Sources     | File Sources    | Source Tag Types   | Tag Type Template           | File Source Tags | Hash Types     | File Hashes  | Frequency      | Timestamps         | Images             | Photos   | Photo Metadata    | Location Metadata    | Videos             | Audio           | Songs      | Song Bands | Bands     | Song Artists | Artists            | Song Tracks       | Song Lyrics | Song Genres    | Music Genres     | Albums     | Books           | Book Languages   | Languages     | Book Publishers       | Literary Publishers     | Book Genres       | Literary Genres     | Book Authors   | Authors            | Folders          |
|-----------|-------------|-----------------|--------------------|-----------------------------|------------------|----------------|--------------|----------------|--------------------|--------------------|----------|-------------------|----------------------|--------------------|-----------------|------------|------------|-----------|--------------|--------------------|-------------------|-------------|----------------|------------------|------------|-----------------|------------------|---------------|-----------------------|-------------------------|-------------------|---------------------|----------------|--------------------|------------------|
| File ID   | Source ID   | File Source ID  | Source Tag Type ID | `Tag Type Table Definition` | File Source ID   | Hash Type ID   | File Hash ID | File ID        | File ID            | File ID            | Photo ID | Photo Metadata ID | Location Metadata ID | File ID            | File ID         | File ID    | File ID    | Band ID   | File ID      | Artist ID          | Song Track ID     | File ID     | Song Genre ID  | Music Genre ID   | Album ID   | File ID         | Book Language ID | Language ID   | File ID               | Literary Publisher ID   | File ID           | Literary Genre ID   | File ID        | Author ID          | File ID          |
| File Name | Source Name | Source ID       |                    |                             | Source ID        | Hash Type Name | Hash Type ID | Times Accessed | Creation Date      | Image Pixel Count  | Image ID | File ID           | File ID              | Video Duration     | Audio Duration  | Song Title | Band ID    | Band Name | Artist ID    | Artist First Name  | Album ID          | Song Lyrics | File ID        | Music Genre Name | Album Name | Book Title      | File ID          | Language Code | Literary Publisher ID | Literary Publisher Name | Literary Genre ID | Literary Genre Name | Author Name ID | Author First Name  | Folder View Type |
| File Path |             | File ID         |                    |                             | Tag Type ID      |                | File ID      |                | Last Updated Date  | Image Width        |          | Date Taken        | Latitude             | Video Width        | Audio Frequency |            |            |           |              | Artist Middle Name | File ID           |             | Music Genre ID |                  |            | Book Page Count | Language ID      | Language Name | Book Publish Date     |                         |                   |                     |                | Author Middle Name |                  |
| MIME Type |             | File Source URL |                    |                             |                  |                | File Hash    |                | Last Accessed Date | Image Height       |          | Time Taken        | Longitude            | Video Height       |                 |            |            |           |              | Artist Last Name   | Song Track Number |             |                |                  |            |                 |                  |               |                       |                         |                   |                     |                | Author Last Name   |                  |
| File Size |             |                 |                    |                             |                  |                |              |                |                    | Image Aspect Ratio |          |                   |                      | Video Aspect Ratio |                 |            |            |           |              |                    |                   |             |                |                  |            |                 |                  |               |                       |                         |                   |                     |                |                    |                  |
|           |             |                 |                    |                             |                  |                |              |                |                    |                    |          |                   |                      |                    |                 |            |            |           |              |                    |                   |             |                |                  |            |                 |                  |               |                       |                         |                   |                     |                |                    |                  |
|           |             |                 |                    |                             |                  |                |              |                |                    |                    |          |                   |                      |                    |                 |            |            |           |              |                    |                   |             |                |                  |            |                 |                  |               |                       |                         |                   |                     |                |                    |                  |

## Table Level Integrity

Each table is associated with a particular primary key.

This primary key is always unique. For tables without an artificial candidate key, they are
considered to be subset tables.

### Sources

A file can have many sources. These sources will always have a url, and are identified by their source name.

Since there can be many sources, we create a table "Sources" that lists out all the possible sources available for our data.
This table contains:
- Source ID
- Source Name

Then we create our "File Sources":
- File Source ID
- Source ID
- File ID
- File Source URL

There can be multiple sources for a single file, so we must create a new artificial candidate key "File Source ID", to ensure every entry is unique.

The "File Source URL" must always be filled, or else there is no "File Source" entry.

### Source Tags

Since a file can have many sources. And each source can have its own defined tag types.

For this we define a table "File Sources". The File Sources table contains fields:
- File Source ID
- Source ID
- File ID
- File Source URL

Every file source can contain any number of tag types.

We define a table "Source Tag Types" to represent any general tag type for any source. This table contains:
- Source Tag Type ID

The "Source Tag Type ID" for any source tag represents any table that we use to represent any arbitrary tag defined by the source.

We're going to have any arbitrary number of tables that correspond to any number of tag definitions.

As such we define a generate template "Tag Types Template" that represents the generic tag definition.

Lastly we define a "File Source Tags" table that aggregates all these tag types together. The table contains fields:
- File Source ID
- Tag Type ID

## File Hashes

Every file can have a number of file hashes. Similarly to our "Sources" table, we define and aggregate all our possible
Hash types in a table "Hash Types" containing:
- Hash Type ID
- Hash Type Name

Next, we define table "File Hashes" that contains:
- File Hash ID
- Hash Type ID
- File ID
- File Hash

Since there could be multiple hashes for a single file, we define artificial candidate key "File Hash ID".

### Primary Keys

The first ID of every row is the designated primary key

## Field Level Integrity
