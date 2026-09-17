# UPPER(), LOWER() 함수를 사용한 대소문자 구분 실습

### UPPER()
입력받은 문자 데이터를 모두 대문자로 변환

```sql
SELECT UPPER('CodeLion') FROM DUAL; # CODELION
```

### LOWER()
입력받은 문자 데이터를 모두 소문자로 변환
```sql
SELECT LOWER('CodeLion') FROM DUAL; # codelion
```

대소문자를 구분하지 않고 아이디가 codelion인 멤버를 찾기

```sql
SELECT * FROM MEMBER WHERE ID = UPPER('codelion')
```