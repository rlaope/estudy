# MongoDB Trade-offs (vs RDB)

MongoDB is an open-source document-based database.

It offers excellent performance, usability, and auto-scaling.

### Document Database

MongoDB stores data in a document format consisting of keys and values.

MongoDB documents have a structure very similar to JSON.

The value of each field can be composed of documents, arrays, or arrays of documents.

The advantages of the document format are as follows:

- Documents are primitive data types in various programming languages. (e.g., objects)
- Embedded documents or arrays can reduce costly join operations.
- Dynamic Schema allows for rich representation of data forms.

> Simply put, embedding refers to the method of inserting other documents or arrays within a document. This allows for easy retrieval of information from other documents.

### Dynamic Schema

Dynamic Schema means that the structure of documents representing data can be defined dynamically.

It's called a dynamic schema because the structure of documents can be defined dynamically.

With a dynamic schema, each document can have a different structure. This means two documents belonging to the same collection can have different fields.

This provides flexibility when adding or deleting new fields.

However, to maintain data validity and consistency, the data model must be easily extensible and modifiable.

We've explored the advantages of document-based databases in [[MongoDB - Document Database]], and now let's look at the trade-offs.

### MySQL vs MongoDB

Let's compare MySQL, often considered a representative RDBMS, with MongoDB.

In what situations should they be used?

If you need to handle structured data, require ACID transactions, and complex queries and join operations, choosing an RDB like MySQL might be a good option.

On the other hand, MongoDB can be considered if you need a flexible data model, need to store unstructured data or various data types, scalability is important, you're dealing with large volumes of data, or you're focusing on handling JSON-formatted documents.

#### MySQL without joins?

This raised a question for me. If you query in MySQL without using joins, how is that different from simply querying in MongoDB?

In fact, there's no difference. If you query in MySQL without using joins, data is retrieved via indexing, so which is faster depends on the index configuration. In other words, it's impossible to say which is inherently better.

Rather, if transactions are required, MySQL might be a better consideration.

**Additionally, MySQL started supporting the JSON data type from version 5.7.** This allows for creating structures similar to embedded documents. It can mimic the advantages of NoSQL while still supporting transactions, thus retaining its benefits.

> Honestly, in my personal opinion, mimicking NoSQL isn't ideal given the reasons RDBs were created and the direction they pursue.

There are differences in sharding methods, which I will discuss in another post.
