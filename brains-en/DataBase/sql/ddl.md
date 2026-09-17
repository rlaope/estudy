# DDL (CREATE, ALTER, DROP, TRUNCATE) Practice

## CREATE
Let's try creating a Netflix table.

The columns will include video name, category, view count, and video registration date.

```sql
CREATE TABLE NETFLIX(

    VIDEO_NAME VARCHAR2(20),
    CATEGORY VARCHAR2(20),
    VIEW_CNT NUMBER(7),
    REGISTER_DATE DATE
);
```

> In ORACLE, character types are declared as VARCHAR2.
> The argument in NUMBER indicates the number of digits. 7 > 1,000,000 (7 digits)

## ALTER
Now, let's add a new column, 'CAST_MEMBER', to the registered Netflix table.
```sql
ALTER TABLE NETFLIX ADD(CAST_MEMBER VARCHAR2(20))
```

This time, let's increase the character length of 'CAST_MEMBER' to 50.
```sql
ALTER TABLE NETFLIX MODIFY(CAST_MEMBER VARCHAR2(50))
```

Now, let's change the type of 'CAST_MEMBER' to NUMBER. (However, an error will occur if existing data for this column is in character format.)
```sql
ALTER TABLE NETFLIX MODIFY(CAST_MEMBER NUMBER(2))
```

Finally, let's delete the 'CAST_MEMBER' column.

```sql
ALTER TABLE NETFLIX DROP(CAST_MEMBER)
```

## DROP, TRUNCATE

DROP deletes the table itself,
while TRUNCATE initializes the table (data recovery is impossible because it doesn't log the operations).

First, let's create a test table and add some data to it.
```sql
CREATE TABLE TEST_TABLE(
    COL1 VARCHAR2(3),
    COL2 VARCHAR2(3)
);

INSERT INTO TEST_TABLE VALUES('AAA', 'BBB');
INSERT INTO TEST_TABLE VALUES('CCC', 'DDD');

COMMIT;
```

After performing DROP and then SELECTing TEST_TABLE, no table will be found.

```sql
DROP TABLE TEST_TABLE;
SELECT * FROM TEST_TABLE;
```

After performing TRUNCATE and then SELECTing TEST_TABLE, all data will be initialized.
```sql
TRUNCATE TABLE TEST_TABLE;
SELECT * FROM TEST_TABLE;
```
