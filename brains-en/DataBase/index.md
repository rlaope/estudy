# Index and Its Working Principle for Database Performance

![](./Image/index.png)

### Why Use Indexes

In the example above, let's say we use a SELECT statement to query for the value 'LCY' in the AIRPORT column of the SCOUNTER table. Then, the SELECT statement would operate by querying all data in the entire table and finally returning all values that include 'LCY'. When a human intuitively looks at the table to find results, they know the `position` where 'LCY' last appears and can ignore the data below it. However, because a database query statement doesn't know the last position of the 'LCY' value, it returns results by scanning the entire dataset. And if there's a service with hundreds of thousands of data entries where the query function is frequently executed, continuously scanning the database from start to finish to return values will inevitably lead to performance degradation as traffic increases. To prevent such issues, databases use `indexes` to create separate Index Tables for frequently queried columns, and when a SELECT statement comes in, they retrieve result values using the values in the Index Table. Therefore, if indexes are used effectively, the performance of `search` operations can be dramatically improved.

<br>

### How Does an Index Work?

Continuing with the example above, if an index is given to the AIRPORT column when creating the SCOUNTER TABLE, an index table for the AIRPORT column is created. Later, when a query containing a WHERE clause for the AIRPORT column is executed on the SCOUNTER table, it references the key-value pairs stored in the AIRPORT index table to return results from the SCOUNTER table.

**Operation Sequence**
- Find the value included in the WHERE clause within the Index Table.
- Retrieve the table_id[PK] for that value.
- Query the original table for the value using the retrieved table_id[PK].

DBMS manages indexes using various algorithms, and the commonly used algorithm is the B+ Tree algorithm.

<br>

### B+Tree Index Algorithm

![](./Image/btree.png)

#### B-Tree Structure
- Leaf Node: The node where actual data is stored.
- Internal Node: A node that serves as a path to the leaf nodes.
- Root Node: The starting point of the path.

B+Tree stores pointers to child nodes all the way to the leaf nodes, making it a very efficient algorithm for searching because only one path from the root node to a specific leaf node needs to be traversed.

<br>

### Index Types

Indexes, unlike the original DB tables, can have data redundancy. Therefore, various types of indexes exist, and it's best to use the appropriate type depending on your specific purpose.

#### Index Classification by Key
- Primary Index: An index that includes the primary key (the order of keys determines the order of records).
- Secondary Index: An index other than the primary index (the order of keys does not imply the order of records).

#### Index Classification by File Organization
- Clustered Index: An index structured so that the physical order of data records is maintained identically or similarly to the order of index entries for that file.
- Non-clustered Index: An index that is not in a clustered form.

#### Index Classification by Data Range
- Dense Index: An index where one index entry is created for each data record.
- Sparse Index: An index where one entry is created for a group of records or a data block.

<br>

## When to Use Indexes?

### Characteristics of Indexes

#### Index Table
- An index is created and used by generating a separate table to store values. Therefore, since a new table dependent on another table is created, indiscriminate index creation can actually lead to performance degradation.

#### Sorting
- Index tables are inherently sorted because they use "binary tree search." Therefore, if "insertions," "deletions," or "modifications" frequently occur in the table referenced by the index table, the index table will perform sorting during these operations, which can lead to overall performance degradation.

- For example, in a service like Baedal Minjok (a food delivery app), where users primarily query and search for delivery restaurants by region, using indexes can optimize database performance.
- However, in social media platforms like Facebook, where users constantly generate new posts, using indexes can actually degrade performance.

In conclusion, since indexes are optimized for search operations, the decision to use them should be carefully considered based on business logic where insertions, deletions, and modifications occur frequently, or on the intended use of the table.
