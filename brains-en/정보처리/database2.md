# Database 2

## Types of Databases
1. `File System`: A data management method that precedes databases, where files are named, and their logical locations for storage or retrieval are defined and managed.
   
### ISAM
 > A system where data content is stored in the primary storage, and data indexes, along with the data's recorded location, are stored in the index section.

 ### VSAM
 > A file management system used in large operating systems.


2. Hierarchical Database (HDBMS)
   - A database that manages data by hierarchically structuring it in parent-child dependent relationships.
   - It offers fast data access, but due to its dependent structure, it's not easy to flexibly adapt to changing data structures.  


3. Network Database Management System (NDBMS)
   - A data model that logically represents data structures in a network-like form.
   - It is more flexible than tree structures or hierarchical databases, but it has the disadvantage of complex design.
   - Products include `IDS`, `TOTAL`, and `IDMS`.

4. Relational Database Management System (RDBMS)
   - The most common database management system.
   - It organizes correlations by representing parts of data-storing tables in parent-child relationships with other tables.
   - It offers good flexibility for changing business requirements and data structures, making maintenance easy.

5. NoSQL (Not Only SQL)
    - It uses a less restrictive consistency model than traditional relational databases and is used for big data and commercial real-time web applications.


## Characteristics of Database Management Systems (DBMS)
- `Data Integrity`: The property of not allowing inappropriate data to be entered, preventing different data from being stored for the same content.
- `Data Consistency`: The property that stored data must remain constant and unchanged even after insertion, deletion, update, or creation.
- `Data Recoverability`: The property that data must be restored to a specific state in the event of a failure.
- `Data Security`: The property that data must be protected from illegal exposure, alteration, or loss.
- `Data Efficiency`: The property that response time, storage space utilization, etc., must be optimized to satisfy the requirements of users, software, and systems.

### Database Management System Classification

1. Classification by System Characteristics
- Relational Databases
  > Oracle, MySQL, MS SQL Server, PostgreSQL, DB2, Maria DB
- Document Store DBMS
  > MongoDB, Amazon DynamoDB, Couchbase, MS Azure Cosmos DB
- Graph DBMS
  > Neo4j, MS Azure Cosmos DB, Orient DB, Arango DB
- Key-Value DBMS
  > Redis, Amazon DynamoDB, Memcached

2. Classification by Commercialization and Open Source
- Commercial DBMS: Software created for commercial purposes or for sale.
- Open Source Based DBMS: Software made public and usable without restrictions.

## Database Management

### Database Operations
- CRUD refers to the basic data processing functions of a database: Create, Read, Update, and Delete.
  
Operation | SQL | Description
---|---|---
Create | Insert | Adds data to a column within a table.  
Read | Select | Retrieves data stored in a column within a table.  
Update | Update | Modifies data stored in a column within a table.  
Delete | Delete | Deletes data stored in a column within a table.

1. Data Insertion
   - To insert data into a table, use the `Insert` command.
  
  ```SQL
  INSERT INTO table_name
  VALUES(value1 , value2, value3, ...);
  ```

2. Data Reading
   - To read data from a table, use the `Select` command.
   - Using `*` after `Select` reads all data, while specifying column names reads only specific columns.

```SQL
SELECT column1 , column2, ...
FROM table_name;
```

3. Data Update
    - To update data in a table, use the `Update` command.
    - You must specify the data to be updated using the `WHERE` clause.

```SQL
UPDATE table_name
SET column1 = value1, column2 = value2, ...
WHERE condition;
```

4. Data Deletion
   - To delete data from a table, use the `Delete` command.
   - You must specify the data to be deleted using the `WHERE` clause.

5. Other SQL Commands
   - Modify a database using the `ALTER DATABASE` command.
   - Modify table structure using the `ALTER TABLE` command.
   - Delete an entire table using the `DROP TABLE` command.
   - Create an index to improve data search speed within a table using the `CREATE INDEX` command.
   - Additionally, various commands such as `UNION` and `GROUP BY` are used.
