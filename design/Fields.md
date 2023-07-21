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
Length              : 4
Decimal Places      : 0
Input Mask          : ####
Display Format      : ####

inherit * from Primary Key

**Logical Elements:**

Range of Values     : 1-9999

inherit * from Primary Key

### Source Tag Type ID

Field Name          : Source Tag Type ID
Parent Table        : Source Tag Types
Label               : Tag Type ID
Shared By           :   Tag Type Template
                        File Source Tags,
Description         :
    A unique whole number used to identify the kinds of tags present in a source.
    For sources that aren't local, these ids are machine generated, and not intended to be changed or modified.
    The Source Tag Type ID determines what type of tag it is, so appropriate action can be taken to
    query more tables and determine all the source metadata for a file. The intent is to create
    a general system that can be used to model the tag systems in use by various websites and other sources.

inherit * from Primary Key

**Physical Elements:**

Data Type           : Numeric
Length              : 6
Decimal Places      : 0
Input Mask          : #,###,###
Display Format      : #,###,###

inherit * from Primary Key

**Logical Elements:**

Range of Values     : 1-999,999

inherit * from Primary Key

### Hash Type ID

Field Name          : Hash Type ID
Parent Table        : Hash Types
Label               : Hash Type ID
Description         :
    A unique whole number used to identify types of file hashing algorithms.

inherit * from Primary Key

**Physical Elements:**

Data Type           : Numeric
Length              : 4
Decimal Places      : 0
Input Mask          : ####
Display Format      : ####

inherit * from Primary Key

**Logical Elements:**

Range of Values     : 1-9999

inherit * from Primary Key
