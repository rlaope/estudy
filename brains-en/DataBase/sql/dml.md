# DML (INSERT, UPDATE, DELETE, SELECT) Practice + TCL (COMMIT, ROLLBACK)

> I will be using the NETFLIX table created in the previous DDL practice for this exercise.

## INSERT

I will now insert data using an INSERT statement.

```sql
INSERT INTO NETFLIX VALUES('나의 아저씨', '드라마', 30, SYSDATE); 

COMMIT;
```

SYSDATE refers to the current time in the program environment.
COMMIT is a command that permanently applies (persists) the saved data.
  
Let's continue. This time, I will add data to specific columns only.
And I will also add new video data that was registered 30, 40, and 300 days ago from today.
```sql
INSERT INTO NETFLIX (VIDEO_NAME, VIEW_COUNT) VALUES('시그널', 42)

INSERT INTO NETFLIX VALUES('응답하라 1988', '드라마', 42, SYSDATE - 30); 
INSERT INTO NETFLIX VALUES('이태원 클라쓰', '드라마', 32, SYSDATE - 40); 
INSERT INTO NETFLIX VALUES('미스터 션샤인', '드라마', 22, SYSDATE - 300); 

COMMIT;

ROLLBACK;
```

If I insert that data and then enter ROLLBACK, only "My Mister" data will remain. (Rolling back the most recent commit)

## UPDATE
In the INSERT statement above, "Signal" has NULL for category and registration date.
This time, I will modify the column values, starting with updating the view count for "My Mister".

```sql
UPDATE NETFLIX SET VIEW_COUNT = 70 WHERE VIDEO_NAME = '나의 아저씨';

COMMIT;
```

Next, I will add the category and registration date to "Signal".

```sql
UPDATE NETFLIX SET CATEGORY = "드라마", REGISTER_DATE = TO_DATE('20230101', 'YYYYMMDD') =  WHERE VIDEO_NAME = '시그널';

COMMIT;
```

The TO_DATE() function takes a date as input (first argument) and converts it into a DATE type using the specified format (second argument).

## DELETE
Differences between TRUNCATE and DELETE

1. TRUNCATE only deletes all data, whereas DELETE can also delete selected data.
2. TRUNCATE does not allow data rollback, but DELETE does.
3. When deleting all data, TRUNCATE performs faster.
  
First, I will delete the data for "Mr. Sunshine".
```sql
DELETE FROM NETFLIX WHERE VIDEO_NAME = '미스터 션샤인';

COMMIT;
```

This time, I will delete data that is a drama and has a view count less than 35.
```sql
DELETE FROM NETFLIX WHERE CATEGORY = '드라마' AND VIEW_COUNT < 35;

COMMIT;
```

I will delete data using the IN clause.
```sql
DELETE FROM NETFLIX WHERE VIDEO_NAME IN('이태원 클라쓰','나의 아저씨');
```

Finally, I will try deleting all data using DELETE.
```sql
DELETE * FROM NETFLIX;
```

## SELECT
Knowing just SELECT can be considered knowing half of SQL. Although there's more to learn, I will only conduct a simple practice this time.
  
Retrieve all columns from the table. The two queries have the same meaning.
```sql
SELECT * FROM NETFLIX

SELECT VIDEO_NAME, VIEW_COUNT, CATEGORY, REGISTER_DATE FROM NETFLIX;
```

You can retrieve data from only the desired columns. There must always be at least one column between SELECT and FROM. (An error will occur if there isn't)
```sql
SELECT VIDEO_NAME, VIEW_COUNT, CATEGORY FROM NETFLIX;
SELECT VIDEO_NAME, VIEW_COUNT FROM NETFLIX;
SELECT VIDEO_NAME FROM NETFLIX;
```

This time, I will query using a WHERE clause.
I will query for videos titled "My Mister" and those not titled "My Mister".
```sql
SELECT * FROM NETFLIX WHERE VIDEO_NAME = '나의 아저씨';

SELECT * FROM NETFLIX WHERE VIDEO_NAME <> '나의 아저씨';
```

I will query for data registered within the last month from the current date.
```sql
SELECT * FROM NETFLIX WHERE REGISTER_DATE > SYSDATE - 30;
SELECT * FROM NETFLIX WHERE REGISTER_DATE < SYSDATE - 30; # 이건 반대로 한달 전
``` 

Adding DISTINCT removes duplicates (returns the first retrieved data).
```sql
SELECT DISTINCT CATEGORY FROM NETFLIX;
```
