# INNER JOIN & OUTER JOIN Practice

## INNER JOIN
> A join that only combines common data. NULLs not allowed.

I will use the Netflix table from the previous practice and the `NETFLIX_CAST` table, which contains Netflix cast members.

```sql
SELECT A.VIDEO_NAME, B.CAST_MEMBER, B.BIRTHDAY
FROM NETFLIX A, NETFLIX_CAST B
WHERE A.VIDEO_NAME = B.VIDEO_NAME;
```

Using type aliases to specify A and B. This will only output rows where A and B's video names are the same. This means if two cast members are in the same video, two results will be returned.

```sql
SELECT A.VIDEO_NAME, B.CAST_MEMBER, B.BIRTHDAY
FROM NETFLIX A, NETFLIX_CAST B
WHERE A.VIDEO_NAME = B.VIDEO_NAME 
AND A.CATEGORY = '예능';
```

If you want to add multiple conditions, just append `AND`.

## OUTER JOIN

There are several types of OUTER JOINs.

1. LEFT OUTER JOIN
2. RIGHT OUTER JOIN
3. FULL OVER JOIN

All values from the specified table are output. This means NULLs are also allowed.
  
I will use LEFT OUTER JOIN. The results below will output all rows that have cast members, and for rows without cast members, NULL will be displayed in the cast member column.
```sql
SELECT A.VIDEO_NAME, A.CATEGORY, B.CAST_MEMBER, B.BIRTHDAY
FROM NETFLIX A
LEFT OUTER JOIN NETFLIX_CAST B
ON A.VIDEO_NAME = B.VIDEO_NAME
```

To achieve the same results as an INNER JOIN using an OUTER JOIN:

```sql
SELECT A.VIDEO_NAME, A.CATEGORY, B.CAST_MEMBER, B.BIRTHDAY
FROM NETFLIX A
LEFT OUTER JOIN NETFLIX_CAST B
ON A.VIDEO_NAME = B.VIDEO_NAME
WHERE B.CAST_MEMBER IS NOT NULL

SELECT A.VIDEO_NAME, A.CATEGORY, B.CAST_MEMBER, B.BIRTHDAY
FROM NETFLIX A, NETFLIX_CAST B
WHERE A.VIDEO_NAME = B.VIDEO_NAME
```

The query above and the query below yield identical results.
