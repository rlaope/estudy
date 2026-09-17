# Presto

Presto is a distributed SQL query engine. It is not a storage engine and has no indexes.

Data resides in S3, HDFS, MySQL, Kafka, etc., and Presto only reads and computes it.

### Query Execution Flow

```
SQL
 ↓
Logical Plan
 ↓ (Optimizer)
Physical Plan
 ↓
Distributed Execution
```

In the Optimizer stage, Predicate Pushdown, Partition Pruning, and Vectorized Execution are determined.

### Predicate Pushdown

Most query engines aim to apply filtering as close to the source as possible. Applying filters close to the source means efficiently reading only the necessary data from the moment the file is read, rather than reading all data from the file system and then filtering it in memory.

Parquet and ORC format files store various statistics data per column, such as min and max. This technique, filter pushdown, uses this statistics information to skip unnecessary data and read only the required data.

It's an optimization that pushes down `WHERE` conditions to a pre-scan stage, avoiding reading unnecessary data blocks altogether.

```
S3 / Parquet / ORC
   ↑
Predicate Pushdown
   ↑
Presto Scan Operator
```

Only queries of the following forms are subject to pushdown:

```
col = 10
col > 100
col BETWEEN 10 AND 20
col LIKE 'abc%'
```

Impossible conditions include `REGEXP_LIKE`, `SUBSTR`, or `col + 1 = 10`, as these require reading all rows to evaluate.

### Partition Pruning

This is an optimization where the system determines, based on the `WHERE` clause, that certain directories do not need to be opened at all.

![](https://miro.medium.com/v2/resize:fit:720/format:webp/1*A2gPlCHCTMiKbrKDlDB4mg.png)

A method to optimize performance by reading only specified partitions during access.

Athena's partition structure is:

```
s3://logs/
  └── dt=2025-12-15/
      └── hour=10/
      └── hour=11/
```
```
WHERE dt = '2025-12-15'
  AND hour = 11
```

It can only access `hour = 11`, meaning it's usable with constant comparisons and without functions, but not in cases requiring a full scan like pushdown.

### Vectorized Execution

This method processes data by grouping columns together and computing them at once, rather than processing rows one by one.

Row-based vs. Vector-based

```
# row based
for row in rows:
  if row.a > 10:
    sum += row.b

# vector (load all at once and sum all at once)
load column a [1024 rows]
load column b [1024 rows]
apply mask (a > 10)
sum masked b
```

It minimizes branching, is efficient for CPU cache, and allows for SIMD utilization.

> SIMD: A CPU execution method that applies the same operation to multiple data items at once, meaning a single CPU instruction processes multiple items simultaneously.

This breaks down in cases like `REGEXP_LIKE(col, 'a.*b')` because a state machine is executed for each row, making vector processing impossible, thus degrading to scalar execution.

```
[Partition Pruning]
   ↓ (directory removal)

[Predicate Pushdown]
   ↓ (file / row group removal)

[Vectorized Execution]
   ↓ (fast computation of read data)
```

| Item                 | LIKE 'abc%' | REGEXP_LIKE |
| ------------------ | ----------- | ----------- |
| Partition Pruning  | Possible    | Impossible  |
| Predicate Pushdown | Possible    | Mostly Impossible |
| Vectorized Exec    | Possible    | Impossible  |
| CPU Cost           | Low         | High        |


### LIKE vs REGEX

Basically, `LIKE` statements are faster because they heavily rely on the predicate pushdown, partition pruning, and vectorized execution discussed above.

This is also advantageous for prefix matching, min/max statistics, and dictionary encoding in Parquet data format, and allows for vectorized comparisons.

Regex cannot do this; it must read the entire string of each row and feed it one by one into the regex engine, which is also CPU-bound.

However, if the syntax is like `LIKE "%abc"`, or if a full regular expression closely matches the desired result, then regex is recommended in those cases.

It's good if the string is fixed, there's little backtracking, few state machines, and the string length is short.

If a `LIKE` clause has a leading wildcard requiring a full scan, it will ultimately take the same or even longer query time as regex because it's a full scan. Therefore, if you want to query by conditioning values at both ends, it's better to define the range with regex.
