# ORM Concepts, Types, and Application Methods

### ORM Concepts
- ORM, or Object Relational Mapping, means connecting objects. It's a tool that allows applications to access databases using the application's development language instead of SQL when connecting to a database.
- By enabling the use of the application's development language instead of SQL syntax, ORM enhances the consistency and readability of the development language.

### ORM Application Directions
ORM offers a powerful advantage in backend development by enhancing the consistency and readability of the development language.

However, it's good to keep in mind that ORM alone makes it difficult to cover all aspects of SQL. Therefore, effective backend development requires a foundation of knowledge and experience in SQL queries.

<br>

### ORM Advantages 😃
- **Object-oriented code makes it more intuitive and helps focus on business logic.**
  - No need to write long SQL statements for CRUD operations.
  - Improves readability by writing code for each object (Model).
  - Increases productivity through an object-oriented approach rather than SQL's procedural approach.
- **Increased convenience for reuse and maintenance.**
  - Clear mapping information reduces dependency on viewing ERDs.
  - ORM is written independently, and these objects are reusable.
- **Reduced dependency on DBMS.**
  - Most ORMs are not dependent on the DB.
  - By focusing on objects, developers face relatively less risk and time even for extreme tasks like replacing the DBMS.
  - Being independent means it's valid not only for implementation methods but also for data types across many solutions.

> Dependency: Refers to a program's structure being affected by its data structure.

### ORM Disadvantages 🥲
- **Difficult to implement with ORM alone.**
  - While convenient to use, design requires extreme caution.
  - As project complexity increases, so does the difficulty.
  - If implemented incorrectly, it can lead to performance degradation and, in severe cases, consistency issues.
- **In systems with many procedures, it's difficult to leverage ORM's object-oriented advantages.**
  - In systems already rich in procedures, they must be converted back to objects, which can lead to decreased productivity or risks.

> Procedure: A part of a program for a specific task, similar to a function.
