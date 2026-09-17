# DDL (데이터 정의어)

- DDL(Data Defintion Language)은 데이터를 정의하는 언어로서 , 보다 엄밀하게 말하면 '데이터를 담는 그릇을 정의하는 언어'이며, 이러한 그릇은 DBMS에서는 오브젝트라고 한다
- DDL을 통해 정의할 수 있는 대상, 오브젝트 유형은 다음과 같다.

DDL 대상 | 설명
--|--
스키마 | DBMS 특성과 구현 환경을 감안한 데이터 구조 , 직관적으로 하나의 데이터베이스로 이해가능.
도메인 | 속성의 데이터 타입과 크기, 제약조건 등을 지정한 정보, 속성히 가질 수 있는 값의 범위로 이해 가능
테이블 | 데이터 저장 공간
뷰 | 하나 이상의 물리 테이블에서 유도되는 가상의 논리 테이블
인덱스 | 검색을 빠르게 하기 위한 데이터 구조

<br>

## DDL 명령어

### CREATE
- 데이터베이스 오브젝트 생성

**신규 생성**
```SQL
CREATE TABLE <table_name> (
  열이름 데이터 타입 [DEFAULT 값][NOT NULL]
  [PRIMARY KEY (열 리스트)]
  {[FOREIGN KEY(열 리스트) REFERENCES 테이블 이름 [(열 이름)]
  [ ON DELETE 옵션]
  [ ON UPDATE 옵션] ]} * 
  [CHECK (조건식) | UNIQUE (열이름)] ]};
)
```

**다른 테이블 정보를 이용한 테이블 생성**
```sql
CREATE TABLE 테이블이름 AS SELECT 문'
```

### ALTER
- 데이터 베이스의 오브젝트 변경

1. 열 추가
```sql
ALTER TABLE 테이블이름 ADD 열이름 데이터 타입
```

2. 열 데이터 타입 변경
```sql
ALTER TABLE 테이블이름 MODIFY 열이름 데이터타입 
```

3. 열 삭제
```sql
ALTER TABLE 테이블이름 DROP 열이름
```

### DROP , TRUNCATE

- `DROP` : 데이터베이스 오브젝트 삭제
- `TRUNCATE` : 데이터베이스 오브젝트 내용 삭제, 테이블 구조는 유지

1. 테이블 삭제
```sql
DROP TABLE 테이블이름
```

2. 테이블 내용 삭제
```sql
TRUNCATE TABLE 테이블이름
```

3. 테이블 이름 변경
```sql
RENAME TABLE 이전 테이블이름 TO 새로운 테이블이름
ALTER TABLE 이전 테이블이름 RENAME 새로운 테이블 이름
```


<br>

## 데이터 타입
- `CHAR` : 고정 길이 문자열 데이터 타입
- `VARCHAR` : 가변 길이 문자열 데이터 타입
- `INT` : 숫자에 사용되는 데이터 타입
- `FLOAT` : 소수형 데이터 타입
- `DATE` : 날짜에 사용되는 데이터 타입