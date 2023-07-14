# Design

Design document and rationale for the Treeleaves database.

## Table of Contents

- [Mission Statement](#mission-statement)
- [Mission Objective](#mission-objective)
- [Terminology](#terminology)

## Mission Statement

The purpose of the Treeleaves database is to maintain the data of files of users to
allow users to both easily and quickly find and access their personal on disk files.

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

#### Images

Images are typically looked up with a file manager. It often takes a lot of time to load image thumbnails,
and there are a lot of files to go through before the user finds the desired image.

On some image boorus, it can be a lot quicker to find images due to the images being tagged very well, indexed,
and easy to search.

#### Audio

Users also often have lots of music they like to listen to.

These songs are made by certain artists, have track titles, are part of an album, are made in a specific year, often as part of
an associated music genre. Many songs also contain lyrics and track numbers specifying their order. These are typically specified
as music tags.

An example of an application that manages such metadata is [audacity](https://manual.audacityteam.org/man/metadata_editor.html).

Some users also have general audio sounds or samples. These don't contain any such metadata.

#### Books

On some book searching websites, books can be searched by title, author(s), publisher, year, language or file type.

Users often have lots of books saved to their personal disks, and it can be a hassle to remember where a book is,
if the user is not constantly trifling through their library.

Users also are able to buy and/or download books online from a variety of web sites and sources.

### Data Structure

#### Subjects

- Files
    - Images
    - Photo
    - Audio/Songs
    - Documents/Books
    - Videos
    - Comics
- File metadata
- Folders
- User

#### Characteristics

- Hash
- File Name, File Path, File Type File Size
- Alternate File Names
- Alias
- Source
- Tags

- Creation Date
- Updated Date

- Author
- Song Title
- Duration

- Page Count

### Preliminary Field List

1. Determine and list all the characteristics of your subjects
2. Refine the field list by specifying more specific names, and pruning duplicates
3. Ensure the list of characteristics list are characteristics by definition

- File Name Variant

- File Alias

- Local File Tags
- File Tag Aliases

- Song Author
- Song Title
- Song Lyrics

- Document Author

### Calculated Field List

- File Name
- File Path
- File MIME Type
- File Size
- File Hash

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

- Song Duration

- Document Page Count
