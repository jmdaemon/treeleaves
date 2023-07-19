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

Alias(es)           :
Specification Type  : [x] Unique [] Generic [] Replica
Source Specification:
Description         :
    A unique whole number used to identify each file in a user's file system.
    It is assigned to the file when it is first added to the database and remains
    throughout the lifetime of the file in the file system.

**Physical Elements:**

Data Type           : Numeric
Length              : 9
Decimal Places      : 0
Input Mask          : ###_###_###
Display Format      : ###,###,###

Character Support:
    [] Letters (A-Z)   [] Keyboard (. , / $ # %)
    [x] Numbers (0-9)   [] Special (© ® ™ Σ π)

**Logical Elements:**

Key Type:
    [] Non      [x] Primary
    [] Foreign  [] Alternate

Key Structure       : [x] Simple         [] Composite
Uniqueness          : [] Non-unique     [x] Unique
Null Support        : [] Nulls Allowed  [x] No Nulls
Values Entered By   : [] User           [x] System
Required Value      : [] No             [x] Yes
Default Value       :
Range of Values     : 1-100,000,000

Edit Rule:
    [] Enter Now, Edits Allowed
    [x] Enter Now, Edits Not Allowed
    [] Enter Later, Edits Allowed
    [] Enter Later, Edits Not Allowed
    [] Not Determined At This Time

Comparisons Allowed:
    [x] Same Field       [] All [x] eq    [] gt [] geq    [] neq    [] lt [] leq
    [] Other Fields     [] All [] eq    [] gt [] geq    [] neq    [] lt [] leq
    [x] Value Expression [] All [x] eq    [] gt [] geq    [] neq    [] lt [] leq

Operations Allowed:
    [] Same Field       [] All [] add    [] sub [] mult    [] div    [] concat
    [] Other Fields     [] All [] add    [] sub [] mult    [] div    [] concat
    [] Value Expression [] All [] add    [] sub [] mult    [] div    [] concat

### Source ID

**General Elements:**

Field Name          : Source ID
Parent Table        : Sources
Label               : Source ID
Shared By           :   File Sources,
                        File Source Tags,

Alias(es)           :
Specification Type  : [x] Unique [] Generic [] Replica
Source Specification:
Description         :
    A unique whole number used to identify sources of file origins. It is
    assigned to a source when it is first added to the database, and remains
    throughout the lifetime of the program. It is rarely changed or removed.

**Physical Elements:**

Data Type           : Numeric
Length              : 5
Decimal Places      : 0
Input Mask          : ##_###
Display Format      : ##,###

Character Support:
    [] Letters (A-Z)   [] Keyboard (. , / $ # %)
    [x] Numbers (0-9)   [] Special (© ® ™ Σ π)

**Logical Elements:**

Key Type:
    [] Non      [x] Primary
    [] Foreign  [] Alternate

Key Structure       : [x] Simple         [] Composite
Uniqueness          : [] Non-unique     [x] Unique
Null Support        : [] Nulls Allowed  [x] No Nulls
Values Entered By   : [] User           [x] System
Required Value      : [] No             [x] Yes
Default Value       :
Range of Values     : 1-10,000

inherit * from File ID
