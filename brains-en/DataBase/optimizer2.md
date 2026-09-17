# Optimizer Optimization, Goals, and Limitations

## Optimizing Total Processing Speed
Assuming the entire final result set of a query is read, the execution plan that uses the fewest system resources is selected. Most DBMS's default optimizer mode is geared towards optimizing total processing speed.

## Optimizing Minimum Response Time
Assuming only a portion of the entire result set is read before stopping, the execution plan that can achieve the fastest response time is selected. If data is read to the end using an execution plan generated in this mode, it may consume more resources and perform slower than an execution plan optimized for total processing speed.

## Factors Affecting Optimizer Behavior

1. SQL and Operator Forms
Even if the result is the same, the optimizer may make different choices depending on how the SQL is written or which operators are used.

2. Optimizing Factors
Even if a query is written identically, the execution plan and performance can vary significantly depending on how indexes, IOTs, clustering, partitioning, etc., are configured.

3. DBMS Constraint Settings
Objects can utilize constraint setting features provided by the DBMS, such as PK, FK, Check, and NotNull, for entity integrity, referential integrity, and domain integrity. These constraint settings provide crucial information for the optimizer to optimize query performance. For example, if a NotNull constraint is set on an indexed column, the optimizer can use this index for a Count query that calculates the total number of rows.

4. Optimizer Hints
User-specified optimizer hints take precedence over the optimizer's judgment.

5. Statistical Information
The influence of statistical information on the optimizer is absolute, and all CBO's decision criteria are derived from statistical information.
Key statistical information includes the following:

- Table
  - Number of total rows in the table
  - Number of total blocks occupied by the table
  - Average length of rows in the table
- Column
  - Number of distinct values in the column
  - Distribution of NULL values within the column
  - Average length of column values
  - Estimated data distribution within the column
- Index
  - Number of LEAF BLOCKs: Number of blocks storing data
  - LEVELS: LEVEL information of the index tree
  - CLUSTERING FACTOR: Density of clustered data to be accessed
- System Statistics
  - I/O performance and utilization
  - CPU performance and utilization

6. Optimizer-Related Parameters
Even if all environments, including SQL, data, statistical information, and hardware, are identical, upgrading the DBMS version can cause the optimizer to behave differently. This phenomenon occurs as optimizer-related parameters are added or changed.

7. DBMS Version and Type
Even if optimizer-related parameters are the same, the execution plan may differ depending on the version. Furthermore, even for the same SQL, the internal processing method may vary depending on the DBMS type.

## Optimizer Limitations
Since the optimizer is merely a software engine created by humans, it can never be perfect.

While some problems are difficult to solve with current technology, others are technically possible but not yet implemented due to practical constraints.

Therefore, **one should not blindly trust the optimizer, and if it is operating inefficiently, additional mechanisms like Oracle hints should be used to guide it to work correctly.**

The factors that prevent the optimizer from being perfect are as follows:

1. Lack of optimizing factors
2. Inaccurate statistical information
3. Assumption of uniform distribution when using bind variables
4. Unrealistic assumptions
5. CBO relying on rules
6. Hardware performance
