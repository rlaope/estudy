# Hierarchical Queries (START WITH, CONNECT BY PRIOR, SIBLINGS BY)

## Hierarchical Queries
A query that displays the vertical relationship between parents and children in a tree structure.
- START WITH: Specifies the top-level row of the tree structure.
- CONNECT BY: Specifies the parent-child relationship.
- PRIOR: Used in the CONNECT BY clause; the column specified in PRIOR finds the corresponding column.
- CONNECT BY PRIOR child_column = parent_column: Forward expansion to children.
- CONNECT BY PRIOR parent_column = child_column: Backward expansion to parents.
- ORDER SIBLINGS: Performs sorting in hierarchical queries.

## Examples

Table Creation
```sql
create table t1
(
     parent_c varchar2(1)
    ,child_c varchar2(1)
);
insert into t1
select 'a','b' from dual
union all 
select 'b','c' from dual
union all 
select 'a','c' from dual
union all 
select 'c','d' from dual
union all 
select 'c','e' from dual
union all 
select 'e','f' from dual;
commit;
```

Test Data Query
```sql
select parent_c as p, child_c as c from t1;
```

Result
```
P  C
-- --
a  b
b  c
a  c
c  d
c  e
e  f
```

### Example of Hierarchical Query for Parent -> Child Forward Expansion

```sql
select parent_c as p, child as c, level
from t1
start with parent_c = 'a'
connect by prior child_c parent_c
```

Result
```
P  C       LEVEL
-- -- ----------
a  b           1 <- Starts searching for children from the first parent row.
b  c           2
c  d           3
c  e           3
e  f           4 <- Ends searching for children of the first parent row.
a  c           1 <- Starts searching for children of the second parent row.
c  d           2
c  e           2
e  f           3 <- Ends searching for children of the second parent row.
```

Looking at the `level`, when the first top-level parent row is found, it continues to traverse down until there are no more children, then after the full traversal, it searches for children of the second top-level parent row.

### Example when adding a CONNECT BY clause

```sql
select parent_c as p, child c, level
from t1
start with parent_c = 'a'
connect by prior child_c = parent_c and parent_c = 'c'
```

Execution Result

```
P  C       LEVEL
-- -- ----------
a  b           1
a  c           1
c  d           2
c  e           2
```
The parent data selected in the `start with` clause is always included, and the result is filtered by the `parent_c = 'c'` condition based on the data.

### Example of Hierarchical Query using ORDER SIBLINGS

```sql
select parent_c as p, child_c as c, level
from t1 
start with parent_c = 'a'
connect by prior child_c = parent_c
order siblings by child_c desc;
```

Execution Result
```
P  C       LEVEL
-- -- ----------
a  c           1 <- Among LEVEL 1, 'c' comes first when sorted by child_c desc.
c  e           2 <- Among LEVEL 2 (based on children of the above LEVEL 1), 'e' comes first when sorted by child_c desc.
e  f           3
c  d           2
a  b           1 <- Among LEVEL 1, 'b' comes second when sorted by child_c desc.
b  c           2
c  e           3
e  f           4
c  d           3
```

Before entering level 1, sorting is performed, and then child rows are searched starting from the first row of the sorted result. This also applies when traversing into child rows: within the same level, sorting is performed, and then traversal starts from the first row.

### Example of Hierarchical Query for Child -> Parent Backward Expansion

```sql
select parent_c as p, child_c as c, level
from t1
start with child_c = 'f'
connect by child_c = prior parent_c;
```

Execution Result
```
P  C       LEVEL
-- -- ----------
e  f           1 <- Starts searching for parents from the first child row.
c  e           2
a  c           3
b  c           3
a  b           4 <- Ends searching for parents of the first child row.
```

Since there is only one child row for `child_c = 'f'`, the search continues up to the last parent level and then terminates.
