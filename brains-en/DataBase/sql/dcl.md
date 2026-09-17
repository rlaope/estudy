# DCL (GRANT, REVOKE) Practice, User Creation

## User Creation

Create two users and grant them unlimited database space.
  
`CREATE USER ${USERNAME} IDENTIFIED BY ${PASSWORD}`
```sql
CREATE USER esperer IDENTIFIED BY 'hope';
CREATE USER hope IDENTIFIED BY 'hope2';

GRANT CREATE SESSION, UNLIMITED TABLESPACE TO esperer, hope;
```

### Check Permissions Granted to a Specific userId
```sql
SHOW GRANTS FOR 'userid'@localhost; (또는 'userid'@'%';)
```

### Grant All Permissions on a Specific TABLE in a Specific DATABASE to a Specific userid

```sql
GRANT ALL ON DATABASE.TABLE TO 'userid'@localhost; (또는 'userid'@'%';)
```

### Allow Only Specific Permissions
```sql
GRANT SELECT, UPDATE ON DATABASE.TABLE TO 'userid'@localhost; (또는 'userid'@'%';)
```

- Option Summary
- ALL: All permissions
- SELECT, INSERT, UPDATE, etc.: Permissions for specific queries, modifications, and additions
- DATABASE.TABLE: Can grant permissions only on a specific table in a specific database / *.*: Grants permissions on all tables in all databases


## Object Permission Grant and Revoke Examples

### Granting Permissions (GRANT)

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

### Revoking Permissions (REVOKE)
```sql
REVOKE { 권한명 [, 권한명...] ALL}

ON 객체명

FROM {유저명 [, 유저명...] | 롤명(ROLE) | PUBLIC} 

[CASCADE CONSTRAINTS]
```

- CASCADE CONSTRAINT: Using this command can also delete referential integrity constraints used in referenced object permissions.
- If you revoke object permissions from a user who was granted them WITH GRANT OPTION, a cascading revocation occurs, meaning the object permissions granted by that user are also revoked.

```sql
REVOKE SELECT , INSERT

ON TEST_TABLE

FROM esperer

[CASCADE CONSTRAINTS]
```
