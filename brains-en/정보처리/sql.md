# SQL Basic Syntax Summary

## Table Creation
Let's create a table named `books` with columns `id`, `title`, `writer`, and `quantity`.
```sql
CREATE TABLE books
(id INTEGER PRIMARY KEY
  title TEXT,
  writer TEXT,
  released_year INTEGER);
```

## Inserting Data

Let's insert data into the `books` table created above. There are two ways to insert data.
```sql
INSERT INTO books VALUES(1,"title" , "hope" , 2022);
INSERT INTO books VALUES(2,"titile","wow",2001)
```
It's tedious to enter all values like this. Sometimes, some values need to be left blank. If you set `autoincrement` for the `id` column when creating the table, the `id` value will automatically increment and be inserted without you having to manually enter it.

```sql
CREATE TABLE books
(id INTEGER PRIMARY KEY AUTOINCREMENT
  title TEXT, 
  writer TEXT, 
  released_year INTEGER);
```
If you specify column names, values that are not entered will be treated as `NULL`.

<br>

## Retrieving Data
When retrieving data, you use the `SELECT` statement, and you must follow a specific order.
- Writing Order
```
SELECT - FROM - WHERE - GROUP BY - HAVING - ORDER BY
```
- Execution Order
```
FROM - WHERE - GROUP BY - HAVING - SELECT - ORDER BY
```

### SELECT  
Enter the `COLUMN` names you want to retrieve.
Column names can be changed, and you can also add data that didn't exist previously.
```sql
SELECT '2005' year, HOUR(datetime) HOUR, count(hour(datetime)) COUNT
```
In this case, even if a column named `year` didn't exist, it can be added, and the data '2005' will be uniformly inserted. You can also set the desired column name after `hour(datetime)`. It can also be displayed as "HOUR".

### FROM
After `FROM`, enter the name of the table from which you want to retrieve values.

### WHERE
Aggregate functions cannot be used.
- To check if a field has a value, you can use `IS NULL` or `IS NOT NULL`.
  
```sql
SELECT * FROM books WHERE released_year IS NULL;
```
Only retrieves data where the `released_year` is `NULL`.

### GROUP BY
Retrieves data by grouping it.

### HAVING
Used with the `GROUP BY` clause, it allows conditional comparisons with aggregate functions.

### ORDER BY
Data sorting criteria. Multiple criteria can be set, and it's also possible to arrange in reverse order.
