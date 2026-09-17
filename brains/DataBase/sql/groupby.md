# GROUP BY, 집계함수로 데이터 그룹별 집계 실습

GROUP BY를 통해 여러 데이터를 집계해서 확인할 수 있다.  
  
집계 함수를 통해 행의 개수, 최대,최소값, 평균, 표춘편차등 여러 값들을 집계할 수 있다.
  
COUNT를 통해 행의 개수를 출력할 수 있다.
> COUNT(*)은 null 값의 행을 집계하지만 COUNT(${COLUMN_NAME})은 집계하지 않는다.

SUM을 통해 각 조건에 맞는 행들의 데이터의 합을 구할 수 있다.

```sql
SELECT CATEGORY, COUNT(*) FROM NETFLIX GROUP BY CATEGORY;

SELECT CATEGORY, SUM(*) FROM NETFLIX GROUP BY CATEGORY;
```

MAX,MIN을 통해 각 행의 값들의 최댓값과 최솟값을 구할 수 있다.

```sql
SELECT CATEGORY, MAX(VIEW_COUNT) FROM NETFLIX GROUP BY CATEGORY;

SELECT CATEGORY, MAX(VIEW_COUNT) FROM NETFLIX GROUP BY CATEGORY ORDER BY MAX(VIEW_COUNT) DESC; # order by 를 사용해 정렬 조건을 추가 가능

SELECT CATEGORY, MIN(VIEW_COUNT) FROM NETFLIX GROUP BY CATEGORY;

SELECT CATEGORY, MAX(VIEW_COUNT), MIN(VIEW_COUNT) FROM NETFLIX GROUP BY CATEGORY;
```

AVG를 통해 평균을 구하는 것도 가능하다. + (참고: STDEV 표준편차 VAR 분산, STRING_AGG 컬럼 합치기)

```sql
SELECT CATEGORY, AVG(VIEW_COUNT) FROM NETFLIX GROUP BY CATEGORY;
```


