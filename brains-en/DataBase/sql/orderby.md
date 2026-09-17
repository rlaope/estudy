# Practicing Data Sorting with ORDER BY

You can retrieve data in a sorted state using ORDER BY.

`ASC`: Ascending order, which is the default value.
`DESC`: Descending order, which must be specified.

Retrieve data in ascending order of view count.
The two statements below perform the same operation.
```sql
SELECT * FROM NETFLIX ORDER BY VIEW_COUNT ASC;
SELECT * FROM NETFLIX ORDER BY VIEW_COUNT;
```

Retrieve data in descending order.
```sql
SELECT * FROM NETFLIX ORDER BY VIEW_COUNT DESC;
```

You can retrieve data by applying sort conditions to multiple columns.

```sql
SELECT * FROM NETFLIX ORDER BY VIDEO_NAME, VIEW_COUNT DESC;
```

Retrieve data sorted by video title, then by view count in descending order.
