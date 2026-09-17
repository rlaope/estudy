# Practice Using SQL DATE Functions + DUAL Table

## DUAL Table
It is a virtual table provided by SQL.

You can get the current time as shown below.
```sql
SELECT SYSDATE FROM DUAL;
```

To get n months after or before the current time:
```sql
SELECT ADD_MONTHS(SYSDATE, 3) FROM DUAL; # 3 months after
SELECT ADD_MONTHS(SYSDATE, -3) FROM DUAL; # 3 months before
```

How to get only the year, excluding the time
```sql
SELECT TRUNC(SYSDATE) FROM DUAL;
```

Output DATE in the specified format using TO_CHAR

```sql
SELECT TO_CHAR(SYSDATE, 'YYYY-MM-DD HH24:MI:SS') FROM DUAL
```
