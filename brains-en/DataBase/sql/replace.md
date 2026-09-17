# Practice Modifying Data with REPLACE() Function

REPLACE(A, B, C)

A contains the data to be modified.
B selects the characters to be replaced within A.
C selects the characters to replace the ones chosen in B.
  
C is not a mandatory input; if omitted, the characters selected in B will be converted to NULL.

```sql
SELECT REPLACE('코드라이언', '코드', 'CODE') FROM DUAL; # CODE라이언
SELECT REPLACE('코드라이언','코드') FROM DUAL; # 라이언
SELECT REPLACE('010-1111-1111','-'); 
```

In SQL, newlines are stored as specific characters. If you want to remove newlines using REPLACE.

```sql
SELECT 
'안녕하세요
코드라이언입니다.'
FROM DUAL;

SELECT REPLACE('안녕하세요
코드라이언입니다.', CHR(10), ' ') FROM DUAL;
```

It is typically used in this way when changing column values.

```sql
SELECT * FROM NETFLIX_CAST; 
SELECT REPLACE(CAST_MEMBER, '이지은', '아이유') FROM NETFLIX_CAST # 이지은의 데이터를 아이유로 변경
```
