# Table Relationships

All the table relationships for our database.

The table relationships are modeled via Entity-Relationship models

The tables are read left-to-right and top-to-bottom.

## Table of Contents

- [Terminology](#terminology)
- [Tables](#tables)
    - [Main Tables](#main-tables)
    - [Subset Tables](#subset-tables)
    - [Simple Feature Tables](#simple-feature-tables)
    - [Advanced Feature Tables](#advanced-feature-tables)

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

A song can be made by a number of song artists. A song artist has a full name.

Relationships:

| Table              | Files | Songs | Song Artists | Song Artists Names |
|--------------------|-------|-------|--------------|--------------------|
| Files              |       | 1:1   | 1:N          |                    |
| Songs              | 1:1   |       |              |                    |
| Song Artists       | 1:1   | 1:1   |              | 1:N                |
| Song Artists Names | 1:1   | 1:1   | 1:1          |                    |

##### Song Bands

A song can be made by a number of song bands. A band has a name.

Relationships:

| Table      | Files | Songs | Song Bands | Bands |
|------------|-------|-------|------------|-------|
| Files      |       | 1:1   | 1:N        |       |
| Songs      | 1:1   |       |            |       |
| Song Bands | 1:1   | 1:1   |            | 1:N   |
| Bands      | 1:1   | 1:1   | 1:1        |       |

##### Song Albums

A song album can contain many songs. A song has a track order.

Relationships:

| Table          | Files | Songs | Albums | Track Name Set  |
|----------------|-------|-------|--------|-----------------|
| Files          |       | 1:1   |        |                 |
| Songs          | 1:1   |       |        | 1:1             |
| Albums         | 1:N   | 1:N   |        | 1:N             |
| Track Name Set | 1:1   | 1:1   | 1:1    |                 |

##### Song Lyrics

A song can have one set of song lyrics.

Relationships:

| Table       | Files | Songs | Song Lyrics |
|-------------|-------|-------|-------------|
| Files       |       | 1:1   |             |
| Songs       | 1:1   |       | 1:1         |
| Song Lyrics | 1:1   | 1:1   |             |

##### Music Genres

A song can have a number of music genres.

Relationships:

| Table        | Files | Songs | Song Genres | Music Genres |
|--------------|-------|-------|-------------|--------------|
| Files        |       | 1:1   | 1:N         |              |
| Songs        | 1:1   |       | 1:N         |              |
| Song Genres  | 1:1   | 1:1   |             | 1:N          |
| Music Genres |       |       | 1:1         |              |

#### Books

A file can be a book. A book:
- Can have a number of literary publishers
- Can be written in a number of languages
- Can be classified by a number of genres
- Can be written by a number of authors

Relationships:

##### Book Publishers

| Table               | Files | Books | Book Publishers | Literary Publishers |
|---------------------|-------|-------|-----------------|---------------------|
| Files               |       | 1:1   | 1:N             |                     |
| Books               | 1:1   |       | 1:N             |                     |
| Book Publishers     | 1:1   | 1:1   |                 | 1:N                 |
| Literary Publishers |       |       | 1:1             |                     |

##### Book Languages

| Table          | Files | Books | Book Languages | Languages |
|----------------|-------|-------|----------------|-----------|
| Files          |       | 1:1   | 1:N            |           |
| Books          | 1:1   |       | 1:N            |           |
| Book Languages | 1:1   | 1:1   |                | 1:N       |
| Languages      |       |       | 1:1            |           |

##### Book Genres

| Table           | Files | Books | Book Genres | Literary Genres |
|-----------------|-------|-------|-------------|-----------------|
| Files           |       | 1:1   | 1:N         |                 |
| Books           | 1:1   |       | 1:N         |                 |
| Book Genres     | 1:1   | 1:1   |             | 1:N             |
| Literary Genres |       |       | 1:1         |                 |

##### Book Authors

| Table                 | Files | Books | Book Authors | Literary Author Names |
|-----------------------|-------|-------|--------------|-----------------------|
| Files                 |       | 1:1   | 1:N          |                       |
| Books                 | 1:1   |       | 1:N          |                       |
| Book Authors          | 1:1   | 1:1   |              | 1:N                   |
| Literary Author Names |       |       | 1:1          |                       |

- **Note:** The above tables for books all have the same table relationship structure between them.
    - Other tables also share this same structure, so we will give a name for it
