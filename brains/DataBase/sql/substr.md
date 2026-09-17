# SUBSTR() 함수를 사용한 문자 데이터 자르기 실습

SUBSTR을 사용해서 문자 데이터를 자를 수 있다.
  
SUBSTR(문자, N, M)  
N은 앞에서부터 N번째 문자를 가져오고 M은 N부터 M까지를 잘라오겠다는 뜻이다.  
M은 생략할 수 있고 생략한다면 N부터 문자 끝까지 가져온다.

```sql
SELECT SUBSTR('코드라이언',3,2) FROM DUAL; # 라이 
SELECT SUBSTR('코드라이언',3) FROM DUAL; # 라이언

SELECT SUBSTR('코드라이언', -4) FROM DUAL; # 드라이언, 음수가 들어가면 앞에서부터가 아닌 뒤에서부터 N번째로 읽는다.
```

1. SUBSTR과 NETFLIX CAST 테이블을 사용해서 캐스트 멤버의 이름 가운데를 *로 대체해보겠다.
2. 긴 문자의 기사를 전부 출력하지말고 20번째 까지만 출력한 후 ....으로 대체해보겠다.

```sql
SELECT SUBSTR(CAST_MEMBER,1,1) || '*' || SUBSTR(CAST_MEMBER,3) FROM NETFLIX_CAST;

SELECT SUBSTR('어쨋든 겁나게 긴 문자 입니다 안녕하세요 저의 이름은 긴 문자입니다 실습을 위해서 조금 길게 써보겠습ㄴ.', 1, 20) || '......' FROM DUAL;
```

