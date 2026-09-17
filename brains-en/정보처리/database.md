# Database

## Database Concepts
- A collection of data integrated and managed for use by multiple people, systems, or programs.
- With the development of IT systems, effective management of rapidly increasing data has become crucial. `Eliminating redundancy`, `ensuring integrity`, `maintaining consistency`, and `guaranteeing usability` of data are core aspects of database management.

<br>

## Database Characteristics
1. Real-time Accessibility
2. Continuous Evolution
3. Concurrent Sharing
4. Content Reference

<br>

### Database Design Order
Requirements Analysis > Conceptual Design > Logical Design > Physical Design > Implementation

<br>

## Database Terminology
- Attribute: Refers to a single column within a relation, representing and storing an entity. It is often expressed as a `column` or `field`.
- Tuple: Refers to a single row within a relation, and is sometimes expressed as a `record` or `row`.
- Degree: Refers to the number of attributes contained within a single relation.
- Cardinality: Refers to the number of tuples contained within a single relation.
- Domain: Refers to the set of values that each attribute within a relation can take.
- View: A virtual table derived from one or more base tables, whose structure and operations are very similar to those of base tables.

 <br>

### Schema
- Refers to the overall specification regarding the structure and constraints of a database.
  1. Internal Schema (Physical): The database defined from the perspective of system programmers or designers.
  2. Conceptual Schema (Logical): The database defined from the perspective of an institution or organization, based on the data users need.
  3. External Schema (Subschema): The database needed from the individual perspective of users.

  <br>

### Key

- `Key` functions as a unique identifier for identifying something.
  1. Candidate Key: A set of attributes that satisfy `uniqueness` and `minimality`.
  2. Primary Key: A key selected from candidate keys, where `duplicate values cannot be entered` and `Null values are not allowed`.
  3. Super Key: A set of attributes that satisfy `uniqueness` but `do not satisfy minimality`.
  4. Alternate Key: Candidate keys that were not chosen as the primary key.
  5. Foreign Key: A key that identifies a row in another table.

  **Uniqueness**
  > The property of unique data that allows a specific row to be found directly using a single key.

  <br><br>

  ## Database Management System (DBMS)
  - It is software that resolves the complexity of data management as described above, `while simultaneously supporting functions` such as `data addition`, `modification`, `deletion`, `backup`, `recovery`, and `security`.
  - The stored information is highly diverse, including text, images, music files, and map data.

  ### DBMS Pros and Cons

  - `Pros`
    1. Minimizing data redundancy
    2. Data sharing (maintaining consistency)
    3. Maintaining consistency, integrity, and security
    4. User-centric data processing
    5. Applicability of data standardization
    6. Easy data access
    7. Savings due to shared data storage space
  
  - `Cons`
    1. Requires database specialists (DBA)
    2. Need for DBMS server setup and maintenance costs
    3. Difficulty in data backup and recovery
    4. System complexity
    5. Overload due to bottleneck when access is concentrated on large-capacity disks
    6. Difficulty in processing large volumes of data

<br>

### Database Administrator (DBA)
- Rather than directly utilizing the database, they design and build it for users, and manage and control it to ensure proper service.
- Key responsibilities of a Database Administrator
  1. Selection of database components
  2. Definition of database schema
  3. Determination of physical storage structure and access methods
  4. Definition of constraints for maintaining integrity
  5. Decision on security and access control policies
  6. Definition of backup and recovery techniques
  7. Management of system databases
  8. System performance monitoring and analysis
  9. Database reorganization
