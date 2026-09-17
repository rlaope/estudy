# DCL (GRANT, REVOKE) 실습, 사용자 생성

## 사용자 생성

사용자를 두명 생성하고 DB의 영역을 무한대로 준다.
  
`CREATE USER ${USERNAME} IDENTIFIED BY ${PASSWORD}`
```sql
CREATE USER esperer IDENTIFIED BY 'hope';
CREATE USER hope IDENTIFIED BY 'hope2';

GRANT CREATE SESSION, UNLIMITED TABLESPACE TO esperer, hope;
```

### 특정 userId에 부여된 권한 확인
```sql
SHOW GRANTS FOR 'userid'@localhost; (또는 'userid'@'%';)
```

### 특정 userid에 특정 DATABASE의 특정 TABLE에 대한 모든 권한 부여하기

```sql
GRANT ALL ON DATABASE.TABLE TO 'userid'@localhost; (또는 'userid'@'%';)
```

### 특정 권한만 허용하기
```sql
GRANT SELECT, UPDATE ON DATABASE.TABLE TO 'userid'@localhost; (또는 'userid'@'%';)
```

- 옵션 정리
- ALL : 모든 권한
- SELECT, INSERT, UPDATE 등 : 특정 조회, 수정, 추가의 관한 권한
- DATABASE.TABLE: 특정 데이터베이스의 특정 테이블에만 권한을 줄 수 있음 / *.* : 모든 데이터베이스의 모든 테이블에 대한 권한을 부여


## 객체 권한 부여, 해제 예제

### 권한 부여 GRANT

```sql
GRANT [객체권한명] (컬럼)

ON [객체명]

TO { 유저명 | 롤명 | PUBLC} [WITH GRANT OPTION]
```

```sql
GRANT SELECT ,INSERT 
ON TEST_TABLE
TO esperer WITH GRANT OPTION
```

### 권한 해제 REVOKE
```sql
REVOKE { 권한명 [, 권한명...] ALL}

ON 객체명

FROM {유저명 [, 유저명...] | 롤명(ROLE) | PUBLIC} 

[CASCADE CONSTRAINTS]
```

- CASCADE CONSTRAINT :이 명령어의 사용으로 참조 객체 권한에서 사용된 참조 무결성 제한을 같이 삭제 가능
- WITH GRANT OPTION 으로 객체 권한을 부여한 사용자의 객체 권한을 철회하면 권한을 부여받은 사용자가 부여한 객체 권한 또한 같이 철회되는 종속철회가 발생한다.

```sql
REVOKE SELECT , INSERT

ON TEST_TABLE

FROM esperer

[CASCADE CONSTRAINTS]
```