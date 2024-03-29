# Database Field Specifications

For the database field specifications, we will ignore the other tables other than the main one for now.

## Table of Contents

- [Field Classes](#field-classes)
    - [Primary Keys](#primary-keys)
    - [Date Timestamps](#date-timestamps)
    - [Dimensions](#dimensions)
    - [Titles](#titles)
    - [Names](#names)
- [Field Specifications](#field-specifications)

## Terminology

- `inherit * from [class]` Means that the field inherits the rest of all its missing fields specifications from some class.

## Field Classes

### Primary Keys

For primary keys:
- If `Label` is not given, it is the same as the field name.
- There are commas in-between every set of 3 digits for the Input Mask & Display Format.
- Size # refers to how many digits the primary key ID will hold
- All IDs inherit from `Primary Key`

Size # also expands to the following. Given Size(8):

```
Data Type           : Numeric
Length              : 8
Input Mask          : ##_###_###
Display Format      : ##,###,###
```

### Date Timestamps

### Dimensions

### Titles

### Names

## Field Specifications

### File ID
**General Elements:**

Field Name          : File ID
Parent Table        : Files
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

**Physical Elements:**

Size(8);

**Logical Elements:**

Range of Values     : 1-99,999,999

### Source ID

**General Elements:**

Field Name          : Source ID
Parent Table        : Sources
Shared By           :   File Sources,
                        File Source Tags,
Description         :
    A unique whole number used to identify various sources from where files can originate from.
    It is assigned to a source when it is first added to the database, and remains the same
    throughout the lifetime of the program. It is rarely changed or removed.

**Physical Elements:**

Size(4);

**Logical Elements:**

Range of Values     : 1-9999

### Source Tag Type ID

Field Name          : Source Tag Type ID
Parent Table        : Source Tag Types
Shared By           :   Tag Type Template
                        File Source Tags,
Description         :
    A unique whole number used to identify the kinds of tags present in a source.
    For sources that aren't local, these ids are machine generated, and not intended to be changed or modified.
    The Source Tag Type ID determines what type of tag it is, so appropriate action can be taken to
    query more tables and determine all the source metadata for a file. The intent is to create
    a general system that can be used to model the tag systems in use by various websites and other sources.

**Physical Elements:**

Size(6);

**Logical Elements:**

Range of Values     : 1-999,999

### Hash Type ID

Field Name          : Hash Type ID
Parent Table        : Hash Types
Shared By           :
Description         :
    A unique whole number used to identify types of file hashing algorithms.

**Physical Elements:**

Size(4);

**Logical Elements:**

Range of Values     : 1-9999

### Band ID

Field Name          : Band ID
Parent Table        : Bands
Description         :
    A unique whole number used to identify different band types. These
    correspond to the bands of a user's song collection.

**Physical Elements:**

Size(3);

**Logical Elements:**

Range of Values     : 1-999

## Artist ID

Field Name          : Artist ID
Parent Table        : Song Artists
Description         :
    A unique whole number to identify different song artists/singers.

**Physical Elements:**

Size(4);

**Logical Elements:**

Range of Values     : 1-9999

## Book Language ID

Field Name          : Book Language ID
Parent Table        : Book Languages
Description         :
    A unique whole number to identify different languages for a book.
    Most books will only be written in one language, with some being written for two or more.

**Physical Elements:**

Size(4);

**Logical Elements:**

Range of Values     : 1-9999

## Language ID

Field Name          : Language ID
Parent Table        : Languages
Description         :
    A unique whole number to identify different languages.
    The MAX_ID number chosen is based on the ISO-639-1 standard for language codes.

**Physical Elements:**

Size(3);

**Logical Elements:**

Range of Values     : 1-999

## Literary Publishers ID

Field Name          : Literary Publishers ID
Parent Table        : Literary Publishers
Description         :
    A unique whole number that identifies different Book Publishers.

**Physical Elements:**

Size(4);

**Logical Elements:**

Range of Values     : 1-9999

## Literary Genres ID

Field Name          : Literary Genres ID
Parent Table        : Literary Genres
Description         :
    A unique whole number that identifies different Book Genre types.

**Physical Elements:**

Size(4);

**Logical Elements:**

Range of Values     : 1-9999

## Author Names ID

Field Name          : Author Names ID
Parent Table        : Author Names
Description         :
    A unique whole number that identifies different book Authors.

**Physical Elements:**

Size(4);

**Logical Elements:**

Range of Values     : 1-9999
