### Partitioning Types and Splitting Criteria

In this post, we will explore the two types of partitioning and their splitting criteria.

## Partitioning Types

Partitioning is divided into the following two types:
- Horizontal Partitioning
- Vertical Partitioning

Let's look at each one.

### Horizontal Partitioning

Horizontal partitioning involves splitting a table based on rows, and its mechanism is the same as sharding.

> The difference from sharding is whether the partitioned tables are stored in a single DB or across multiple DBs.

The criterion for splitting is based on a single piece of data, also known as the partitioning key.

![](https://gmlwjd9405.github.io/images/database/horizontal-partitioning.png)

Tables are split, stored, and managed based on the partitioning key.

This is done to manage data by splitting it based on the partitioning key for performance and availability.

For example, let's say customer IDs from 1 to 10000 are stored in Table A, and from 10001 to 20000 in Table B.

This is horizontal partitioning using customer ID as the partitioning key (range partitioning).

There are criteria for splitting partitioning keys, which I will mention again below.

<br>

### Vertical Partitioning

Unlike horizontal partitioning, this involves splitting a table based on columns.

It's a concept similar to 3rd Normal Form.

It's also the process of separating a single entity into two, and unlike horizontal partitioning, it doesn't divide tables within the same schema but rather divides schemas, with data moving accordingly.

While it might seem similar to 3rd Normal Form, it's slightly different because it involves partitioning an already normalized table, unlike 3rd Normal Form itself.

The advantage is that it can improve I/O performance by separating frequently used columns, or columns with data types that consume a lot of space.

For example, if a `board` table contains information like `content`, `title`, and `writer_id`, and you try to fetch only `title` and `writer_id` through a DB I/O operation, even when specific columns are designated in a `SELECT` statement, the `content` column is also read into memory and filtered there. Therefore, always fetching a potentially large column like `content` can increase task latency.

For the reasons above, separating frequently used and infrequently used columns can improve I/O performance.

<br>

## Splitting Criteria

DBMS provides various partitioning techniques when splitting tables. At this time, partitioning keys are used together.

![](https://gmlwjd9405.github.io/images/database/partitioning.png)

As shown above, there are four partitioning techniques; let's look at each one.

### Range Partitioning
This distinguishes keys based on the range of partitioning key values.

For example, it involves dividing ranges like postal codes between 1 and 10000.

### List Partitioning
This assigns partitions to a list of values and selects a partition by checking if the partitioning key value corresponds to that list.

For example, if a `Country` column contains Japan, Korea, and China, you can build partitions for Asian countries.

### Hash Partitioning
This involves splitting partitioning keys according to the value of a hash function.

If there are four partition tables, this hash function returns values from 0 to 3.

### Composite Partitioning
This refers to using a combination of the three partitioning techniques mentioned above.

For example, one might first partition by range and then by hash.

Consistent hashing can be considered a composite of hash partitioning and list partitioning, allowing for enumeration by hash-reducing the key space.
