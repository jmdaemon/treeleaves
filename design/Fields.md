# Database Field Specifications

## Primary Keys

### File ID
**General Elements:**

Field Name          : File ID
Parent Table        : Files
Label               : File ID
Shared By           :   File Sources,
                        File Source Tags,
                        File Hashes,
                        Frequency,
                        Timestamps,
                        Images,
                        Photos,
                        Photo Metadata,
                        Location Metadata
                        Videos,
                        Audio,
                        Songs,
                        Song Bands,
                        Song Artists,
                        Song Tracks
                        Song Lyrics,
                        Song Genres,
                        Books,
                        Book Languages,
                        Book Publishers,
                        Book Genres,
                        Book Authors,
                        Folders,

Description         :
    A unique whole number used to identify each file in a user's file system.
    It is assigned to the file when it is first added to the database and remains
    throughout the lifetime of the file in the file system.

inherit * from Primary Key

**Physical Elements:**

Data Type           : Numeric
Length              : 8
Input Mask          : ##_###_###
Display Format      : ##,###,###

inherit * from Primary Key

**Logical Elements:**

Range of Values     : 1-99,999,999

inherit * from Primary Key

### Source ID

**General Elements:**

Field Name          : Source ID
Parent Table        : Sources
Label               : Source ID
Shared By           :   File Sources,
                        File Source Tags,
Description         :
    A unique whole number used to identify various sources from where files can originate from.
    It is assigned to a source when it is first added to the database, and remains the same
    throughout the lifetime of the program. It is rarely changed or removed.

inherit * from Primary Key

**Physical Elements:**

Data Type           : Numeric
Length              : 5
Decimal Places      : 0
Input Mask          : ##_###
Display Format      : ##,###

inherit * from Primary Key

**Logical Elements:**

Range of Values     : 1-10,000

inherit * from Primary Key

