# Table Relationships

All the table relationships for our database.

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
Photo Metadata
Location Metadata
Videos
Audio
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
