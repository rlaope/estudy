# SQL Window Functions (WINDOW FUNCTION)

## Window Function
These functions are designed to easily define relationships between rows.

Window functions are also known as analytic functions or ranking functions.

Some are existing aggregate functions, while others are new features specifically created for window functions.

Unlike other functions, window functions cannot be nested, but they can be used in subqueries.

### Types
Window functions can be broadly classified into 5 groups.

1. Group-internal ranking (RANK) functions: RANK, DENSE_RANK, ROW_NUMBER
2. Group-internal aggregate (AGGREGATE) functions: SUM, MAX, MIN, AVG, COUNT
3. Group-internal row order functions: FIRST_VALUE, LAST_VALUE, LAG, LEAD (supported only in Oracle)
4. Group-internal ratio functions: CUME_DIST, PERCENT_RANK. NTILE, RATIO_TO_REPORT
5. Statistical analysis functions, including linear analysis

### Syntax

The `OVER` clause is a mandatory keyword for window functions.

```sql
SELECT WINDOW_FUNCTION (ARGUMENT) OVER
( [PARTITION BY 컬럼] [ORDER BY 컬럼] [WINDOWING 절])
FROM 테이블명;
```

- WINDOW_FUNCTION: The window function
- ARGUMENTS: 0 to N arguments can be specified depending on the function.
- PARTITION BY: Divides the entire set into subgroups based on a criterion.
- ORDER BY: Specifies the column(s) by which to order for ranking.
- WINDOWING clause: The WINDOWING clause can powerfully specify the row-based range for the function's target. (Not supported in SQL Server)

### WINDOWING
- ROWS: Specifies a set of rows as a physical unit for the window size (subset).
- RANGE: Specifies a set of rows by logical address.
- BETWEEN ~ AND: Specifies the start and end positions of the window.
- UNBOUNDED PRECEDING: Indicates that the window's start position is the first row.
- UNBOUNDED FOLLOWING: Indicates that the window's end position is the last row.
- CURRENT ROW: Indicates that the window's start position is the current row.

## Group-Internal RANK Functions

### RANK
A function that calculates rank.

It can calculate rank within a specific range (PARTITION) or for the entire dataset.

It assigns the same rank to identical values.

Example showing employees ranked by salary in descending order, and by salary in descending order within each JOB.

```sql
SELECT JOB, ENAME, SAL,
       RANK() OVER (ORDER BY SAL DESC) ALL_RANK, # 급여 높은 순
       RANK() OVER (PARTITION BY JOB ORDER BY SAL DESC) JOB_RANK # job 별로 급여 높은 순
FROM EMP;
```

If salaries are identical, they are assigned the same rank.

`JOB_RANK`, partitioned by `JOB`, assigns ranks only within the same job scope.

Because the `ORDER BY SAL DESC` and `PARTITION BY JOB` conditions conflict in a single statement, the results are not sorted by `JOB` but by `ORDER BY SAL DESC`.

### DENSE_RANK
Similar to `RANK`, but treats identical ranks as a single count.

`RANK` would assign ranks like 1, 2, 3, whereas `DENSE_RANK` assigns ranks like 1, 1, 3.

```sql
SELECT JOB, ENAME, SAL,
       RANK() OVER(ORDER BY SAL DESC) ALL_RANK,
       DENSE_RANK() OVER (ORDER BY SAL DESCL) DENSE_RANK
FROM EMP;
```

### ROW_NUMBER
While `RANK` and `DENSE_RANK` assign the same rank to identical values,

`ROW_NUMBER` assigns a unique rank even to identical values.

```sql
SELECT JOB, ENAME, SAL,
       RANK() OVER (ORDER BY SAL DESC) ALL_RANK,
       ROW_NUMBER() OVER (ORDER BY SAL DESC) ROW_NUMBER
FROM EMP;
```

`ROW_NUMBER` assigns unique ranks to avoid duplicate ranks.

If you want to define the order in which results appear for identical values, include `ORDER BY` as well.

<br>

## Group-Internal Aggregate Functions

### SUM
The `SUM` function can be used to calculate the sum of a window per partition.

Example calculating the sum of salaries for employees who share the same manager.

```sql
SELECT MGR, ENAME, SAL,
       SUM(SAL) OVER (PARTITION BY MGR) MGR_SUM
FROM EMP;
```

Example of adding an `ORDER BY` clause within the `OVER` clause to sort data within a partition and output the cumulative sum up to the previous salary data.

```sql
SELECT MGR, ENAME, SAL,
       SUM(SAL) OVER (PARTITION BY MGR ORDER BY SAL RANGE UNBOUNDED PRECEDING) MGR_SUM
FROM EMP;

RANGE UNBOUNDED PRECEDING; # 현재 행을 기준으로 파티션 내의 첫 번째 행까지 범위를 지정
```

### MAX
It can find the maximum value within a window per partition.

Example calculating the maximum salary among employees who share the same manager.

```sql
SELECT MGR, ENAME, SAL,
       MAX(SAL) OVER (PARTITION BY MGR) MGR_MAX
FROM EMP;
```

You can also extract only the rows with the maximum value per partition using an inline view.

```sql
SELECT MGR, ENAME, SAL
FROM (SELECT MGR, ENAME, SAL,
             MAX(SAL) OVER (PARTITION BY MGR) IV_MAX_SAL
             FROM EMP )
WHERE SAL = IV_MAX_SAL;
```

### MIN
It can find the minimum value within a window per partition.

Example sorting employees who share the same manager by hire date and simultaneously finding the minimum salary.

```sql
SELECT MGR, ENAME, HIREDEAT, SAL,
       MIN(SAL) OVER (PARTITION BY MGR ORDER BY HIREDATE) MGR_MIN
FROM EMP;
```

### AVG

It can calculate statistical values per partition.

Example calculating the average salary for employees under the same manager, but only considering the employee immediately before and immediately after the current employee within that manager's group (sum of previous row + current row + next row divided by 3; if there's no previous row, sum of current row + next row divided by 2).

```sql
SELECT MGR, ENAME, HIREDEAT, SAL,
       ROUND(AVG(SAL)) OVER (PARTITION BY MGR ORDER BY HIREDATE)
       ROW BETWEEN 1 PRECEDING AND 1 FOLLOWING) ) MGR_AVG
FROM EMP;
```

### COUNT

Example sorting employees by salary and outputting the count of employees whose salary is within 50 less or 150 more than their own salary.

```sql
SELECT ENAME , SAL,
       COUNT(*) OVER (ORDER BY SAL
       RANGE BETWEEN 50 PRECEDING AND 150 FOLLOWING) SIM_CNT
FROM EMP;

RAGE BETWEEN 50 PRECEDING AND 150 FOLLOWING;
```

All rows whose salary falls within the range of -50 to +150 relative to the current row's salary are included.

`RANGE` indicates the range of data values before and after the current row's data value.

<br>

## Group-Internal Row Order Functions

### FIRST_VALUE
It can retrieve the first value that appears in a window per partition.

Not supported in SQL Server.

The same result can be obtained using the `MIN` function.

Example sorting employees by department in descending order of salary and outputting the first value that appears within the partition.

```sql
SELECT DEPTNO, ENAME, SAL,
       FIRST_VALUE(ENAME) OVER (PARTITION BY DEPTNO ORDER BY SAL DESC
       ROWS UNBOUNDED PRECEDING) DEPT_RICH
FROM EMP;
```

If there are people with the same salary, sorting must be specified.

This is because `FIRST_VALUE` processes the first row encountered and does not acknowledge ties.

Example with added sorting: if salaries are the same, names will appear in ascending order.

```sql
SELECT DEPTNO, ENAME, SAL,
       FIRST_VALUE(ENAME) OVER (PARTITION BY DEPTNO ORDER BY SAL DESCENAME ASC)
       ROWS UNBOUNDED PRECEDING) DEPT_RICH
FROM EMP;
```

### LAST_VALUE
It can retrieve the last value that appears in a window per partition.

Not supported in SQL Server.

The same result can be obtained using the `MAX` function.

Example sorting employees by department in descending order of salary and outputting the last value that appears within the partition.

```sql
SELECT DEPTNO, ENAME, SAL,
       LAST_VALUE(ENAME) OVER (PARTITION BY DEPTNO ORDER BY SAL DESC ROW BETWEEN CURRENT ROW AND UNBOUNDED FOLLOWING) DEPT_POOR
FROM EMP;

ROW BETWEEN CURRENT ROW AND UNBOUNDED FOLLOWING # 현재 행을 포함해서 파티션 내의 마지막 행까지의 범위를 지정한다.
```

### LAG

It can retrieve the value from a previous row (Nth row back) within a window per partition.

Not supported in SQL Server.

Example sorting employees by earliest hire date and outputting the salary of the employee hired one position before the current employee, along with the current employee's salary.

```sql
SELECT ENAME, HIREDATE, SAL,
       LAG(SAL) OVER (ORDER BY HIREDATE) PREV_SAL
FROM EMP
WHERE JOB = 'SALESMAN';
```

Execution Result
```
[Execution Result]
ENAME   HIREDATE  SAL  PREV_SAL
------- --------- ---- -------
ALLEN   1981-02-20 1600
WARD    1981-02-22 1250 1600
TURNER  1981-09-08 1500 1250
MARTIN  1981-09-28 1250 1500

4 rows selected.
```

The `LAG` function can use up to 3 arguments.

`LAG(SAL,2,0)` <- The second argument determines how many rows back to fetch the value from; the default is 1. Here, 2 is specified, so it fetches the value from the 2nd row back.

The third argument handles cases where the first row of a partition has no preceding data, resulting in `NULL`. In such cases, it allows replacing `NULL` with another value, similar to `NVL`/`ISNULL`.

```sql
SELECT ENAME, HIREDATE, SAL,
       LAG(SAL,2,0) OVER (ORDER BY HIREDATE) PREV_SAL
FROM EMP
WHERE JOB = 'SALESMAN';
```

Execution Result
```
[Execution Result]
ENAME   HIREDATE  SAL  PREV_SAL
------- --------- ---- -------
ALLEN   1981-02-20 1600 0
WARD    1981-02-22 1250 0
TURNER  1981-09-08 1500 1600
MARTIN  1981-09-28 1250 1250

4 rows selected.
```

### LEAD
It can retrieve the value from a subsequent row (Nth row forward) within a window per partition.

Not supported in SQL Server.

Example sorting employees by earliest hire date and outputting the hire date of the next employee hired, along with their own.

```sql
SELECT ENAME, HIREDATE
       LEAD(HIREDATE) OVER (ORDER BY HIREDATE) NEXT_HIRED
FROM EMP;
```

Execution Result
```
[Execution Result]
ENAME    HIREDATE   NEXTHIRED
-------- ---------  ---------
ALLEN    1981-02-20 1981-02-22
WARD     1981-02-22 1981-04-02
TURNER   1981-09-08 1981-09-28
MARTIN   1981-09-28

4 rows selected.
```

Like `LAG`, the `LEAD` function can also use up to 3 arguments.

<br>

## Group-Internal Ratio Functions

### CUME_DIST
It calculates the cumulative distribution (percentile rank) of a value within a window per partition, representing the proportion of rows less than or equal to the current row.

Not supported in SQL Server.

Example outputting a value between 0 and 1 indicating the cumulative position of one's salary within the set of employees belonging to the same department.

```sql
SELECT DEPTNO, ENAME, SAL,
       CUME_DIST() OVER (PARTITION BY ORDER BY SAL DESC) CUME_DIST
FROM EMP;
```

### PERCENT_RANK
Using a partition-based function, it calculates the percentile rank based on row order within a window per partition, where the first row is 0 and the last row is 1. (This is a percentile based on row order, not value.)

Not supported in SQL Server.

Example outputting a value between 0 and 1 indicating the positional rank of one's salary within the set of employees belonging to the same department.

```sql
SELECT ENAME, SAL,
       PERCENT_RANK() OVER (PARTITION BY DEPTNO ORDER BY SAL DESC) P_R
FROM EMP;
```

```
[Execution Result]
DEPTNO  ENAME SAL  P_R
------ ------ ---- ----
10      KING   5000 0
10      CLARK  2450 0.5
10      MILLER 1300 1
20      SCOTT  3000 0
20      FORD   3000 0
20      JONES  2975 0.5
20      ADAMS  1100 0.75
20      SMITH  800  1
30      BLAKE  2850 0
30      ALLEN  1600 0.2
30      TURNER 1500 0.4
30      MARTIN 1250 0.6
30      WARD   1250 0.6
30      JAMES  950  1

14 rows selected.
```

For `DEPTNO` 10, there are 3 rows, so there are 2 intervals.

Dividing the range between 0 and 1 into 2 intervals yields 0, 0.5, 1.

For `DEPTNO` 20, there are 5 rows, and 4 intervals.

Dividing the range between 0 and 1 into 4 intervals yields 0, 0.25, 0.5, 0.75, 1.

For `DEPTNO` 30, there are 6 rows, and 5 intervals.

Dividing the range between 0 and 1 into 5 intervals yields 0, 0.2, 0.4, 0.6, 0.8, 1.

### NTILE
It can divide the total number of rows per partition into N groups based on the `ARGUMENT` value.

Sorts all employees by salary in descending order and classifies them into 4 groups based on salary.

```sql
SELECT ENAME, SAL,
       NTILE(4) OVER (ORDER BY SAL DESC) QUAR_TILE
FROM EMP;
```

```
[Execution Result]
DEPTNO ENAME    SAL  QUAR_TILE
------ ------- ---- --------
10     KING    5000 1
10     FORD    3000 1
10     SCOT    3000 1
20     JONES   2975 1
20     BLAKE   2850 2
20     CLARK   2450 2
20     ALLEN   1600 2
20     TURNER  1500 2
30     MILLER  1300 3
30     WARD    1250 3
30     MARTIN  1250 3
30     ADAMS   1100 4
30     JAMES   950  4
30     SMITH   800  4

14 rows selected.
```

`NTILE(4)` means dividing 14 team members into 4 groups.

Dividing 14 people into 4 sets results in a quotient of 3 and a remainder of 2.

The remaining two are allocated to the earlier groups first.

The groups will be divided as 4 + 4 + 3 + 3.

### RATIO_TO_REPORT
It can calculate the percentage of a column's value for each row relative to the total `SUM(column)` value within a partition, as a decimal.

The result value ranges from > 0 to <= 1.

The sum of individual ratios will be 1.

Not supported in SQL Server.

Example calculating the proportion of one's salary out of the total salary for salesmen.

```sql
SELECT ENAME, SAL,
       ROUND(RATIO_TO_REPORT(SAL) OVER (), 2) P_R
FROM EMP
WHERE JOB = 'SALESMAN'
```

```
[Execution Result]
ENAME  SAL   R_R
------ ---- ----
ALLEN  1600 0.29 (1600 / 5600)
WARD   1250 0.22 (1250 / 5600)
MARTIN 1250 0.22 (1250 / 5600)
TURNER 1500 0.27 (1500 / 5600)

4 rows selected.
```
