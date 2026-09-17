# VARCHAR vs TEXT

오늘은 MySQL에서 TEXT와 VARCHAR에 대해서 알아보고 차이점도 함께 보겠습니다.

### VARCHAR

VARCHAR를 통해 컬럼 타입을 지정해준다고 했을때

VARCHAR(10), VARCHAR(1000) 이렇게 각각 다르게 지정해주었을 때

성능상의 차이점이 있을까요? 그냥 다 크게 잡아버리면 안될까요?

한 번 VARCHAR를 엄청 길게 잡고 테이블을 생성해보면

```mysql
mysql> CREATE TABLE tb_long_varchar (id INT PRIMARY KEY, fd1 VARCHAR(1000000));  
ERROR 1074 (42000): Column length too big for column 'fd1' (max = 16383); use BLOB or TEXT instead

mysql> CREATE TABLE tb_long_varchar (id INT PRIMARY KEY, fd1 VARCHAR(16383));  
ERROR 1118 (42000): Row size too large. The maximum row size for the used table type, not counting BLOBs, is 65535. This includes storage overhead, check the manual. You have to change some columns to TEXT or BLOBs  
  
mysql> CREATE TABLE tb_long_varchar (id INT PRIMARY KEY, fd VARCHAR(16382));  
Query OK, 0 rows affected (0.19 sec)  
  
mysql> ALTER TABLE tb_long_varchar ADD fd2 VARCHAR(10);  
ERROR 1118 (42000): Row size too large. The maximum row size for the used table type, not counting BLOBs, is 65535. This includes storage overhead, check the manual. You have to change some columns to TEXT or BLOBs
```

위와같이 보면 테이블에 하나의 레코드가 저장할 수 있는 최대길이(65,535byte)를 초과했기 때문에 테이블 생성, 변경이 안되는 것을 확인할 수 있습니다

4번째 줄에서 VARCHAR 컬럼이 너무 큰 길이를 사용하면 다른 컬럼들이 사용할 수 있는 최대 공간의 크기가 영향을 받게 된다는 것을 확인했어요.

그래서 VARCHAR 타입의 길이는 최대 저장 길이 설정시 공간을 아껴서 사용을 해야합니다.

<br>

### 여기서 TEXT에 관한 의문

그럼 여기서 의문이 있어요 TEXT와 같은 LOB(Large Object) 타입의 컬럼은 길이 제한 사항에 영향을 받지 않아요. 그래서 많은 컬럼을 가진 테이블에서는 TEXT 타입을 사용해야할 수도 있죠

그런데 VARCHAR 타입의 길이 설정에 주의해야하는 이유는 이거 하나뿐일까요? 추가로 새로운 컬럼이 필요하지 않아 아래와 같이 설정했다면요?

```mysql
CREATE TABLE user (  
id BIGINT NOT NULL,  
name VARCHAR(4000),  
phone_no VARCHAR(4000),  
address VARCHAR(4000),  
email VARCHAR(4000),  
PRIMARY KEY(id)  
);
```

TEXT를 사용해서 테이블을 설계하면 어래처럼 제한 문제가 싹 살아지게되어요 길이 설정 제약 뿐만 아니라 저장하는 값의 길이 제한도 훨씬 크고 유연하게 테이블을 만들 수 있어요

```mysql
CREATE TABLE user (  
id BIGINT NOT NULL,  
name TEXT,  
phone_no TEXT,  
address TEXT,  
email TEXT,  
PRIMARY KEY(id)  
);
```

그럼 여기서 궁금증이 생길 수 있겠죠, 왜 VARCHAR 쓰는걸까? 길이도 유연하게 지정못하고 확장성도 낮고 TEXT가 저장 가능 길이도 훨신 큰데 말이죠..

이제 한 번 알아보도록 할게요

<br>

## VARCHAR vs TEXT

### 일반적인 RDBMS

일반적인 RDBMS에서는 TEXT와 같은 LOB(대용량 데이터를 저장하는 타입 Large Object) 타입의 데이터를 `Off-Page`라고 하는 외부 저장소에 데이터를 보관해요

그리고 일반적인 길이의 레코드의 컬럼 데이터는 `B-Tree(Clustering Index)`에 저장해요(inline 저장)

### MySQL

하지만 MySQL 서버는 LOB 타입의 컬럼을 항상 `Off-Page`로 저장하지는 않고 길이가 길어서 저장 공간이 많이 필요한 경우에만 그곳에 저장해요

```mysql
CREATE TABLE tb_lob (  
id INT PRIMARY KEY,  
fd TEXT  
);  
  
INSERT INTO tb_lob VALUES (1, REPEAT('A',8100)); -- // Inline 저장소  
INSERT INTO tb_lob VALUES (2, REPEAT('A',8101)); -- // Off-Page 저장소
```

위와 같이 1번 데이터는 8100 글자(8100byte)를 저장하면 B-Tree에 데이터를 저장하고
2번 데이터는 8101글자(8101byte)를 저장하면 Off-Page애 데이터를 저장해요

이것은 MySQL 서버 레코드 포멧에 따라서 조금씩 다르게 동작하기도 해요

> MySQL 서버의 레코드 크기 제한은 65,535 바이트지만 InnoDB 스토리지 엔진의 레코드 크기 제한은 페이지(블록) 크기에 따라서 달라집니다, 대부분 페이지 크기에 절반이 InnoDB 스토리지 엔진의 최대 레코드 크기 제한으로 작동해요 (위에 예시에서는 제한사항 16KB페이지에서는, 8117바이트를 초과하면 길이가 긴 컬럼을 선택해 Off-Page에 저장합니다.)

그럼 TEXT만 길이가 길 때 Off-Page에 저장이 되는 것일까요?

그건 아닙니다. VARCHAR도 마찬가지로 컬럼에 저장된 레코드 값이 크면 Off-Page로 저장이 되어요

```mysql
CREATE TABLE tb_varchar (  
id INT PRIMARY KEY,  
fd VARCHAR  
);  
  
INSERT INTO tb_varchar VALUES (1, REPEAT('A',8100)); -- // Inline 저장소  
INSERT INTO tb_varchar VALUES (2, REPEAT('A',8101)); -- // Off-Page 저장소
```


### 인덱스에 관한 오해

VARCHAR 타입은 인덱스를 생성할 수 있지만 LOB 타입은 생성할 수 없다는 오해도 있는데, 이건 잘못된 오해입니다. LOB 타입도 데이터의 길이 제한만 충족시키면 인덱스를 생성할 수 있어요

```mysql
-- // 컬럼 그대로 사용시, 인덱스 생성 불가  
mysql> ALTER TABLE tb_varchar ADD INDEX ix_fd (fd);  
ERROR 1071 (42000): Specified key was too long; max key length is 3072 bytes  
  
mysql> ALTER TABLE tb_lob ADD INDEX ix_fd (fd);  
ERROR 1170 (42000): BLOB/TEXT column 'fd' used in key specification without a key length  
  
-- // 컬럼 값의 길이(프리픽스)를 지정하면, 인덱스 생성 가능  
mysql> ALTER TABLE tb_varchar ADD INDEX ix_fd ( fd(50) );  
mysql> ALTER TABLE tb_lob ADD INDEX ix_fd ( fd(50) );
```

위와 같이 길이 프리픽스를 지정해주니 인덱스 생성이 되는 것을 확인할 수 있어요


결국 Off-Page 저장도 그렇고 , B-Tree 인덱스도 생성이 가능하면 도대체 어떤 차이가 있을까요? 선언의 차이? 오히려 더 차이가 애매모호해지는 것 같아요. 그럼 도대체 어떤 상황에 사용해야할까요?

<br>

## VARCHAR, TEXT 메모리 활용

MySQL 서버는 스토리지 엔진과 Handler API를 통해서 데이터를 주고받습니다.

이때 MySQL 엔진과 스토리지 엔진은 `uchar *records[2]` 메모리 포인터를 이용해서 데이터를 주고받습니다. 이때, `records[2]` 라는 메모리 객체는 데이터의 길이에 상관없이 항상 최대 크기로 메모리를 할당해줍니다.

여기서 VARCHAR는 길이가 지정되어 있기 때문에 미리 버퍼에 데이터를 할당받을 수 있어요

그러나 TEXT와 같은 LOB 타입은 실제 최대 크기만큼 메모리를 할당해두면 메모리 낭비가 너무 심해지는 문제가 있습니다. 그래서 `record[2]`가 가르키는 메모리 공간은 VARCHAR는 포함하지만 TEXT 컬럼을 위한 공간은 포함하지 않아요.

`uchar *records[2]` 메모리 공간은 TABLE의 구조체 내에서 정의되어 있으면서 TABLE 구조체 **MySQL 서버 내부에 캐싱되어서 여러 커넥션에서 공유해서 사용될 수 있도록 구현되어 있어요.**

즉 `record[2]` 메모리 버퍼는 처음 한번 할당되면 많은 커넥션들에 의해서 재사용될 수 있도록 설계되어 있어요

그러나 TEXT, LOB 컬럼을 위한 메모리 공간은 `records[2]` 에 미리 할당되어 있지 않기 때문에 매번 레코드를 읽을때마다 필요한 만큼 메모리가 할당되어야해요

```mysql
CREATE TABLE tb_lob (  
id INT PRIMARY KEY,  
fd TEXT  
);  
  
CREATE TABLE tb_varchar1 (  
id INT PRIMARY KEY,  
fd VARCHAR(100)  
);  
  
CREATE TABLE tb_varchar2 (  
id INT PRIMARY KEY,  
fd VARCHAR(10000)  
);
```

예를 들어서 위와같이 테이블을 생성했다면, tb_lob 테이블을 위한 `record[2]` 버퍼 공간은 16x2 바이트만큼 할당되고, tb_varchar 1, 2는 각각 408 x 2, 40008 x 2 바이트를 할당해요

- `tb_lob` 테이블은 `INT` 타입의 컬럼(id)을 위한 4 바이트와 `TEXT` 값을 위한 포인터 공간 8바이트 그리고 헤더 공간 4바이트
- `tb_varchar1` 테이블은 `INT` 타입의 컬럼(id)을 위한 4 바이트와 `VARCHAR(100)`타입 컬럼을 위한 공간 400바이트 그리고 헤더 공간 4바이트
- `tb_varchar2` 테이블은 `INT` 타입의 컬럼(id)을 위한 4 바이트와 `VARCHAR(10000)` 타입 컬럼을 위한 공간 40000바이트 그리고 헤더 공간 4바이트

그래서 VARCHAR 타입의 데이터를 읽을때는 새롭게 메모리를 할당받는 것이 아닌 `records[2]` 버퍼를 이용해요, 그러나 TEXT는 미리 할당해둔 메모리 공간이 없기 때문에 매번 필요한 만큼 메모리를 할당해서 사용한 후 해제합니다.

LOB 컬럼의 값을 읽기 위해서 할당 및 해제하는 메모리 공간은 `Performance_schema`에 의해서 측정되지 않아요. 그래서 성능 영향도도 파악하기가 어렵스빈다.

> 추가로 주의해야할 점은 VARCHAR도 off-page에 저장될 정도로 길이가 길다면 records\[2]의 버퍼를 사용하지 못합니다. 그래서 주의를 해야해요

결국 컬럼타입의 선정 규칙을 요약해보면 다음과 같을 수 있어요

### VARCHAR
- 최대 길이가 **상대적으로** 크지 않은 경우
- 테이블 데이터를 읽을 때 항상 해당 컬럼이 필요한 경우
- DBMS 서버의 메모리가 **상대적으로** 충분한경우

### TEXT
- 최대 길이가 **상대적으로** 큰 경우
- 테이블에 길이가 긴 문자열 타입 컬럼이 많이 필요한 경우
- 테이블 데이터를 읽을 때 해당 컬럼이 자주 필요치 않은 경우

위에서 상대적이라는 말을 강조했는데, DBMS의 서버 스펙이나 데이터 모델 그리고 유입되는 트래픽에 따라서 미치는 영향도가 다르기때문이에요 그래서 모든 판단 기준을 하나로 정의하는 것은 힘들긴합니다. 잘 분석해서 설계하셨으면 좋겠어요