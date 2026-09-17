# WHERE 조건절 실습

`WHERE` 쿼리에 조건을 부여한다.
  
> DDL, DML 실습에서 만들었던 NETFLIX 데이터를 사용한다.
  
=, IN, NOT IN을 사용해서 데이터를 조회 해 보겠다

```sql
SELECT * FROM NETFLIX WHERE CATEGORY = '애니메이션'; # 카테고리가 애니메이션인 컬럼 조회

SELECT * FROM NETFLIX WHERE CATEGORY IN ('애니메이션', '영화'); # 카테고리가 애니메이션이거나 영화인 컬럼 조회

SELECT * FROM NETFLIX WHERE CATEGORY NOT IN ('애니메이션', '영화'); # 카테고리가 애니메이션, 영화가 아닌 컬럼 조회
```

이번에는 범위를 사용해서 데이터를 조회 해 보겠다.  
각각 70미만, 70 이하의 조회수를 가진 데이터들을 조회한다.

```sql
SELECT * FROM NETFLIX WHERE VIEW_COUNT < 70;

SELECT * FROM NETFLIX WHERE VIEW_COUNT <= 70;
```
이번에는 날짜 DATE를 이용해 데이터를 조회 해 보겠다.  
2023년 전에 등록된 데이터들을 불러온다.
```sql
SELECT * FROM NETFLIX WHERE REGISTER_DATE < TO_DATE('20230101', 'YYYYMMDD');
```

이번에는 AND를 사용하여 다중 조건절을 작성해보겠다.

```sql
SELECT * FROM NETFLIX WHERE CATEGORY = '애니메이션' AND VIEW_COUNT < 70;

SELECT * FROM NETFLIX WHERE CATEGORY = '애니메이션' AND REGISTER_DATE < TO_DATE('20230101','YYYYMMDD');
```

OR을 통해 하나이상 만족하는 데이터도 조회할 수 있다.

```sql
SELECT * FROM NETFLIX WHERE CATEGORY = '애니메이션' OR VIEW_COUNT < 70;

SELECT * FROM NETFLIX WHERE CATEGORY = '애니메이션' OR CATEGORY = '영화'; # 같은 컬럼에도 조건 부여 가능, 위에서 IN 절하고 같은 구문
```

LIKE를 사용해 특정 값을 지정할 수도 있다.

```sql
SELECT * FROM NETFLIX WHERE VIDEO_NAME LIKE '미%'; # 미로 시작하는 비디오 이름을 가지고 있는 데이터

SELECT * FROM NETFLIX WHERE VIDEO_NAME LIKE '%미'; # 미로 끝나는 비디오 이름을 가지고 있는 데이터

SELECT * FROM NETFLIX WHERE VIDEO_NAME LIKE '%미%'; # 미가 값 중간에 있는 비디오 이름을 가지고 있는 데이터
```

범위를 AND로 두번 지정해 조건을 지정할 수 있지만, BETWEEN을 사용해 더욱 간단하게 지정할 수도 있다.

```sql
SELECT * FROM NETFLIX WHERE VIEW_COUNT >= 60 AND VIEW_COUNT <= 70;

SELECT * FROM NETFLIX WHERE VIEW_COUNT BETWEEN 60 AND 70;
```