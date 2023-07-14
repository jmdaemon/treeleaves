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

- Preliminary Field List: A characteristic of a particular subject
- Calculated Field List: A field that is calculated as a result from the preliminary field list

## Database Analysis

### Collection

- Files collections are aggregated to by way of being saved to various on disk directories.
- Files with "like" properties are grouped into folders

### Presentation

### Data Structure

#### Subjects

- Files
    - Images
    - Songs
    - Documents
- User

#### Characteristics

- Hash
- File Name, File Path, File Type File Size
- Alternate File Names
- Alias
- Source
- Tags

- Author
- Song Title
- Duration

- Page Count

### Preliminary Field List

- File Name Variant

- File Alias

- Local File Tags

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

- File Source Site
- File Source URL
- File Source Tags

- Video Duration

- Song Duration

- Document Page Count
