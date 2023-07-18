# Database Fields

## Field Specification Types

The field specifications for a general class of field.

### Field Specification Type: Primary Key Spec Field

**General Elements:**

Field Name          : [name]
Parent Table        : [parent]
Label               : [label]
Shared By           : [shared_by]
Alias(es)           :
Specification Type  : [x] Unique [] Generic [] Replica
Source Specification:
Description         : [description]

**Physical Elements:**

Data Type           : Numeric
Length              : 6
Decimal Places      : 0
Input Mask          : ######
Display Format      : 000000

Character Support:
    [] Letters (A-Z)    [] Keyboard (. , / $ # %)
    [x] Numbers (0-9)   [] Special (© ® ™ Σ π)

**Logical Elements:**

Key Type:
    [] Non      [x] Primary
    [] Foreign  [] Alternate

Key Structure       : [x] Simple        [] Composite
Uniqueness          : [] Non-unique     [x] Unique
Null Support        : [] Nulls Allowed  [x] No Nulls
Values Entered By   : [] User           [x] System
Required Value      : [] No             [x] Yes
Default Value       :
Range of Values     : 100,000–200,000

Edit Rule:
    [] Enter Now, Edits Allowed
    [x] Enter Now, Edits Not Allowed
    [] Enter Later, Edits Allowed
    [] Enter Later, Edits Not Allowed
    [] Not Determined At This Time

Comparisons Allowed:
    [x] Same Field       [] All [x] eq    [] gt [] geq    [] neq    [] lt [] leq
    [] Other Fields      [] All [] eq    [] gt [] geq    [] neq    [] lt [] leq
    [x] Value Expression [] All [x] eq    [] gt [] geq    [] neq    [] lt [] leq

Operations Allowed:
    [] Same Field       [] All [x] add    [] sub [] mult    [] div    [] concat
    [] Other Fields     [] All  [] add    [] sub [] mult    [] div    [] concat
    [] Value Expression [] All [x] add    [] sub [] mult    [] div    [] concat
