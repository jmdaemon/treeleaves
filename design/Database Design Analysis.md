# Database Design Analysis

Document draft for the initial database analysis and rationale for the Treeleaves database.

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
