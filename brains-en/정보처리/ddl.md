# DDL (Data Definition Language)

- DDL (Data Definition Language) is a language for defining data, or more precisely, 'a language for defining containers that hold data,' and these containers are referred to as objects in a DBMS.
- The targets, or object types, that can be defined through DDL are as follows.

DDL Target | Description
--|--
Schema | Data structure considering DBMS characteristics and implementation environment, intuitively understood as a single database.
Domain | Information specifying an attribute's data type, size, constraints, etc., understood as the range of values an attribute can hold.
Table | Data storage space
View | A virtual logical table derived from one or more physical tables
Index | Data structure to speed up searches

<br>

## DDL Commands

### CREATE
- Creates database objects

**New Creation**
```SQL
CREATE TABLE <table_name> (
  열이름 데이터 타입 [DEFAULT 값][NOT NULL]
  [PRIMARY KEY (열 리스트)]
  {[FOREIGN KEY(열 리스트) REFERENCES 테이블 이름 [(열 이름)]
  [ ON DELETE 옵션]
  [ ON UPDATE 옵션] ]} * 
  [CHECK (조건식) | UNIQUE (열이름)] ]};
)
```

**Creating a table using information from another table**
```sql
CREATE TABLE 테이블이름 AS SELECT 문'
```

### ALTER
- Modifies database objects

1. Add column
```sql
ALTER TABLE 테이블이름 ADD 열이름 데이터 타입
```

2. Change column data type
```sql
ALTER TABLE 테이블이름 MODIFY 열이름 데이터타입 
```

3. Delete column
```sql
ALTER TABLE 테이블이름 DROP 열이름
```

### DROP , TRUNCATE

- `DROP` : Deletes database objects
- `TRUNCATE` : Deletes the contents of a database object, but retains the table structure

1. Delete table
```sql
DROP TABLE 테이블이름
```

2. Delete table contents
```sql
TRUNCATE TABLE 테이블이름
```

3. Rename table
```sql
RENAME TABLE 이전 테이블이름 TO 새로운 테이블이름
ALTER TABLE 이전 테이블이름 RENAME 새로운 테이블 이름
```

<br>

## Data Types
- `CHAR` : Fixed-length string data type
- `VARCHAR` : Variable-length string data type
- `INT` : Data type used for numbers
- `FLOAT` : Floating-point data type
- `DATE` : Data type used for dates
