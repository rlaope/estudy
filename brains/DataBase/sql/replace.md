# REPLACE() 함수로 데이터 수정하기 실습

REPLACE(A, B, C)

A에는 바꿀 데이터의 정보  
B는 A에서 바꿀 문자를 고른다.  
C에서는 B에서 고른 문자를 바꿀 문자를 고른다.  
  
C는 입력이 필수가 아니며 입력하지 않으면 B에서 고른 문자가 NULL로 변환된다.

```sql
SELECT REPLACE('코드라이언', '코드', 'CODE') FROM DUAL; # CODE라이언
SELECT REPLACE('코드라이언','코드') FROM DUAL; # 라이언
SELECT REPLACE('010-1111-1111','-'); 
```

SQL에서는 줄바꿈을 특정 기호로 저장한다. 줄바꿈을 REPLACE를 사용해서 없애고 싶다면.

```sql
SELECT 
'안녕하세요
코드라이언입니다.'
FROM DUAL;

SELECT REPLACE('안녕하세요
코드라이언입니다.', CHR(10), ' ') FROM DUAL;
```

보통 컬럼 값을 변경할 때 이런식으로 사용된다.

```sql
SELECT * FROM NETFLIX_CAST; 
SELECT REPLACE(CAST_MEMBER, '이지은', '아이유') FROM NETFLIX_CAST # 이지은의 데이터를 아이유로 변경
```