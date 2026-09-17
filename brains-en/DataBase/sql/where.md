# WHERE Clause Practice

The `WHERE` clause applies conditions to a query.

> We will use the NETFLIX data created in the DDL and DML practice.

Let's query data using =, IN, and NOT IN.

```sql
SELECT * FROM NETFLIX WHERE CATEGORY = '애니메이션'; # 카테고리가 애니메이션인 컬럼 조회

SELECT * FROM NETFLIX WHERE CATEGORY IN ('애니메이션', '영화'); # 카테고리가 애니메이션이거나 영화인 컬럼 조회

SELECT * FROM NETFLIX WHERE CATEGORY NOT IN ('애니메이션', '영화'); # 카테고리가 애니메이션, 영화가 아닌 컬럼 조회
```

This time, let's query data using ranges.
We will retrieve data with view counts less than 70 and less than or equal to 70, respectively.

```sql
SELECT * FROM NETFLIX WHERE VIEW_COUNT < 70;

SELECT * FROM NETFLIX WHERE VIEW_COUNT <= 70;
```
Now, let's query data using dates.
This retrieves data registered before 2023.
```sql
SELECT * FROM NETFLIX WHERE REGISTER_DATE < TO_DATE('20230101', 'YYYYMMDD');
```

Next, I will write multiple conditions using AND.

```sql
SELECT * FROM NETFLIX WHERE CATEGORY = '애니메이션' AND VIEW_COUNT < 70;

SELECT * FROM NETFLIX WHERE CATEGORY = '애니메이션' AND REGISTER_DATE < TO_DATE('20230101','YYYYMMDD');
```

You can also retrieve data that satisfies one or more conditions using OR.

```sql
SELECT * FROM NETFLIX WHERE CATEGORY = '애니메이션' OR VIEW_COUNT < 70;

SELECT * FROM NETFLIX WHERE CATEGORY = '애니메이션' OR CATEGORY = '영화'; # 같은 컬럼에도 조건 부여 가능, 위에서 IN 절하고 같은 구문
```

You can also specify specific values using LIKE.

```sql
SELECT * FROM NETFLIX WHERE VIDEO_NAME LIKE '미%'; # 미로 시작하는 비디오 이름을 가지고 있는 데이터

SELECT * FROM NETFLIX WHERE VIDEO_NAME LIKE '%미'; # 미로 끝나는 비디오 이름을 가지고 있는 데이터

SELECT * FROM NETFLIX WHERE VIDEO_NAME LIKE '%미%'; # 미가 값 중간에 있는 비디오 이름을 가지고 있는 데이터
```

While you can specify conditions by defining a range twice with AND, you can also do so more simply using BETWEEN.

```sql
SELECT * FROM NETFLIX WHERE VIEW_COUNT >= 60 AND VIEW_COUNT <= 70;

SELECT * FROM NETFLIX WHERE VIEW_COUNT BETWEEN 60 AND 70;
```
