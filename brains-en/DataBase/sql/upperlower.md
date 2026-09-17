# Case Sensitivity Practice Using UPPER() and LOWER() Functions

### UPPER()
Converts all input character data to uppercase.

```sql
SELECT UPPER('CodeLion') FROM DUAL; # CODELION
```

### LOWER()
Converts all input character data to lowercase.
```sql
SELECT LOWER('CodeLion') FROM DUAL; # codelion
```

Find members whose ID is 'codelion', case-insensitively.

```sql
SELECT * FROM MEMBER WHERE ID = UPPER('codelion')
```
