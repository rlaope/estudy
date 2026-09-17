# DML(INSERT, UPDATE, DELETE, SELECT) 실습 + TCL(COMMIT, ROLLBACK)

> 이전 DDL 실습에서 만들었던 NETFLIX 테이블을 사용해 실습을 진행하겠다.

## INSERT

INSERT문으로 데이터를 삽입해보겠다.

```sql
INSERT INTO NETFLIX VALUES('나의 아저씨', '드라마', 30, SYSDATE); 

COMMIT;
```

SYSDATE는 프로그램 환경에서 현재 시간을 의미한다.  
COMMIT은 저장한 데이터를 영구적으로 반영한다 (영속화) 한다는 명령어이다.
  
계속해서 진행해보겠다. 이번에는 특정 컬럼에만 데이터를 추가해보겠다.  
그리고 또 오늘로부터 30일전, 40일전, 300일전에 등록된 영상 데이터들의 새로운 데이터들을 추가해보겠다
```sql
INSERT INTO NETFLIX (VIDEO_NAME, VIEW_COUNT) VALUES('시그널', 42)

INSERT INTO NETFLIX VALUES('응답하라 1988', '드라마', 42, SYSDATE - 30); 
INSERT INTO NETFLIX VALUES('이태원 클라쓰', '드라마', 32, SYSDATE - 40); 
INSERT INTO NETFLIX VALUES('미스터 션샤인', '드라마', 22, SYSDATE - 300); 

COMMIT;

ROLLBACK;
```

만약 저 데이터들을 인서트하고 ROLLBACK을 입력한다면 다시 데이터가 나의 아저씨만 남게 될것이다. (최근 커밋을 롤백함)

## UPDATE
위에 인서트문에서 시그널은 카테고리와 등록 날짜가 NULL로 되어있다.  
이번에는 컬럼의 값들을 수정해볼 것인데 일단 먼저 나의 아저씨 조회수를 수정해보겠다.

```sql
UPDATE NETFLIX SET VIEW_COUNT = 70 WHERE VIDEO_NAME = '나의 아저씨';

COMMIT;
```

이어서 시그널에 카테고리와 등록 날짜를 추가해보겠다.

```sql
UPDATE NETFLIX SET CATEGORY = "드라마", REGISTER_DATE = TO_DATE('20230101', 'YYYYMMDD') =  WHERE VIDEO_NAME = '시그널';

COMMIT;
```

TO_DATE() 함수는 날짜를 입력받고 (첫번째 인자) 지정한 포멧 형식(두번째 인자)으로 변환해 DATE형으로 만든다. 

## DELETE 
TRUNCATE와 DELETE의 차이

1. TRUNCATE는 모든 데이터를 삭제하기만 하지만 DELETE는 선택해서 삭제도 가능하다.
2. TRUNCATE는 데이터 롤백이 불가능하지만 DELETE는 롤백이 가능하다.
3. 모든 데이터를 삭제할 때 성능은 TRUNCATE가 더 빠르다.
  
먼저 미스터 션샤인의 데이터를 삭제해보도록 하겠다.
```sql
DELETE FROM NETFLIX WHERE VIDEO_NAME = '미스터 션샤인';

COMMIT;
```

이번에는 드라마이면서 조회수가 35미만인 데이터들을 삭제하겠다.
```sql
DELETE FROM NETFLIX WHERE CATEGORY = '드라마' AND VIEW_COUNT < 35;

COMMIT;
```

IN 조건절을 사용해서 데이터를 삭제하겠다.
```sql
DELETE FROM NETFLIX WHERE VIDEO_NAME IN('이태원 클라쓰','나의 아저씨');
```

마지막으로 데이터 전체 삭제를 DELETE를 사용해서 해보겠다.
```sql
DELETE * FROM NETFLIX;
```

## SELECT
SELECT만 할 줄 알아도 sql의 절반을 안 것이라고 볼 수 있다. 더 많은 내용을 배워야하지만 이번에는 간단한 실습만 진행 해 보겠다.
  
테이블의 컬럼을 모두 조회한다. 두개의 쿼리는 같은 의미다.
```sql
SELECT * FROM NETFLIX

SELECT VIDEO_NAME, VIEW_COUNT, CATEGORY, REGISTER_DATE FROM NETFLIX;
```

원하는 컬럼의 데이터들만 조회할 수 있다. SELECT FROM 사이에는 무조건 하나 이상의 컬럼이 들어있어야 한다. (없으면 오류발생)
```sql
SELECT VIDEO_NAME, VIEW_COUNT, CATEGORY FROM NETFLIX;
SELECT VIDEO_NAME, VIEW_COUNT FROM NETFLIX;
SELECT VIDEO_NAME FROM NETFLIX;
```

이번에는 조건절을 사용해서 조회 해 보겠다.  
비디오 제목이 나의 아저씨인 것과 나이 아저씨가 아닌 것들을 조회해보겠다.
```sql
SELECT * FROM NETFLIX WHERE VIDEO_NAME = '나의 아저씨';

SELECT * FROM NETFLIX WHERE VIDEO_NAME <> '나의 아저씨';
```

등록 날짜가 최근으로 부터 한달동안안에 등록되어있는 데이터들을 조회하겠다.
```sql
SELECT * FROM NETFLIX WHERE REGISTER_DATE > SYSDATE - 30;
SELECT * FROM NETFLIX WHERE REGISTER_DATE < SYSDATE - 30; # 이건 반대로 한달 전
``` 

DISTINCT를 붙여주면 중복을 제거한다 (가장 먼저 조회된 데이터를 반환)
```sql
SELECT DISTINCT CATEGORY FROM NETFLIX;
```