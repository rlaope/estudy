# Practice Truncating Character Data Using the SUBSTR() Function

You can truncate character data using SUBSTR.

SUBSTR(string, N, M)
N means to get the Nth character from the beginning, and M means to truncate from N up to M.
M can be omitted; if omitted, it retrieves from N to the end of the string.

```sql
SELECT SUBSTR('코드라이언',3,2) FROM DUAL; # 라이 
SELECT SUBSTR('코드라이언',3) FROM DUAL; # 라이언

SELECT SUBSTR('코드라이언', -4) FROM DUAL; # 드라이언, 음수가 들어가면 앞에서부터가 아닌 뒤에서부터 N번째로 읽는다.
```

1. I will use SUBSTR and the NETFLIX CAST table to replace the middle of cast members' names with an asterisk (*).
2. Instead of printing the entire long article text, I will print only up to the 20th character and then replace the rest with '....'.

```sql
SELECT SUBSTR(CAST_MEMBER,1,1) || '*' || SUBSTR(CAST_MEMBER,3) FROM NETFLIX_CAST;

SELECT SUBSTR('어쨋든 겁나게 긴 문자 입니다 안녕하세요 저의 이름은 긴 문자입니다 실습을 위해서 조금 길게 써보겠습ㄴ.', 1, 20) || '......' FROM DUAL;
```
