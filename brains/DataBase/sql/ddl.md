# DDL (CREATE, ALTER, DROP, TRUNCATE) 실습

## CREATE
넷플릭스 테이블을 한번 CREATE 해보겠다.
  
컬럼으로는 영상 이름, 카테고리, 조회수, 영상 등록 날짜가 있다.

```sql
CREATE TABLE NETFLIX(

    VIDEO_NAME VARCHAR2(20),
    CATEGORY VARCHAR2(20),
    VIEW_CNT NUMBER(7),
    REGISTER_DATE DATE
);
```

> ORACLE에서는 문자형을 VARCHAR2로 선언한다.  
> NUMBER에서의 인자는 숫자의 자릿수를 의미한다. 7 > 1,000,000 7자리


## ALTER
등록한 넷플릭스 테이블의 새로운 컬럼 캐스팅 멤버를 추가해보겠다
```sql
ALTER TABLE NETFLIX ADD(CAST_MEMBER VARCHAR2(20))
```

이번에는 캐스팅 멤버의 문자 길이를 50으로 늘려보겠다.
```sql
ALTER TABLE NETFLIX MODIFY(CAST_MEMBER VARCHAR2(50))
```

이번에는 캐스팅 멤버의 타입을 NUMBER로 바꾸어보겠다. (단 이미 등록된 데이터가 문자로 존재하면 오류가 발생한다.)
```sql
ALTER TABLE NETFLIX MODIFY(CAST_MEMBER NUMBER(2))
```

마지막으로 이번에는 캐스팅 멤버의 컬럼을 삭제해보겠다.

```sql
ALTER TABLE NETFLIX DROP(CAST_MEMBER)
```

## DROP, TRUNCATE

DROP은 테이블 자체를 삭제하는 것이고  
TRUCATE는 테이블을 초기화하는 것이다 (로그를 남기지 않기 때문에 데이터 복구가 불가능하다)
  
먼저 테스트용 테이블을 생성해보겠다. 그리고 데이터도 추가하겠다.
```sql
CREATE TABLE TEST_TABLE(
    COL1 VARCHAR2(3),
    COL2 VARCHAR2(3)
);

INSERT INTO TEST_TABLE VALUES('AAA', 'BBB');
INSERT INTO TEST_TABLE VALUES('CCC', 'DDD');

COMMIT;
```

DROP을 하고 TEST_TABLE을 SELECT해보면 어떤 테이블도 조회가 되지 않는다.

```sql
DROP TABLE TEST_TABLE;
SELECT * FROM TEST_TABLE;
```

TRUNCATE를 하고 TEST_TABLE을 SELECT해보면 데이터가 전부 초기화 된다.
```sql
TRUNCATE TABLE TEST_TABLE;
SELECT * FROM TEST_TABLE;
```