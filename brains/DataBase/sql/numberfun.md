# 숫자함수 ROUND(), TRUNC(), CEIL() 실습

### ROUND()

입력한 데이터의 반올림을 한다.

```sql
SELECT ROUND(3.16) FROM DUAL; # 3
SELECT ROUND(3.76) FROM DUAL; # 4

SELECT ROUND(3.16, 1) FROM DUAL; # 3.2 두번째 인자가 있으면 그 인자의 소숫점 자릿수까지 출력함
```

### TRUNC()

입력받은 데이터의 소숫점 자리수를 다 버림 

```sql
SELECT TRUNC(3.16) FROM DUAL; # 3
SELECT TRUNC(3.76) FROM DUAL; # 3

SELECT TRUNC(3.16, 1) # 3.1
```

### CEIL()
입력받은 데이터를 올림

```sql
SELECT CEIL(3.16) FROM DUAL; # 4
SELECT CEIL(3.76) FROM DUAL; # 4
SELECT CEIL(-3.16) FROM DUAL; # 3
```