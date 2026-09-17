# ORDER BY로 데이터 정렬하기 실습

ORDER BY를 통해서 데이터를 정렬된 상태로 조회할 수 있다.  
  
`ASC`: 오름차순 정렬으로 기본 값이다
`DESC`: 내림차순 정렬으로 지정을 해주어야한다.
  
조회수 오름차순으로 데이터를 조회한다.  
아래의 두 구문은 같은 동작을 한다.
```sql
SELECT * FROM NETFLIX ORDER BY VIEW_COUNT ASC;
SELECT * FROM NETFLIX ORDER BY VIEW_COUNT;
```

내림차순으로 데이터를 조회한다.
```sql
SELECT * FROM NETFLIX ORDER BY VIEW_COUNT DESC;
```

여러 컬럼에 정렬 조건을 줘 조회할 수 있다.

```sql
SELECT * FROM NETFLIX ORDER BY VIDEO_NAME, VIEW_COUNT DESC;
```

비디오 제목을 기준으로 조회수를 내림차순으로 정렬하여 조회한다.

