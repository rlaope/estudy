# What is Spring JDBC, DataSource?

![jdbc](./image/jdbc.png)

### What is DataSource?
- It contains connection information related to the DB and is registered as a bean and passed as an argument. >> Through this process, Spring obtains a connection to the DB via the DataSource.
  - Establishes a connection with the DB server.
  - DB Connection pooling functionality.

- Types: Various types exist depending on the JDBC Driver vendor (MySQL, Oracle).

### How to Configure, Register, and Inject DataSource as a Bean
1. Configure **DB server information (Properties)** for connecting to the DB. (It's best to configure this in a file to avoid hardcoding, as changes made here will apply to all other parts.)
2. Register the DataSource as a bean using the configured property file.
3. Inject the created DataSource bean into Spring JDBC.

### What is DB Connection Pooling?
- In Java programs, establishing a database connection (obtaining a connection object) takes a long time.
- A certain number of Connection objects are pre-created and stored. They are retrieved when requested!
- Improves speed and performance.
- DataSource manages the connection pool and handles the process of retrieving and returning connection objects from the pool!

<br>

## What is JDBC?
**An API provided by Java to access databases (Java Database Connectivity)**
In other words, it provides a way to query or update data in a database.
Using JDBC, you can implement database-independent DB integration logic. This means you can easily switch from MySQL to PostgreSQL, for example, because the JDBC API provides an interface that is compatible with various DB Drivers.

### Problems with Plain JDBC
- A lot of code needs to be written before and after executing a query (connection creation, statements, etc.).
- Time and resources are consumed for exception handling and transaction processing.
  - Errors occurring in JDBC are Runtime Exceptions. Therefore, all of them must be handled with exception handling.
- Spring JDBC emerged to address these issues.

<br>

## What is Spring JDBC?
It complements the shortcomings of JDBC and provides more convenient features.

### What Spring JDBC Does
- Opening and closing Connections
- Preparing and closing Statements
- Executing Statements
- Processing ResultSet loops
- Handling and returning Exceptions
- Handling Transactions

### What Developers Do in Spring JDBC
Developers only need to perform the core tasks; the framework handles the rest automatically.
- Configure DataSource
- Write SQL statements
- Process results
