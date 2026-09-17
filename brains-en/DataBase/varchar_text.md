# VARCHAR vs TEXT

Today, we'll explore TEXT and VARCHAR in MySQL and look at their differences.

### VARCHAR

When specifying a column type as VARCHAR,

if we set VARCHAR(10) and VARCHAR(1000) differently,

will there be a performance difference? Why not just set it to a very large size?

Let's try creating a table with a very long VARCHAR.

```mysql
mysql> CREATE TABLE tb_long_varchar (id INT PRIMARY KEY, fd1 VARCHAR(1000000));  
ERROR 1074 (42000): Column length too big for column 'fd1' (max = 16383); use BLOB or TEXT instead

mysql> CREATE TABLE tb_long_varchar (id INT PRIMARY KEY, fd1 VARCHAR(16383));  
ERROR 1118 (42000): Row size too large. The maximum row size for the used table type, not counting BLOBs, is 65535. This includes storage overhead, check the manual. You have to change some columns to TEXT or BLOBs  
  
mysql> CREATE TABLE tb_long_varchar (id INT PRIMARY KEY, fd VARCHAR(16382));  
Query OK, 0 rows affected (0.19 sec)  
  
mysql> ALTER TABLE tb_long_varchar ADD fd2 VARCHAR(10);  
ERROR 1118 (42000): Row size too large. The maximum row size for the used table type, not counting BLOBs, is 65535. This includes storage overhead, check the manual. You have to change some columns to TEXT or BLOBs
```

As seen above, table creation and alteration fail because a single record in the table exceeds the maximum length it can store (65,535 bytes).

From the 4th line, we can see that if a VARCHAR column uses too much length, it affects the maximum space available for other columns.

Therefore, when setting the maximum length for a VARCHAR type, space must be used sparingly.

<br>

### A Question about TEXT here

Now, I have a question. LOB (Large Object) type columns like TEXT are not affected by length restrictions. So, in tables with many columns, we might need to use TEXT types.

But is this the only reason to be careful with VARCHAR type length settings? What if we set it up like this, assuming no new columns are needed?

```mysql
CREATE TABLE user (  
id BIGINT NOT NULL,  
name VARCHAR(4000),  
phone_no VARCHAR(4000),  
address VARCHAR(4000),  
email VARCHAR(4000),  
PRIMARY KEY(id)  
);
```

If we design the table using TEXT, the length restriction issues completely disappear. Not only are length setting constraints removed, but the length limit for stored values is also much larger, allowing for a more flexible table design.

```mysql
CREATE TABLE user (  
id BIGINT NOT NULL,  
name TEXT,  
phone_no TEXT,  
address TEXT,  
email TEXT,  
PRIMARY KEY(id)  
);
```

This might raise a question: why use VARCHAR? It's less flexible in length, has lower scalability, and TEXT allows for much larger storage.

Let's find out.

<br>

## VARCHAR vs TEXT

### General RDBMS

In general RDBMS, LOB (Large Object) type data like TEXT is stored in external storage called `Off-Page`.

Column data for records of typical length is stored `inline` in a `B-Tree (Clustering Index)`.

### MySQL

However, MySQL Server does not always store LOB type columns `Off-Page`; it only stores them there if their length is long and requires a lot of storage space.

```mysql
CREATE TABLE tb_lob (  
id INT PRIMARY KEY,  
fd TEXT  
);  
  
INSERT INTO tb_lob VALUES (1, REPEAT('A',8100)); -- // Inline storage  
INSERT INTO tb_lob VALUES (2, REPEAT('A',8101)); -- // Off-Page storage
```

As shown above, data item 1, storing 8100 characters (8100 bytes), is stored in the B-Tree.
Data item 2, storing 8101 characters (8101 bytes), is stored Off-Page.

This behavior can vary slightly depending on the MySQL Server record format.

> The record size limit for MySQL Server is 65,535 bytes, but the InnoDB storage engine's record size limit varies depending on the page (block) size. In most cases, half of the page size acts as the maximum record size limit for the InnoDB storage engine. (In the example above, for a 16KB page, if a column exceeds 8117 bytes, it is chosen to be stored Off-Page.)

So, is TEXT the only type that gets stored Off-Page when it's long?

No, that's not the case. VARCHAR also gets stored Off-Page if the record value stored in the column is large.

```mysql
CREATE TABLE tb_varchar (  
id INT PRIMARY KEY,  
fd VARCHAR  
);  
  
INSERT INTO tb_varchar VALUES (1, REPEAT('A',8100)); -- // Inline storage  
INSERT INTO tb_varchar VALUES (2, REPEAT('A',8101)); -- // Off-Page storage
```

### Misconception about Indexes

There's also a misconception that VARCHAR types can be indexed but LOB types cannot. This is incorrect. LOB types can also be indexed if they meet the data length restrictions.

```mysql
-- // When using the column as is, index creation is not possible  
mysql> ALTER TABLE tb_varchar ADD INDEX ix_fd (fd);  
ERROR 1071 (42000): Specified key was too long; max key length is 3072 bytes  
  
mysql> ALTER TABLE tb_lob ADD INDEX ix_fd (fd);  
ERROR 1170 (42000): BLOB/TEXT column 'fd' used in key specification without a key length  
  
-- // If the column value length (prefix) is specified, index creation is possible  
mysql> ALTER TABLE tb_varchar ADD INDEX ix_fd ( fd(50) );  
mysql> ALTER TABLE tb_lob ADD INDEX ix_fd ( fd(50) );
```

As shown above, index creation is possible when a length prefix is specified.

Ultimately, if Off-Page storage is possible and B-Tree indexes can be created, what exactly is the difference? Is it just a difference in declaration? It seems the distinction becomes even more ambiguous. So, in what situations should each be used?

<br>

## VARCHAR, TEXT Memory Utilization

MySQL Server exchanges data with the storage engine via the Handler API.

At this point, the MySQL engine and storage engine exchange data using `uchar *records[2]` memory pointers. The `records[2]` memory object always allocates memory to its maximum size, regardless of the data length.

Here, VARCHAR has a specified length, so it can pre-allocate data in the buffer.

However, for LOB types like TEXT, allocating memory to the actual maximum size would lead to severe memory waste. Therefore, the memory space pointed to by `records[2]` includes VARCHAR but does not include space for TEXT columns.

The `uchar *records[2]` memory space is defined within the TABLE structure and is implemented to be cached internally within the MySQL server, allowing it to be shared across multiple connections.

This means the `record[2]` memory buffer, once allocated, is designed to be reusable by many connections.

However, memory space for TEXT and LOB columns is not pre-allocated in `records[2]`, so memory must be allocated as needed each time a record is read, and then freed.

```mysql
CREATE TABLE tb_lob (  
id INT PRIMARY KEY,  
fd TEXT  
);  
  
CREATE TABLE tb_varchar1 (  
id INT PRIMARY KEY,  
fd VARCHAR(100)  
);  
  
CREATE TABLE tb_varchar2 (  
id INT PRIMARY KEY,  
fd VARCHAR(10000)  
);
```

For example, if tables are created as above, the `record[2]` buffer space for `tb_lob` will be allocated as 16x2 bytes, while `tb_varchar1` and `tb_varchar2` will allocate 408x2 and 40008x2 bytes respectively.

- The `tb_lob` table allocates 4 bytes for the `INT` type column (id), 8 bytes for the pointer space for `TEXT` values, and 4 bytes for header space.
- The `tb_varchar1` table allocates 4 bytes for the `INT` type column (id), 400 bytes for the `VARCHAR(100)` type column, and 4 bytes for header space.
- The `tb_varchar2` table allocates 4 bytes for the `INT` type column (id), 40000 bytes for the `VARCHAR(10000)` type column, and 4 bytes for header space.

Therefore, when reading VARCHAR type data, it uses the `records[2]` buffer instead of newly allocating memory. However, TEXT does not have pre-allocated memory space, so it allocates memory as needed for each use and then frees it.

The memory space allocated and freed to read LOB column values is not measured by `Performance_schema`. Thus, it's difficult to gauge its performance impact.

> Additionally, it's important to note that if a VARCHAR is long enough to be stored off-page, it cannot use the `records[2]` buffer. This is something to be mindful of.

Ultimately, the rules for selecting column types can be summarized as follows:

### VARCHAR
- When the maximum length is **relatively** not large.
- When the column is always needed when reading table data.
- When the DBMS server has **relatively** sufficient memory.

### TEXT
- When the maximum length is **relatively** large.
- When the table requires many long string type columns.
- When the column is not frequently needed when reading table data.

I emphasized the word "relatively" above because the impact varies depending on the DBMS server specifications, data model, and incoming traffic. Therefore, it's difficult to define a single criterion for all judgments. I hope you analyze carefully and design well.
