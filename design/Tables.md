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

Our main table is the `Files` table. This table will contain all the
main central data that we will base all other table relationships from.

We will tackle the relationships of these types of tables in isolation both to make it
easier to reason about certain properties of these tables, as well as being easier to manage.

### Main Tables

More specifically, our main database tables are:
- Files
- Folders
- Timestamps

#### Relationships

| Table      | Files | Folders | Timestamps |
|------------|-------|---------|------------|
| Files      |       | 1:1     | 1:1        |
| Folders    | 1:N   |         | 1:1        |
| Timestamps | 1:1   | 1:1     |            |

### Main Subset Tables

We also have a set of subset tables that are also important to our main tables.

These tables are:

- Hashes
- Frequency
- Images
- Audio
- Video

### Feature Tables

Feature tables are add-on tables/standalone databases that add more enhanced or specific
metadata to our main tables and our main subset tables.

Our feature tables for the database are:

- Sources
- Photos
- Songs
- Books

These feature tables will be designed and addressed separately from the concerns of our main database tables


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
