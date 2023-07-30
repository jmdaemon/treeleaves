# Table Relationships

All the table relationships for our database.

The table relationships are modeled via Entity-Relationship models

The tables are read left-to-right and top-to-bottom.

## Table of Contents

## Terminology

- `None`: Denotes no table relationship
- `1:1` : Denotes a one-to-one relationship
- `1:N` : Denotes a one-to-many relationship
- `M:N` : Denotes a many-to-many relationship

A pair of tables bears the following relationship if the two have the following relations.

- `1:1 + 1:1 = 1:1`
- `1:N + 1:1 = 1:N`
- `1:N + 1:N = M:N`

## Tables

The main table of interest is the `Files` table.

This table will contain all the main central data that we will base all other table relationships from.

We will separate our main tables of interest and additional tables to make it easier to reason about their relationships,
as well as being easier to manage.

We will split our tables into the following types:
- Main Tables
- Subset Tables
- Simple Feature Tables
- Advanced Feature Tables

### Main Tables

More specifically, our main database tables are:
- Files
- Folders
- Timestamps

Relationships:

| Table      | Files | Folders | Timestamps |
|------------|-------|---------|------------|
| Files      |       | 1:1     | 1:1        |
| Folders    | 1:N   |         | 1:1        |
| Timestamps | 1:1   | 1:1     |            |

### Subset Tables

Our subset table are:
- Images
- Audio
- Video

Relationships:

| Table  | Files | Images | Audio | Video |
|--------|-------|--------|-------|-------|
| Files  |       | 1:1    | 1:1   | 1:1   |
| Images | 1:1   |        |       |       |
| Audio  | 1:1   |        |       |       |
| Video  | 1:1   |        |       |       |

### Simple Feature Tables

Simple feature tables are like subset tables except they are not concerned
with the type of subject file, but are general features that apply to all files.

These simple feature tables may also employ the use of one or more additional tables that
reduce data duplication between records.

Our simple feature tables are:
- Hashes:
    - Hash Types
    - File Hashes
- Frequency

Relationships:

| Table       | Files | Hash Types | File Hashes | Frequency |
|-------------|-------|------------|-------------|-----------|
| Files       |       | 1:1        | 1:N         | 1:1       |
| Hash Types  | 1:N   |            | 1:N         |           |
| File Hashes | 1:1   |            |             |           |
| Frequency   | 1:1   |            |             |           |

### Advanced Feature Tables

Advanced feature tables are subset tables that require significantly more
tables or require table generation to be employed.

These tables may be resource intensive, are optional, and often highly
specific or tailored to a single task. For that reason these table relationships are looked
at more closely.

For this reason, we split them up into different sets of features

#### Sources

Our source tables are:
- Sources:
    - Sources
    - File Sources
- Source Tags:
    - Source Tag Types
    - Tag Type Template
    - File Source Tags

Note that these tables aren't necessarily normal. Some aren't directly related to `Files`.

##### File Sources

`Sources` is a standalone defined table.

Relationships:

| Table        | Files | Sources | File Sources |
|--------------|-------|---------|--------------|
| Files        |       |         | 1:N          |
| Sources      |       |         | 1:N          |
| File Sources | 1:1   | 1:1     |              |

##### File Source Tags

A source can have a number of tag types. A tag type is a table.

A file source can have a number of tags.

Relationships:

For the source tag types:

| Table             | Sources | Source Tag Types | Tag Type Template |
|-------------------|---------|------------------|-------------------|
| Sources           |         | 1:N              |                   |
| Source Tag Types  | 1:1     |                  | 1:N               |
| Tag Type Template | 1:1     | 1:1              |                   |

And for how these source tags relate to files:

| Table            | File Sources | File Source Tags |
|------------------|--------------|------------------|
| File Sources     |              | 1:N              |
| File Source Tags | 1:1          |                  |

#### Photos

A file can be an image. An image can be a photo.
We have the following tables for Photos:
- Photo Metadata
- Location Metadata

Relationships:

| Tables            | Files | Photo Metadata | Location Metadata |
|-------------------|-------|----------------|-------------------|
| Files             |       | 1:1            | 1:1               |
| Photo Metadata    | 1:1   |                |                   |
| Location Metadata | 1:1   |                |                   |

#### Songs

A file can be an audio file. An audio file can be a song.

Songs can be made by either artists or bands.

##### Song Artists

##### Song Bands

##### Music Genres



### Standalone Tables

These are tables that are completely independent from other tables.
They do not rely on data from any other table for any of their fields.

Table Name          |
--------------------|
Files               |
Sources             |
Hash Types          |
Bands               |
Music Genres        |
Albums              |
Languages           |
Literary Publishers |
Literary Genres     |
Author Names        |

## Subset Tables

These are tables that are subset tables of the `Standalone` table types.

Table Name              | Files |

- File Sources          | 1:N   |
- File Source Tags      | 1:N   |
- Frequency             | 1:1   |
- Timestamps            | 1:1   |

- Images                | 1:1   |
- Photo Metadata        | 1:1   |
- Location Metadata     | 1:1   |
- Videos                | 1:1   |

- Audio                 | 1:1   |
- Songs                 | 1:1   |
- Songs Bands           | 1:N   |
- Songs Artists         | 1:N   |

- Song Artist Names     | None  |

Generated Subset Table  | Parent Table  | Relationship
Song Genre Template     | Files         | 1:N

## Relationships

Table Name | Files 
|----------|
Files

Sources
File Sources
Source Tag Types
Tag Type Template
File Source Tags

Hash Types
File Hashes

Frequency
Timestamps
Images
Videos
Audio

Photo Metadata
Location Metadata

Songs
Song Bands
Bands

Song Artists
Song Artist Names

Song Lyrics
Song `Genre` Template

Music Genres

Albums
Track `Name` Set Template

Books
Book Languages
Languages
Book Publishers
Literary Publishers
Book Genres Literary Genres
Book Authors
Author Names

Folders
