# MongoDB

MongoDB is a database developed to address the scalability and speed issues of traditional RDBs.
It is a type of NoSQL database and uses a `Document` data structure.

> See the difference between [[NoSQL vs SQL]] here

Data duplication can occur, but it offers good accessibility, visibility, and read performance.

A disadvantage is that changes (creation, modification, deletion) must be reflected in each collection.

Schema design can be challenging, but its flexible schema allows it to accommodate data according to application requirements.

It natively supports HA and Scale-Out solutions, making it easy to scale.

This means applications don't need to consider Scale-Out.

It supports Secondary Indexes.

It provides various types of Indexes.

I will discuss indexes later.

> Please link here when you organize indexes!

<br>

## Concepts
![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbqqSg9%2FbtrURkI0bj7%2FCv8vUK0OhLjixrQkRMR34k%2Fimg.png)

These are concepts in MongoDB that correspond to RDBMS.

### 1. Core Databases

MongoDB provides the following three core databases. (They might not be visible depending on root privileges.)

- admin: For authentication and authorization roles
- local: Stores collections like oplog needed for replication, and information like startup_log for instance diagnostics; excluded from replication targets
- config: Stores information about each shard in a sharded cluster

<br>

### 2. Collection

It has a **dynamic schema**. Therefore, to modify the schema, you only need to add, modify, or delete values. It might be better to think of it as having no schema concept at all?

- Indexes can be created per Collection.
- Shards can be divided per Collection.
    - This means that to utilize Indexes or Sharding Keys, the schema needs to be maintained to some extent.

<br>

### 3. Document

Data is stored in BSON (Binary-JSON) format.

It offers faster text-based parsing and is more space-efficient than JSON.

Every Document has an `_id` field; if not provided during creation, a unique ObjectId is stored.

If the parent structures, DataBase and Collection, do not exist during creation, they are created first, and then the Document is created.

The maximum size is fixed at 16MB.
