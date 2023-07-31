# Database Overview

A quick overview of the most important features of the database.

This document also provides an overview of the files to be found in the database implementation.

## Table of Contents

- [Features](#features)
- [Hashes](#hashes)
- [Sources](#sources)
- [Sources Tags](#sources-tags)
- [File Type Metadata](#file-type-metadata)
- [Advanced File Type Metadata](#advanced-file-type-metadata)

## Features

- Hashes
- Timestamps
- Frequency
- Sources
- Sources Tags
- File Type Metadata
    - Images
    - Audio
    - Videos
- Advanced File Type Metadata
    - Photos
    - Songs
    - Books

## Hashes

Each file can have a file hash. There are many hash types.

To model this accurately we create the following directory structure:
- `hashes`
    - `sha1.db`
    - `sha256.db`
    - `md5.db`
Each one of these databases contains a table of file hashes for a given directory.

The database names are the given hash type algorithm names in lowercase.

## Sources

Each file can have a file source. There are many file sources.

We model this with the following hierarchy:
- `sources`
    - `source1`
        - `sources.db`
    - `source2`
        - `sources.db`
    - `source3`
        - `sources.db`

Every database contains a table of file sources for files.

## Sources Tags

Each file can have a source. A source has any number of tag definitions in the form of tables. These make up the source database's schema.

One tag definition can reference one file. These tag definitions are aggregated into a single view table.

This is modeled in the following hierarchy:
- `sources`
    - `source1`
        - `sources.db`
        - `tags.db`
    - `source2`
        - `sources.db`
        - `tags.db`

## File Type Metadata

Each file has an mime-type. We define some general types based on the mime-type of a file.

This general type is a subset of a file, and offers more specific metadata.

We have the hierarchy:
- `types`
    - `images`
    - `videos`
    - `audio`

## Advanced File Type Metadata

Although files can have the same mime-type, some files can be more specific.
This specific metadata is encapsulated in the advanced file type classifications.

These are created as `addons` which offer more specific information for our data.

- `addons`
    - `songs`
    - `albums`
    - `books`