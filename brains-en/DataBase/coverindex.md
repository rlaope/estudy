# Covering Index

A covering index refers to an **index that contains all the data necessary to satisfy a query**.

An index is a method for efficiently searching data. When querying using an index, there's no need to access the actual data. In other words, an index improves query performance.

> Satisfying a query -> When all columns used in SELECT, WHERE, ORDER BY, LIMIT, GROUP BY, etc., are included within the index columns.

### Example
```SQL
SELECT * FROM user WHERE id = 1;
```

![](https://velog.velcdn.com/images/boo105/post/84e67cc0-efbf-4239-86de-d06d0f0a778e/image.png)

The above is the execution plan for the query.

- `selected_type`: SIMPLE indicates a simple SELECT statement without UNIONs or subqueries.
- `type`: When a `ref` join maps to a key that is not a PK or Unique Key. Here, since it's a single table query, it refers to searching with an equality condition.
- `key`: The index used in the query. Here, it appears as PRIMARY because the PK was used.
- `ref`: Indicates which value was used as a `const` comparison condition. Here, you can see `const` because the constant value 1 was used.
- `extra`: If empty, it's a general query.

Now, let's change the query again. This time, instead of fetching everything, I'll fetch only the `id`.

```sql
select id from user
where id = 1;
```

![](https://velog.velcdn.com/images/boo105/post/74a512ba-ef27-4570-8d86-aac8804f3572/image.png)

You can see that the value of Extra has changed to `Using index`. This query uses a covering index because the query can be satisfied solely by the `id` column, which is included in the index.

**In other words, a covering index is applied because all columns appearing in the query are part of the index.**

<br>

### GROUP BY

In the example, we only checked `WHERE`. How can we apply a covering index to `GROUP BY` as well?

Assuming the index columns are a, b, c, the `GROUP BY` index conditions must satisfy the following:

1. The order of columns specified in `GROUP BY` must be the same.
2. The preceding columns must always be specified, while subsequent columns can be omitted.
3. Columns not in the index must not be specified.
4. When `WHERE` + `GROUP BY` are used together, columns in `WHERE` do not need to be in `GROUP BY`.

```sql
GROUP BY a, c, b        # Index not applied
GROUP BY a, b, c        # Index applied
```

```sql
GROUP BY a              # Index applied
GROUP BY a,b            # Index applied
```

```sql
GROUP BY a, b, c, d     # Index not applied
```

```sql
WHERE a = 1
GROUP BY b, c           # Index applied

WHERE a = 1 and b = '홍길동'
GROUP BY c              # Index applied
```

<br>

### Pagination + Covering Index

```sql
// Before applying covering index
SELECT *
FROM boards
WHERE condition
ORDER BY id DESC
OFFSET page_number
LIMIT page_size

// After applying covering index
SELECT  *
FROM  boards as b
JOIN (SELECT id
      FROM boards
      WHERE condition
      ORDER BY id DESC
      OFFSET page_number
      LIMIT page_size) as cover 
on cover.id = b.id
```

Inside the `JOIN`, the `select`, `where`, `orderby`, and `limit` clauses consist only of index columns, meaning the index itself contains all the data necessary to satisfy the query.

Therefore, by using the `id` retrieved via the covering index, items to be queried from the actual data block can be retrieved quickly.

Typically, a query's performance degrades when `order by` and `limit` operations access data blocks.

![](https://velog.velcdn.com/images/minbo2002/post/9c450c6d-00f5-436b-a748-d9ab98bd001c/image.png)

If a covering index is applied, for a query, `WHERE`, `ORDER BY`, and `LIMIT` index searches are processed first, and then only specific rows access the data block, thereby improving query speed.

At this point, the clustered index (PK) `id` is automatically included in all indexes, so paging operations are quickly handled by the covering index, and only the necessary columns are fetched at the end.

![](https://velog.velcdn.com/images/minbo2002/post/385d7767-f2da-48e8-8798-7e77584421d7/image.png)

**Example Code QueryDSL**
```java
public Page<Match> findList(Pageable pageable, MatchSearchRequest searchRequest) {

	// Retrieve PK id using covering index
    List<Long> ids = queryFactory
            .select(match.id)
            .from(match)
            .where(eqStartAt(searchRequest.getMatchDay()),
                   eqGender(searchRequest.getGender()),
                   eqStatus(searchRequest.getMatchStatus()),
                   eqPersonnel(searchRequest.getPersonnel()),
                   eqStadiumName(searchRequest.getStadiumName()))
            .orderBy(match.id.desc())
            .offset(pageable.getOffset())           
            .limit(pageable.getPageSize())
            .fetch();

	// Return empty page if no target
	if (CollectionUtils.isEmpty(ids)) {
            return Page.empty();
    }

	// Retrieve select clause using the fetched PK id
	List<Match> matchList = queryFactory
            .selectFrom(match)
            .where(match.id.in(ids))
            .orderBy(match.startAt.asc())
            .fetch();

	// Retrieve page count using the fetched PK id
	JPAQuery<Long> countQuery = queryFactory
            .select(match.count())
            .from(match)
            .where(match.id.in(ids));

	return PageableExecutionUtils.getPage(matchList, pageable, countQuery::fetchOne);
}
```
