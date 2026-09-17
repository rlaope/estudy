# GROUP BY, Practice Data Aggregation by Group with Aggregate Functions

You can aggregate and view multiple data records using GROUP BY.

Aggregate functions allow you to aggregate various values such as the number of rows, maximum, minimum, average, and standard deviation.

You can output the number of rows using COUNT.
> COUNT(*) aggregates rows with null values, but COUNT(${COLUMN_NAME}) does not.

You can calculate the sum of data for rows that meet each condition using SUM.

```sql
SELECT CATEGORY, COUNT(*) FROM NETFLIX GROUP BY CATEGORY;

SELECT CATEGORY, SUM(*) FROM NETFLIX GROUP BY CATEGORY;
```

You can find the maximum and minimum values for each row's data using MAX and MIN.

```sql
SELECT CATEGORY, MAX(VIEW_COUNT) FROM NETFLIX GROUP BY CATEGORY;

SELECT CATEGORY, MAX(VIEW_COUNT) FROM NETFLIX GROUP BY CATEGORY ORDER BY MAX(VIEW_COUNT) DESC; # You can add sorting conditions using order by

SELECT CATEGORY, MIN(VIEW_COUNT) FROM NETFLIX GROUP BY CATEGORY;

SELECT CATEGORY, MAX(VIEW_COUNT), MIN(VIEW_COUNT) FROM NETFLIX GROUP BY CATEGORY;
```

It's also possible to calculate the average using AVG. + (Reference: STDEV for standard deviation, VAR for variance, STRING_AGG for concatenating columns)

```sql
SELECT CATEGORY, AVG(VIEW_COUNT) FROM NETFLIX GROUP BY CATEGORY;
```
