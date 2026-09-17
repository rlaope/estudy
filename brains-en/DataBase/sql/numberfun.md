# Numeric Functions ROUND(), TRUNC(), CEIL() Practice

### ROUND()

Rounds the input data.

```sql
SELECT ROUND(3.16) FROM DUAL; # 3
SELECT ROUND(3.76) FROM DUAL; # 4

SELECT ROUND(3.16, 1) FROM DUAL; # 3.2 If a second argument is provided, it specifies the number of decimal places.
```

### TRUNC()

Truncates all decimal places of the input data.

```sql
SELECT TRUNC(3.16) FROM DUAL; # 3
SELECT TRUNC(3.76) FROM DUAL; # 3

SELECT TRUNC(3.16, 1) # 3.1
```

### CEIL()
Rounds up the input data.

```sql
SELECT CEIL(3.16) FROM DUAL; # 4
SELECT CEIL(3.76) FROM DUAL; # 4
SELECT CEIL(-3.16) FROM DUAL; # 3
```
