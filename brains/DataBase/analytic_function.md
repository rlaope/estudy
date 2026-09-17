# 분석 함수를 활용해 쿼리 성능 높히기

분석함수 analytic functions는 sql 쿼리 성능을 한 단계 높히는 강력한 도구다.

단순히 데이터를 처리하는 것을 넘어서 데이터 분석과 쿼리 최적화에 있어 핵심적인 역할이다.

이런 함수들은 복잡한 데이터 집합 내에서 row별로 세부적인 계산을 가능하게 한다.

전체 데이터에 걸쳐 다양한 통계와 계산을 유연하게 수행할 수 있게 도와주는 것이다.

대표적으로 ROW_NUMBER(), RANK(), DENSE_RANK() LEAD(), LAG()등이 있다.

### 쿼리 효율성을 높히는 분석 함수

분석 함수를 사용하면 sql 쿼리에 효율성이 여러 면에서 향상된다.

먼저, 전통적인 집계 함수와 달리 사전에 데이터를 그룹화할 필요가 없다.

이는 불필요한 자원 소모를 줄이고 쿼리 성능을 높이는데 큰 도움이 된다.

또한 복잡한 데이터 분석 과정에서 발생할 수 있는 **중간 결과물의 저장과 재처리를 최소화** 할 수 있어 쿼리 실행 시간을 단축시킨다.

### 순위 결정 함수로 데이터 순서 매기기

`ROW_NUMBER()`, `RANK()`, `DENSE_RANK()` 같은 함수는 데이터 내 각 항목의 순위를 정하는 데 유용하게 쓰인다.

예를 들어서 부서별로 급여가 높은 직원을 순위로 매기고 싶다면 ROW_NUMBER()를 통해 구현할 수 있다.

```sql
SELECT name, department, salary, ROW_NUMBER() OVER(PARTITION BY department ORDER BY salary DESC) AS rank
FROM employees;
```

이 쿼리는 각 부서 내에서 급여가 높은 순으로 직원들에게 고유한 순위를 부여한다

RANK(), DENSE_RANK() 함수도 비슷한 방식으로 동작하지만, 동일한 값에 대해서 동일 순서를 부여한다는 점이 다르다. DENSE_RANK()에 경우에는 순위 간격을 항상 1로 유지하는 특징이 있다.

### 데이터 변화를 추적하는 분석 함수

LEAD(), LAG() 함수는 현재 Row와 관련하여 이전 또는 다음 Row의 데이터를 참조할 수 있게 해준다.

이를 활용해 각 직원의 연봉 변화율을 쉽게 계산이 가능하다.

```sql
SELECT name, salary, LAG(salary) OVER(PARTITION BY department ORDER BY hire_date) AS prev_salary,
salary - LAG(salary) OVER(PARTITION BY department ORDER BY hire_date) AS salary_increase
FROM employees;
```

이 쿼리는 각 부서 내에서 입사일자 순으로 직원을 정렬한 후에 이전 직원의 연봉과 현재 직원의 연봉 차이를 계산하여 연봉 인상액을 구하고 있다.

이런 함수들은 특히 시계열 데이터나 연속적인 데이터 집합을 다룰 때, 이전 데이터 포인트와 비교가 필요한 분석에 아주 유용하게 활용된다.

### 분석 함수로 데이터 필터링 최적화하기

만약 각 부서별로 급여가 높은 상위 3명의 직원 정보만 추출하고 싶다면 ROW_NUMBER() 함수를 이용해 다음과 같이 작성이 가능하다.

```sql
WITH ranked_employees AS (
  SELECT name, department, salary,
  ROW_NUMBER() OVER(PARTITION BY department ORDER BY salary DESC) AS rank
)

SELECT * FROM ranked_employees
WHERE rank <= 3;
```

이 쿼리는 먼저 각 부서 내에서 급여 순위를 매긴 후, 순위가 3이하인 직원들만 선택하는 방식으로 동작한다.

이렇게 하면 데이터를 전체 스캔하지 않고도 필요한 결과만 빠르게 필터링이 가능하다.

분석 함수를 활용하면 이처럼 SQL 쿼리의 효율성과 성능을 크게 개선할 수 있다.