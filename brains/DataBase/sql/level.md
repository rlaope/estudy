# 계층형 쿼리(START WITH, CONNECT BY PRIOR, SIBLINGS BY)

## 계층형 쿼리
부모, 자식 간의 수직관계를 트리 구조 형태로 보여주는 쿼리  
- START WITH: 트리 구조의 최상위 행 지정
- CONNETC BY: 부모, 자식 관계 지정
- PRIOR: CONNECT BY 절에 사용되며 PRIOR에 지정된 컬럼이 맞은편 컬럼을 찾아감
- CONNECT BY PRIOR 자식 컬럼 = 부모 컬럼: 자식 순방향 전개
- CONNECT BY PRIOR 부모 커럶 = 자식 컬럼: 부모 역방향 전개
- ORDER SIBLINGS: 계층형 쿼리에서 정렬 수행

## 예제

테이블 생성
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

테스트 데이터 조회
```sql
select parent_c as p, child_c as c from t1;
```

결과
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

### 부모 -> 자식 순방향 전개 계층형 쿼리 예제

```sql
select parent_c as p, child as c, level
from t1
start with parent_c = 'a'
connect by prior child_c parent_c
```

결과
```
P  C       LEVEL
-- -- ----------
a  b           1 <- 첫번째 부모행을 시작으로 자식 탐색
b  c           2
c  d           3
c  e           3
e  f           4 <- 첫번째 부모행의 자식 탐색 종료
a  c           1 <- 두번째 부모행의 자식 탐색 시작
c  d           2
c  e           2
e  f           3 <- 두번째 부모행의 자식 탐색 종료
```

level을 보면 첫 번째 최상위 부모 행을 찾으면 자식이 없을 때까지 계속 타고 들어갔다가 마지막까지 탐색 후 두 번째 최상위 부모 행의 자식을 탐색한다.

### CONNECT BY 조건절을 넣을 경우 에제

```sql
select parent_c as p, child c, level
from t1
start with parent_c = 'a'
connect by prior child_c = parent_c and parent_c = 'c'
```

실행 결과

```
P  C       LEVEL
-- -- ----------
a  b           1
a  c           1
c  d           2
c  e           2
```
start with 절에서 선택된 부모 데이터는 무조건 포함이 되고 데이터들에 의해 parent_c = 'c' 조건으로 필터링 된 결과가 나타납니다.

### ORDER SIBLINGS 사용한 계층형 쿼리 예제

```sql
select parent_c as p, child_c as c, level
from t1 
start with parent_c = 'a'
connect by prior child_c = parent_c
order siblings by child_c desc;
```

실행 결과
```
P  C       LEVEL
-- -- ----------
a  c           1 <- LEVEL 1 중에서 child_c desc 정렬했을 때 c가 가장 맨위에 옴
c  e           2 <- LEVEL 2 중에서(위 LEVEL 1의 자식 기준) child_c desc 정렬했을 때 e가 가장 맨위에 옴
e  f           3
c  d           2
a  b           1 <- LEVEL 1 중에서 child_c desc 정렬했을 때 두번째인 b가 옴
b  c           2
c  e           3
e  f           4
c  d           3
```

level 1에서 진입하기 전에 정렬을 한 후 정렬한 결과의 첫 번째 행부터 자식 행을 찾습니다. 이는 자식행을 들어가서도 같은 level 내에서 정렬이 된 후 첫 번째 행부터 타고 들어갑니다.

### 자식 -> 부모 역방향 전개 계층형 쿼리 예제

```sql
select parent_c as p, child_c as c, level
from t1
start with child_c = 'f'
connect by child_c = prior parent_c;
```

실행 결과
```
P  C       LEVEL
-- -- ----------
e  f           1 <- 첫번째 자식행을 시작으로 부모 탐색
c  e           2
a  c           3
b  c           3
a  b           4 <- 첫번째 자식행의 부모 탐색 종료
```

child_c = 'f'에 대한 자식행이 1행밖에 없으므로 마지막 부모 계층까지 탐색 후 종료된다.

