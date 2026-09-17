# Presto

Presto는 분산 sql 퀄 엔진이다. 저장 엔진아니고 인덱스는 없다.

데이터는 s3, hfds, mysql, kafka등에 있고 읽어서 계산만 한다.

### 쿼리 실행 흐름

```
SQL
 ↓
Logical Plan
 ↓ (Optimizer)
Physical Plan
 ↓
Distributed Execution
```

Optimizer 단계에서는 Predicate Pushdown, Partition Pruning, Vectorized Execution 이 세계가 결정된다.

### Predicate Pushdown

대부분의 쿼리 엔진은 필터링을 최대한 소스에 가깝게 적용하고자한다. 소스에 가깝게 필터를 적용한다는 의미는 파일 시스템에서 데이터를 읽어온 이후에 메모리에서 필터링하는것이 아니라 파일을 읽을때부터 꼭 필요한 데이터만 효율적으로 읽겠다는 점이다.

Parquet와 ORC 포캣의 파일을 각 컬럼별로 다양한 stats 데이터를 보관하고 있다 min, max같은? 이러한 stats 정보가 불필요한 데이터를 건너뛰고 필요한 데이터만 읽을수있도록 filter push down. 하는것이 이 기법이다.



where 조건을 스캔 이전 단계로 내려서 필요없는 데이터 블록 자체를 읽지 않는 최적화다.

```
S3 / Parquet / ORC
   ↑
Predicate Pushdown
   ↑
Presto Scan Operator
```

다음 형태 쿼리만 푸시다운의 대상임

```
col = 10
col > 100
col BETWEEN 10 AND 20
col LIKE 'abc%'
```

불가능한 조건은 REGEXP_LIKE이나 substr, col + 1 = 10 이런거는 모든 로우를 읽어야 판단이 가능하다.

### Partition Pruning

where절을 보고 아예 이 디렉토리는 열 필요가 없다고 판단하는 최적화다.

![](https://miro.medium.com/v2/resize:fit:720/format:webp/1*A2gPlCHCTMiKbrKDlDB4mg.png)

일긍ㄹ때 지정된 파티션만 읽을수 있도록 성능최적화하는 방법이다.

아테나 기준 파티션구조는

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

hour = 11만 접근 즉 상수비교, 함수 없을때 사용이 가능하고 푸시다운처럼 전체스캔하는 경우에는 사용이 불가능함.

### Vectorized Execution

row를 하나씩 처리하지 않고 컬럼 단위로 묶어서 한 번에 계산하는 방식이다.

Row 기반 vs Vector 기반

```
# row 기반
for row in rows:
  if row.a > 10:
    sum += row.b

# vector (한번에 로드해서 한번에 다 합침)
load column a [1024 rows]
load column b [1024 rows]
apply mask (a > 10)
sum masked b
```

분기를 최소화하고 cpu cache에 효율적이며 simd 활용이 가능하다

> SIMD: 같은 연산을 여러 데이터에 한 번에 적용시키는 cpu 실행 방식 즉 cpu 명령어 1번으로 여러개를 동시에 처리함.

REGEXP_LIKE(col, 'a.*b') 이런거에서는 깨지는데 row마다 상태 머신 실행되어 벡터 처리가 불가능해서임 scalar execution으로 강등.

```
[Partition Pruning]
   ↓ (디렉토리 제거)

[Predicate Pushdown]
   ↓ (파일 / row group 제거)

[Vectorized Execution]
   ↓ (읽은 데이터 빠르게 계산)
```

| 항목                 | LIKE 'abc%' | REGEXP_LIKE |
| ------------------ | ----------- | ----------- |
| Partition Pruning  | 가능          | 불가          |
| Predicate Pushdown | 가능          | 거의 불가       |
| Vectorized Exec    | 가능          | 불가          |
| CPU 비용             | 낮음          | 높음          |


### LIKE vs REGEX

기본적으로 LIKE문이 더 빠른데 위에서 본 predicate pushdown, parition pruning, vectorized execution에 크게 의존하고 있기 때문이다.

Parquet 데이터 포맷에 prefix match, min/max statistics, dictionary encoding에도 유리하며 vectorized 비교도 가능해서다

regex는 그게 불가능하다 각 row의 문자열 전체를 읽어서 정규직 엔진에 하나씩 넣어야해 cpu 바운드도 크다

근데 LIKE "%abc" 같은 문법이라면 regex를 더 권장하는데 혹은 완전 정규식이 결과와 일치에 가깝다거나. 그런경우에도 권장한다.

일단 문자열 고정에 backtracking이 거의 없고 상태머신이 적으며 문자열 길이도 짧다면 좋다.

와일드카드가 앞에달려 전체스캔을 해야하는 LIKE라면 결국 전체 스캔이기 때문에 regex와 동일한 수준 혹은 더 높은 쿼리시간을 소요한다. 그래서 regex로 범위 지정을 하자 앞뒤로 값을 조건걸어 쿼리하고싶다면