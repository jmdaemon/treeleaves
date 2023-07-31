# Database Design

Final revised design document for the Treeleaves database.

## Table of Contents

- [Final Table List](#final-table-list)
- [Tables with Associated Fields](#tables-with-associated-fields)
- [Table Level Integrity](#table-level-integrity)
- [Field Level Integrity](#field-level-integrity)

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

The tables have been split up and listed below to make them easier to view.

#### Main Tables

| Files     | Folders          | Timestamps         |
|-----------|------------------|--------------------|
| File ID   | File ID          | File ID            |
| File Name | Folder View Type | Creation Date      |
| File Path |                  | Last Updated Date  |
| MIME Type |                  | Last Accessed Date |
| File Size |                  |                    |

#### Subset Tables

| Images             | Videos             | Audio           |
|--------------------|--------------------|-----------------|
| File ID            | File ID            | File ID         |
| Image Pixel Count  | Video Duration     | Audio Duration  |
| Image Width        | Video Width        | Audio Frequency |
| Image Height       | Video Height       |                 |
| Image Aspect Ratio | Video Aspect Ratio |                 |

#### Simple Feature Tables

| Hash Types     | File Hashes | Frequency      |
|----------------|-------------|----------------|
| Hash Type ID   | File ID     | File ID        |
| Hash Type Name | File Hash   | Times Accessed |

#### Advanced Feature Tables

##### Feature: Aliases

| Aliases    |
|------------|
| File ID    |
| Alias Name |

##### Feature: Sources

| Sources     | File Sources    |
|-------------|-----------------|
| Source ID   | File ID         |
| Source Name | File Source URL |

##### Feature: Sources Tags

| Source Tag Types   | Tag Type Template           | File Source Tags |
|--------------------|-----------------------------|------------------|
| Source Tag Type ID | `Tag Type Table Definition` | File ID          |
|                    |                             | Tag Type ID      |

##### Feature: Photos

| Photo Metadata | Location Metadata |
|----------------|-------------------|
| File ID        | File ID           |
| Date Taken     | Latitude          |
| Time Taken     | Longitude         |

###### Feature: Photo Albums

| Photo Albums |
|--------------|
| File ID      |
| Photo Order  |

##### Feature: Songs

| Songs      |
|------------|
| File ID    |
| Song Title |

###### Feature: Songs Artists

| Song Artists | Song Artist Names  |
|--------------|--------------------|
| File ID      | Artist ID          |
| Artist ID    | Artist First Name  |
|              | Artist Middle Name |
|              | Artist Last Name   |

###### Feature: Songs Bands

| Song Bands | Bands     |
|------------|-----------|
| File ID    | Band ID   |
| Band ID    | Band Name |

###### Feature: Songs Lyrics

| Song Lyrics |
|-------------|
| File ID     |
| Song Lyrics |

###### Feature: Songs Genres

| Song `Genre` Template | Music Genres     |
|-----------------------|------------------|
| File ID               | Music Genre ID   |
| Music Genre ID        | Music Genre Name |

###### Feature: Song Albums

| Albums     | Track `Name` Set Template |
|------------|---------------------------|
| Album ID   | File ID                   |
| Album Name | Track Order               |

##### Feature: Books

| Books           |
|-----------------|
| File ID         |
| Book Title      |
| Book Page Count |

###### Feature: Book Publishers

| Book Publishers       | Literary Publishers     |
|-----------------------|-------------------------|
| File ID               | Literary Publisher ID   |
| Literary Publisher ID | Literary Publisher Name |
| Book Publish Date     |                         |

###### Feature: Book Languages

| Book Languages   | Languages     |
|------------------|---------------|
| Book Language ID | Language ID   |
| File ID          | Language Code |
| Language ID      | Language Name |

###### Feature: Book Genres

| Book Genres       | Literary Genres     |
|-------------------|---------------------|
| File ID           | Literary Genre ID   |
| Literary Genre ID | Literary Genre Name |

###### Feature: Book Authors

| Book Authors   | Author Names       |
|----------------|--------------------|
| File ID        | Author ID          |
| Author Name ID | Author First Name  |
|                | Author Middle Name |
|                | Author Last Name   |

## Table Level Integrity

Each table is associated with a particular unique primary key. The first ID of every row is the designated primary key

For tables without a unique primary key, they are designated as subset tables.

### Sources

A file can have many sources. These sources will always have a url, and are identified by their source name.

Since there can be many sources, we create a table "Sources" that lists out all the possible sources available for our data.
This table contains:
- Source ID
- Source Name

Then we create our "File Sources":
- File ID
- File Source URL

There can be many multiple sources for a single file.

Instead of creating a new primary key to ensure uniqueness. We're going to instead store these sources in separate databases.

This design allows us to:
- Save the most amount of space on disk
- Modify & change "Sources" however we'd like. We are now able to remove sources, and change previous sources
    since "Source ID" is not, and will not be used anywhere else. In general, our database will be more resilient.

The "File Source URL" must always be filled, or else there is no "File Source" entry.

### Source Tags

Since a file can have many sources. And each source can have its own defined tag types.

For this we define a table "File Sources". The File Sources table contains fields:
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

### File Hashes

Every file can have a number of file hashes. Similarly to our "Sources" table, we define and aggregate all our possible
Hash types in a table "Hash Types" containing:
- Hash Type ID
- Hash Type Name

Next, we define table "File Hashes" that contains:
- File ID
- File Hash

In similar fashion, we opt not to create additional primary keys, and instead decide to store each file hash
in its own individual table/database. If we need to find a hash for a given file we'll just use the FILE ID,
and do one lookup per hash database.

### Albums

Songs may be part of an album. These songs often come with a designated song order.

### Album Track `Name` Set Template

We generate this table from Albums, thus spawning as many Album `Name` track sets.

This table contains:
- File ID
- Order

There aren't many fields because we need to conserve space.

## Song Genres

We have to generate this field too. Although we can probably make do with a single table with a unique ID,
since any song can have anywhere from between 1 to 5 genre tags, it's not going to be enough to cut it,
if we have to resize the ID field.

### Song `Genre` Template

Instead we'll adopt the same approach and generate a new Genre table from a template each time we need to.

This table will contain:
- File ID
- Music Genre ID

This will save us storage space, but lookups are likely to take longer, as we'll have to query all the tables
to find all the genres associated with a given song.

## Field Level Integrity
