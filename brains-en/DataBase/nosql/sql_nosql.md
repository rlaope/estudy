# SQL vs NoSQL

### RDB

In RDB, **normalization** is used for efficient data management.

It prevents data redundancy and allows for complex and diverse queries.

However, a disadvantage is that queries are slow because relationships between multiple tables are maintained via foreign keys, and related data is retrieved by joining them.

Another disadvantage is poor scalability.

The effort required for schema changes is also very high.

It can be seen as having been designed without considering techniques like scale-out.

### NoSQL

It emerged to solve problems that relational databases couldn't or struggled with.

Data visibility is excellent. It's possible to embed documents within documents in JSON format.

Data retrieval is very fast because it's possible to query data without joins.

The schema is flexible, allowing data models to accommodate application requirements.

Scale-out is easy.

Disadvantages include significant data redundancy due to denormalization, and performance degradation if schema design is poor.

> NoSQL doesn't actually mean "No SQL"; it means "Not-Only SQL." You can think of it as a database that supports various methods in addition to basic SQL.

Is NoSQL always better? Not necessarily. This is because when storing related data in NoSQL, there can be a lot of data redundancy. Let's use it according to the appropriate situation.
