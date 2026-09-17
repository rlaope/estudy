# SQL 날짜(DATE) 함수를 사용한 실습 + DUAL 테이블

## DUAL 테이블
sql 에서 지원해주는 가상의 테이블이다.
  
아래와 같이 현재 시간을 가져올 수 있다.  
```sql
SELECT SYSDATE FROM DUAL;
```

현재 시간에서 n개월 후, 전을 구하는 방법은
```sql
SELECT ADD_MONTHS(SYSDATE, 3) FROM DUAL; # 3개월 후
SELECT ADD_MONTHS(SYSDATE, -3) FROM DUAL; # 3개월 전
```

시간 빼고 년도만 구하는 방법
```sql
SELECT TRUNC(SYSDATE) FROM DUAL;
```

TO_CHAR를 사용해 지정된 포멧 형식으로 DATE 출력

```sql
SELECT TO_CHAR(SYSDATE, 'YYYY-MM-DD HH24:MI:SS') FROM DUAL
```