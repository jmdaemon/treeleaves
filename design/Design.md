# Design

Design document and rationale for the Treeleaves database.

## Table of Contents

- [Mission Statement](#mission-statement)
- [Mission Objective](#mission-objective)
- [Terminology](#terminology)

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

Users find folders useful in that some folders/directories are frequently navigated to
during a session. Folders can also contain thumbnails representative of the contents stored inside.

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
    - Photos
    - Audio
    - Songs
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

- Page Count

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
- File Hash

- File Date

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

## Preliminary Table List

The preliminary table list is obtained from grouping the various fields in the preliminary field list.

- Files
- Sources
- Hashes

- File Timestamps

- Images
- Photos

- Videos

- Songs
- Audio

- Books

- Frequency
- Folders

## Final Table List

| Name            | Type       | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
|-----------------|------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Files           | Data       | The abstract concept of a file written and stored to disk. Files contain metadata that is useful for various applications to take advantage of, in order to provide better services. The file metadata is used to provide relevant features and services for specific file types.                                                                                                                                                                              |
| Sources         | Linking    | The origin of a file. Many websites contain files, and some websites are purposefully built to allow users to search for files stored on their servers. Publicly stored files on a server have the added benefit that users can share links to such information instead of either reuploading or sharing the file directly. This results in a much less wasteful and more efficient transfer of data, which will be important in sharing data to other people. |
| Hashes          | Validation | Various hash sums for given files. Hash sums calculated on files have the added benefit that they are unique and replicable given the same file. Hash sums allow us to make quick file comparisons, and also reinforce security of files. In this way, users can benefit from being able to search files using pre-calculated hash sums of files.                                                                                                              |
| Frequency       | Data       | The access frequency of files. Users may find themselves frequently searching for the same files/folders over and over again. Frequency is another way to use dates to discriminate and easily find relevant information. Users can benefit from even faster search times by reusing cached search results to quickly navigate their data.                                                                                                                     |
| File Timestamps | Data       | The time a file was saved to disk, or was last updated. Datetimes are useful to us since we can find files by how recently they were accessed.                                                                                                                                                                                                                                                                                                                 |
| Images          | Subset     | Any digital picture stored on disk. Images can contain and represent many other things. Images have some characteristics that distinguish it from other files and are useful to track to make it easier for users to find later.                                                                                                                                                                                                                               |
| Photos          | Subset     | A digital picture taken from a camera. Photos are similar to images except they are specifically pictures of real-life things. Photos carry inherent metadata within them, most important being the date & time the photo was taken as well as the location. Photos are essentially human souvenirs of various past memories, so keeping track of these characteristics allows us to provide another search heuristic for users.                               |
| Videos          | Subset     | Videos are essentially stacked images. Videos are different to images in that they are typically larger, come in different file formats, and also have a play duration. Discriminating videos from images allows users to take advantage of their own knowledge to find their files.                                                                                                                                                                           |
| Audio           | Subset     | An audible, digital sound file. Audio files must have a duration, and typically a set frequency.                                                                                                                                                                                                                                                                                                                                                               |
| Songs           | Subset     | A specific type of audio file. Songs are made by humans, and songs are specific types of audio files. These files can contain more specific metadata than a normal audio file such as the artist name, track title, album title, and genre. Users often listen to songs multiple times so being able to easily filter and present songs will make it easier for users to listen to their favorite tunes.                                                       |
| Books           | Subset     | A digital document. Books contain lots of information about a variety of subjects, some are non-fiction, some are fiction. Some are guides and others are fantasy books. Book information must reflect the contents of the specified book, and should be accurate.                                                                                                                                                                                             |
| Folders         | Linking    | An on-disk directory or collection that contains various files. Users make folders to group together like things. These directories may be useful to frequent given that related information may be consolidated together within one central place. Repeatedly accessing the same folders can be used as another heuristic to make searching a breeze for the end user.                                                                                        |
