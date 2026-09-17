# Hibernate

### What is Hibernate?

![](https://user-images.githubusercontent.com/48986787/130643074-1e9bc19b-c81a-42d4-bfaf-921ef8e4bd9f.png)

Hibernate is an ORM framework for the Java language. It is an implementation of JPA, implementing the JPA interface and internally using the JDBC API.

Key features of JPA include its ability to resolve the paradigm mismatch issue between relational database objects and its provision of a persistence context (an environment for permanently storing entities).

### JPA

It is an interface that defines how relational databases are used in Java applications. Since it's an interface, not a library, it doesn't perform specific functions on its own.

### JDBC
It is a standard that supports independent connections between the Java programming language and various databases, whether SQL or table-structured data. In other words, it can be seen as a standard for database operations.

DBMS companies implement and provide the JDBC interface. These are called JDBC drivers, and ultimately, a JDBC driver can be seen as an implementation of the methods specified in the standard JDBC interface, allowing DBMS companies to access their database systems.

Therefore, by using the JDBC API, a single Java application can access any type of relational DBMS that provides a JDBC driver, and users can manipulate databases by knowing only the JDBC API, even without knowing the exact usage methods of a specific company's database.

<br>

### Advantages of Hibernate

- **Productivity**
  - Hibernate executes queries using method calls instead of direct SQL. This increases productivity by eliminating repetitive SQL tasks.
  - It doesn't mean you don't need to know SQL; you need to understand its internal workings.
- **Maintainability**
  - When table columns change, it handles the parameters, results, and SQL of related DAOs on your behalf. This improves maintainability.
- **Vendor-independent**
  - JPA provides an abstract data access layer, making it vendor-independent.
  - You can easily switch databases by simply informing JPA which database you are using in the configuration file.
- **Resolves Paradigm Mismatch**
  - It can resolve the paradigm mismatch between objects and relational databases, including inheritance, associations, object graph traversal, and comparisons.
- **Delegates Business Responsibility to Objects**
  - When an object is loaded, its associated objects are also loaded, allowing for development that is not dependent on SQL.
  - In other words, object-oriented development is possible, rather than SQL-centric development.

### Disadvantages of Hibernate
- **Performance**
  - Executing queries solely through method calls is generally not as performant as writing direct SQL queries.
- **Granularity**
  - There are limitations to manipulating database data solely through method calls. To compensate for this, it supports JPQL, QueryDSL, and similar tools.
  - Since it supports Native Query, you can also write raw SQL queries.
- **Learning Curve**
  - There's a lot to learn.

### Spring Data JPA

Spring Data JPA is a module designed to make using JPA easier. It achieves this by providing a `Repository` interface, which abstracts JPA one step further.

If you define methods in the `Repository` interface according to its conventions, Spring automatically creates an implementation that executes queries appropriate for those method names and registers it as a Bean.
