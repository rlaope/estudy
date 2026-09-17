# SQL 기초 문법 정리

## 테이블 생성
books 라는 이름을 가지고 있으며 id, title, writer , quantity의 컬럼을 가지고 있는 테이블을 생성해보자.
```sql
CREATE TABLE books
(id INTEGER PRIMARY KEY
  title TEXT,
  writer TEXT,
  released_year INTEGER);
```

## 데이터 입력하기

위에서 만든 books 테이블에 데이터를 입력하자 입력하는 방식은 두 가지다.
```sql
INSERT INTO books VALUES(1,"title" , "hope" , 2022);
INSERT INTO books VALUES(2,"titile","wow",2001)
```
이렇게 모든 값을 입력해주기는 귀찮은 일이다. 가끔 몇몇 값은 빈값으로 둬야 할 때도 있다. 테이블을 생성할 때 id값에 autoincrement 설정을 두면 데이터를 입력할 때 id 값을 일일이 입력하지 않아도 저절로 값이 증가하면서 입력된다.

```sql
CREATE TABLE books
(id INTEGER PRIMARY KEY AUTOINCREMENT
  title TEXT, 
  writer TEXT, 
  released_year INTEGER);
```
컬럼명을 지정하면 입력하지 않은 값은 `NULL` 처리가 된다.

<br>

## 데이터 불러오기
데이터를 불러올 떄는 SELECT 문을 쓰는데 순서를 지켜야한다
- 작성 순서
```
SELECT - FROM - WHERE - GROUP BY - HAVING - ORDER BY
```
- 실행 순서
```
FROM - WHERE - GROUP BY - HAVING - SELECT - ORDER BY
```

### SELECT  
가져오고 싶은 COLUMN 명을 입력한다
column 명은 변경이 가능하고, 기존에 없던 데이터를 추가할 수도 있다.
```sql
SELECT '2005' year, HOUR(datetime) HOUR, count(hour(datetime)) COUNT
```
이 경우는 year이라는 컬럼이 기존에 없더라도 추가가 가능하며 '2005' 라는 데이터기 일괄적으로 들어간다. 그리고 hour(datetime)뒤에 넣고싶은 컬럼명을 설정하면된다. "HOUR" 이라고 표시할 수도 있다.

### FROM
from 뒤에는 값을 가져오고 싶은 테이블 명을 입력한다.

### WHERE
집계함수를 사용할 수 없다.
- 필드 값이 있는지 확인하고 싶을때는 IS NULL , IS NOT NULL 을 사용하면 된다.
  
```sql
SELECT * FROM books WHERE released_year IS NULL;
```
출판년도가 NULL인 데이터만 가져온다

### GROUP BY
데이터를 그룹화하여 가져온다.

### HAVING
group by 절과 함께 사용되며, 집계함수로 조건비교를 할 수 있다.

### ORDER BY
데이터 정렬 기준. 여러개의 기준을 세울수도 있고, 역순으로 배열하는것도 가능하다.