# Database Design Notes

General database design notes

## Table of Contents

- [Terminology](#terminology)
- [Tables](#tables)
- [Keys](#keys)
- [Fields](#fields)

## Terminology

- Data: The values and records present in the database.
- Information: Data that is processed to be useful or have meaning.
- Subjects: Nouns, specifically nouns that reference a person, place or thing.
- Characteristics: Adjectives, specifically descriptions of aspects of the subjects.
- Multipart/Multivalue Fields: A field with 2 or more data values.
- Calculated/Concatenated Field: A field that is processed from data.

## Tables

### Elements of an Ideal Table

- Represents a single subject
- Has a primary key
- Does not contain multipart or multivalue fields
- Does not contain calculated fields
- Does not contain unnecessary duplicate fields (except for related fields for subset tables)
- Contains a minimum amount of redundant data

## Keys

Keys are important for:
- Ensuring every record in a table is unique and precisely identified.
- Enforcing table-level and relationship level integrity
- Establishing table relationships

The main key types are:
- Candidate
- Primary
- Foreign
- Non-key

### Key Types

## Candidate

### Elements of a Candidate Key
- Can't be a multipart field
- Must contain unique values
- Cannot cause a breach of security or privacy rules
- Cannot be null or optional in whole or in part
- Uses the least amount of fields needed to establish uniqueness
- Its values must uniquely and exclusively identify each record in the table
- Its values must uniquely and exclusively identify the value of each field in the record
- Its value can only be modified in rare cases

### Artificial Candidate Keys

A candidate key that wasn't determined naturally from the table characteristics.

This key conforms to the elements of a candidate key and also becomes the official candidate key of the table.

### Alternate Keys

Alternate keys can alternatively identify particular records in a table.

The designation for an alternate key is "AK" or "CAK" for composite alternate keys composed of 2+ fields.

These keys will be used during the implementation phase of an RDBMS program.

## Primary

A primary key is a candidate key selected to:
- Exclusively identify the table throughout the database structure
- Establish relationships with other tables
- Uniquely identify a given record within a table and exclusively throughout the whole database

### Elements of a Primary Key

The primary key must conform to all the elements of a candidate key.

### Rules

- Each table must have only one primary key
- Each primary key within the database must be unique (except for subset tables)

## Foreign
## Non-key

## Fields

### Elements of the Ideal Field

- Represents a single distinct characteristic of the table subject
- Contains a single value
- Cannot be deconstructed into smaller components
- Does not contain a calculated/concatenated field
- Is unique within the entire database structure
- Retains majority of its characteristics when referenced in more than one table

### Field Level Integrity

#### Requirements
- The identity and purpose of a field is clear, and all of the tables referencing the field are properly identified
- Field definitions are consistent throughout the database
- The values of a field are consistent and valid
- The usage (i.e modifications, comparisons, operations) of the values in the field are clearly identified

### Field Specification Types

The main field specification types are:
- General Elements:
    - Field Name
    - Parent Table
    - Label
    - Specification Type
    - Source Specification
    - Shared By
    - Alias(es)
    - Description
- Physical Elements
    - Data Type
    - Length
    - Decimal Places
    - Character Support
    - Input Mask
    - Display Format
- Logical Elements
    - Key Type
    - Key Structure
    - Uniqueness
    - Null Support
    - Values Entered By
    - Required Value
    - Default Value
    - Range of Values
    - Edit Rule
    - Comparisons Allowed
    - Operations Allowed

### Guidelines for Composing Field Descriptions

- Use a statement that accurately identifies and clearly states its intended purpose
- Write a clear and succinct statement
- Refrain from restating/rephrasing the field name
- Refrain from using technical jargon, acronyms, or abbreviations
- Refrain from including implementation-specific information
- Do not make the description dependent on another field's description
- Do not use examples
