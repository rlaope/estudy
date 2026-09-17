# SQL 윈도우 함수 (WINDOW FUNCTION)

## Window Function
행과 행 간의 관계를 쉽게 정의하기 위해 만든 함수다.  
  
윈도우 함수는 분석 함수나 순위 함수로도 알려져 있다.
  
기존에 사용하던 집계 함수도 있고, 새로이 윈도우 함수 전용으로 만들어진 기능도 있다.
  
윈도우 함수는 다른 함수와 달리 중첩해서 사용은 못하지만, 서브쿼리에는 사용할 수 있다.

### 종류
윈도우 함수는 크게 5가지 그룹으로 분류가 가능하다.

1. 그룹 내 순위(RANK) 관련 함수: RANK, DENSE_RANK, ROW_NUMBER
2. 그룹 내 집계(AGGREGATE) 관련 함수: SUM, MAX, MIN, AVG, COUNT
3. 그룹 내 행 순서 관련 함수: FIRST_VALUE, LAST_VALUE, LAG, LEAD(오라클에서만 지원)
4. 그룹 내 비율 관련 함수: CUME_DIST, PERCENT_RANK. NTILE, RATIO_TO_REPORT
5. 선형 분석을 포함한 통계 분석 함수

### 문법

윈도우 함수에는 OVER 문구가 키워드로 필수 포함이다.

```sql
SELECT WINDOW_FUNCTION (ARGUMENT) OVER
( [PARTITION BY 컬럼] [ORDER BY 컬럼] [WINDOWING 절])
FROM 테이블명;
```

- WINDOW_FUCTION: 윈도우 함수
- ARGUMENTS(인수): 함수에 따라 0~N개 인수가 지정될 수 있다.
- PARTITION BY: 전체 집합을 기준에 의해 소그룹으로 나눌 수 있다.
- ORDER BY: 어떤 항목에 대해 순위를 지정할 지 order by를 기술한다.
- WINDOWING 절: WINDOWING 절은 함수의 대상이 되는 행 기준의 범위를 강력하게 지정할 수 있다. (sql server 에서는 지원하지 않음)

### WINDOWING
- ROWS: 부분집합인 윈도우 크기를 물리적 단위로 행의 집합을 지정한다.
- RANGE: 논리적인 주소에 의해 행 집합을 지정한다.
- BETWEEN ~ AND: 윈도우의 시작과 끝의 위치를 지정한다.
- UNBOUNDED PRECEDING: 윈도우의 시작 위치가 첫 번째 행임을 의미한다.
- UNBOUNDED FOLLOWING: 윈도우 마지막 위치가 마지막 행임을 의미한다.
- CURRENT ROW: 윈도우 시작 위치가 현재 행임을 의미한다.

## 그룹 내 RANK 관련 함수

### RANK
순위를 구하는 함수.
  
특정 범위(PARTITION) 내에서 순위를 구할 수도 있고, 전체 데이터에 대한 순위를 구할 수도 있다.  
  
동일한 값에 대해서는 동일한 순위를 부여하게 된다.
  
사원 데이터에서 급여가 높은 순서와 JOB별로 급여가 높은 순서를 출력하는 예제

```sql
SELECT JOB, ENAME, SAL,
       RANK() OVER (ORDER BY SAL DESC) ALL_RANK, # 급여 높은 순
       RANK() OVER (PARTITION BY JOB ORDER BY SAL DESC) JOB_RANK # job 별로 급여 높은 순
FROM EMP;
```

동일한 급여가 있다면 같은 순위를 부여한다.  
  
PARTITION으로 구분한 JOB_RANK는 같은 업무 범위 내에서만 순위를 부여한다.  
  
하나의 문장에 ORDER BY SAL DESC와 PARTITION BY JOB의 조건이 충돌 났기 때문에 JOB별로는 정렬되지 않고, ORDER BY SAL DESC 조건으로 정렬된다.

### DENSE_RANK
RANK와 흡사하지만, 동일한 순위를 하나의 건수로 취급한다.  
  
RANK는 1,2,3 순위로 표기하지만, DENSE_RANK는 1,1,3 순위로 순위를 부여한다

```sql
SELECT JOB, ENAME, SAL,
       RANK() OVER(ORDER BY SAL DESC) ALL_RANK,
       DENSE_RANK() OVER (ORDER BY SAL DESCL) DENSE_RANK
FROM EMP;
```

### ROW_NUMBER
RANK, DENSE_RANK가 동일한 값에 대해서는 동일한 순위를 부여하는데 반해  
  
ROW_NUMBER는 동일한 값이라도 고유한 순위를 부여한다.

```sql
SELECT JOB, ENAME, SAL,
       RANK() OVER (ORDER BY SAL DESC) ALL_RANK,
       ROW_NUMBER() OVER (ORDER BY SAL DESC) ROW_NUMBER
FROM EMP;
```

ROW_NUMBER는 동일한 순위를 배제하기 위해 유니크한 순위를 정한다.  
  
같은 값에 대해서 어떤 결과가 먼저 낭로지 순서가 정하고 싶다면 ORDER BY를 같이 기재하라.

<br>

## 그룹 내 집계 (AGGREGATE) 관련 함수

### SUM
SUM 함수를 이용해 파티션별 윈도우의 합을 구할 수 있다.
  
사원들의 급여와 같은 매니저를 두고 있는 사원들의 급여 합을 구하는 예제

```sql
SELECT MGR, ENAME, SAL,
       SUM(SAL) OVER (PARTITION BY MGR) MGR_SUM
FROM EMP;
```

OVER 절에서 ORDER BY 절을 추가해서 파티션 내 데이터를 정렬하고 이전 급여 데이터까지의 누적값을 출력하는 예제
```sql
SELECT MGR, ENAME, SAL,
       SUM(SAL) OVER (PARTITION BY MGR ORDER BY SAL RANGE UNBOUNDED PRECEDING) MGR_SUM
FROM EMP;

RANGE UNBOUNDED PRECEDING; # 현재 행을 기준으로 파티션 내의 첫 번째 행까지 범위를 지정
```

### MAX
파티션별 윈도우의 최대값을 구할 수 있다.
  
사원들의 급여와 같은 매니저를 두고 있는 사원들의 급여 중 최대값을 구하는 예제
```sql
SELECT MGR, ENAME, SAL,
       MAX(SAL) OVER (PARTITION BY MGR) MGR_MAX
FROM EMP;
```

INLINE VIEW를 이용해서 파티션별 최대값을 가진 행만 추출할 수도 있다.

```sql
SELECT MGR, ENAME, SAL
FROM (SELECT MGR, ENAME, SAL,
             MAX(SAL) OVER (PARTITION BY MGR) IV_MAX_SAL
             FROM EMP )
WHERE SAL = IV_MAX_SAL;
```

### MIN
파티션별 윈도우의 최소값을 구할 수 있다.  
  
사원들의 급여와 같은 매니저를 두고 있는 사원을 입사일자 기준으로 정렬하고, 급여 최소값을 같이 구하는 예제

```sql
SELECT MGR, ENAME, HIREDEAT, SAL,
       MIN(SAL) OVER (PARTITION BY MGR ORDER BY HIREDATE) MGR_MIN
FROM EMP;
```

### AVG

파티션별 통계값을 구할 수 있다.  
  
같은 매니저를 두고 있는 사원들의 평균 급여를 구하되, 같은 매니저 내에서 자기 바로 앞의 사번과 바로 뒤의 사번인 직원을 대상으로만 하는 예제(앞 줄 + 자신 + 뒷 줄의 합을 3으로 나누는 형식, 만약 앞줄이 없다면 나 + 뒷 줄의 합을 2로 나누게 된다.)

```sql
SELECT MGR, ENAME, HIREDEAT, SAL,
       ROUND(AVG(SAL)) OVER (PARTITION BY MGR ORDER BY HIREDATE)
       ROW BETWEEN 1 PRECEDING AND 1 FOLLOWING) ) MGR_AVG
FROM EMP;
```

### COUNT

사원들을 급여 기준으로 정렬하고, 본인 급여보다 50 이하 적거나, 150 이하로 많은 급여를 받는 인원수를 출력하는 예제

```sql
SELECT ENAME , SAL, 
       COUNT(*) OVER (ORDER BY SAL
       RANGE BETWEEN 50 PRECEDING AND 150 FOLLOWING) SIM_CNT
FROM EMP;

RAGE BETWEEN 50 PRECEDING AND 150 FOLLOWING;
```

현재 행의 급여값을 기준으로 -50 ~ +150 범위 내에 포함된 모든 행이 대상이 된다.
  
RANGE는 현재 행의 데이터 값을 기준으로 앞 뒤 데이터 값의 범위를 표시함.

<br>

## 그룹 내 행 순서 관련 함수

### FIRST_VALUE
파티션별 윈도우에서 가장 먼저 나온 값을 구할 수 있다.  
  
sql server 에서는 지원하지 않는다.
  
MIN 함수를 이용해도 같은 결과를 얻을 수 있다.  
  
부서별 직원들을 연봉이 높은 순서부터 정렬하고, 파티션 내에서 가장 먼저 나온 값을 출력하는 예제

```sql
SELECT DEPTNO, ENAME, SAL,
       FIRST_VALUE(ENAME) OVER (PARTITION BY DEPTNO ORDER BY SAL DESC
       ROWS UNBOUNDED PRECEDING) DEPT_RICH
FROM EMP;
```

같은 급여를 받는 사람이 있다면 정렬을 지정해야 한다.  
FIRST_VALUE는 공동 등수를 인정하지 않고 처음 나온 행을 처리하기 때문이다.
  
정렬을 추가한 예제, 같은 급여가 있다면 이름 오름차수능로 나오게 된다.

```sql
SELECT DEPTNO, ENAME, SAL,
       FIRST_VALUE(ENAME) OVER (PARTITION BY DEPTNO ORDER BY SAL DESCENAME ASC)
       ROWS UNBOUNDED PRECEDING) DEPT_RICH
FROM EMP;
```

### LAST_VALUE
파티션별 윈도우에서 가장 먼저 나중에 나온 값을 구할 수 있다.
  
sql server에서는 지원하지 않음.
  
MAX 함수를 이용해도 같은 결과를 얻을 수 있다.
  
부서별 직원들을 연봉이 높은 순서부터 정렬하고, 파티션 내에서 가장 먼저 나중에 나온 값을 출력하는 예제

```sql
SELECT DEPTNO, ENAME, SAL,
       LAST_VALUE(ENAME) OVER (PARTITION BY DEPTNO ORDER BY SAL DESC ROW BETWEEN CURRENT ROW AND UNBOUNDED FOLLOWING) DEPT_POOR
FROM EMP;

ROW BETWEEN CURRENT ROW AND UNBOUNDED FOLLOWING # 현재 행을 포함해서 파티션 내의 마지막 행까지의 범위를 지정한다.
```

### LAG

파티션별 윈도우에서 이전 몇 번째 행의 값을 가져올 수 있다.  
  
sql server에서는 지원하지 않음.
  
직원들을 입사일자가 빠른 기주으로 정렬하고, 본인보다 입사 일자가 한 명 앞선 사원의 급여를 본인의 급여와 함께 출력하는 예제

```sql
SELECT ENAME, HIREDATE, SAL,
       LAG(SAL) OVER (ORDER BY HIREDATE) PREV_SAL
FROM EMP
WHERE JOB = 'SALESMAN';
```
실행 결과
```
[실행 결과] 
ENAME   HIREDATE  SAL  PREV_SAL 
------- --------- ---- ------- 
ALLEN   1981-02-20 1600 
WARD    1981-02-22 1250 1600 
TURNER  1981-09-08 1500 1250 
MARTIN  1981-09-28 1250 1500 

4개의 행이 선택되었다.
```

LAG함수는 3개의 인수까지 사용이 가능하다.  
  
LAG(SAL,2,0) <- 두번째 인수는 몇 번째 앞의 행을 가져올지 결정하는 것이고 디폴트는 1 여기서는 2를 지정했으니 2번째 앞에 있는 행을 가져오는 것이다.
  
세번째 인수는 파티션 첫 번째 행의 경우 가져올 데이터가 없어 null이 들어오는데, 이 경우 다른 값으로 바꾸어줄 수 있다. NVL/ISNULL과 같은 기능

```sql
SELECT ENAME, HIREDATE, SAL,
       LAG(SAL,2,0) OVER (ORDER BY HIREDATE) PREV_SAL
FROM EMP
WHERE JOB = 'SALESMAN';
```

실행결과
```
[실행 결과] 
ENAME   HIREDATE  SAL  PREV_SAL 
------- --------- ---- ------- 
ALLEN   1981-02-20 1600 0
WARD    1981-02-22 1250 0 
TURNER  1981-09-08 1500 1600 
MARTIN  1981-09-28 1250 1250 

4개의 행이 선택되었다.
```

### LEAD
파티션별 윈도우에서 이후 몇 번째 행의 값을 가져올 수 있다.  
  
sql server에서는 지원하지 않는다.
  
직원들을 입사일자가 빠른 기준으로 정렬하고, 바로 다음에 입사한 인력의 입사 일자를 함께 출력하는 예제

```sql
SELECT ENAME, HIREDATE
       LEAD(HIREDATE) OVER (ORDER BY HIREDATE) NEXT_HIRED
FROM EMP;
```
실행 결과
```
[실행 결과] 
ENAME    HIREDATE   NEXTHIRED 
-------- ---------  --------- 
ALLEN    1981-02-20 1981-02-22 
WARD     1981-02-22 1981-04-02 
TURNER   1981-09-08 1981-09-28 
MARTIN   1981-09-28 

4개의 행이 선택되었다.
```

LAG처럼 LEAD 함수도 3개의 인수까지 사용할 수 있다.

<br>

## 그룹 내 비율 관련 함수

### CUME_DIST
파티션별 윈도우의 전체 건수에서 현재 행보다 작거나 같은 건수에 대한 누적 백분율을 구한다.
  
sql server 에서는 지원하지 않는다.
  
같은 부서 소속 사원들의 집합에서 본인의 급여가 누적 순서상 몇 번째 위치쯤에 있는지 0~1 사이의 값으로 출력하는 예제다.

```sql
SELECT DEPTNO, ENAME, SAL,
       CUME_DIST() OVER (PARTITION BY ORDER BY SAL DESC) CUME_DIST
FROM EMP;
```

### PERCENT_RANK
파티션별 함수를 이용해 파티션별 윈도우에서 제일 먼저 나오는 것을 0으로, 제일 늦게 나오는 것을 1로 하여, 행의 순서별 백분율을 구한다. (값이 아니라 행의 순서별 백분율이다.)
  
sql server에서는 지원하지 않는다.
  
같은 부서 소속 사원들의 집합에서 본인의 급여가 순서상 몇 번째 위치해 있는지 0과 1 사이의 값으로 출력하는 예제다
```sql
SELECT ENAME, SAL,
       PERCENT_RANK() OVER (PARTITION BY DEPTNO ORDER BY SAL DESC) P_R
FROM EMP;
```

```
[실행 결과] 
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

14개의 행이 선택되었다.
```

DEPTNO 10의 경우 3건 이므로 구간을 2가 된다.  
0과 1 사이를 2개의 구간으로 나누면 0, 0.5, 1이 된다. 
  
DEPTNO 20의 경우 5건 이고, 구간은 4.  
0과 1 사이를 4개 구간으로 나누면 0, 0.25, 0.5, 0.75, 1이 된다. 
  
DEPTNO 30의 경우 6건 이고, 구간은 5.  
0과 1 사이를 5개 구간으로 나누면 0, 0.2, 0.4, 0.6, 0.8, 1이 된다. 


### NTILE
파티션별 전체 건수를 ARGUMENT 값으로 N등분한 결과를 구할 수 있다.
  
전체 사원을 급여가 높은 순서로 정렬하고, 급여를 기준으로 4개 그룹으로 분류한다.

```sql
SELECT ENAME, SAL,
       NTILE(4) OVER (ORDER BY SAL DESC) QUAR_TILE
FROM EMP; 
```

```
[실행 결과] 
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

14개의 행이 선택되었다.
```

NTIL(4)의 의미는 14명의 팀원을 4개의 조로 나눈다는 의미이다.  
14명을 4개의 집합으로 나누면 몫이 3, 나머지가 2가된다.  
나머지 두 명의 앞의 조부터 할당된다.  
4 + 4 + 3 + 3으로 조를 나누게 된다.

### RATIO_TO_REPORT
파티션 내 전체 SUM(컬럼) 값에 대한 행별 컬럼 값의 백분율을 소수점으로 구할 수 있다.  
결과값은 > 0 & <= 1의 범위를 가진다.  
개별 ratio의 합을 구하면 1이 된다.  
sql server에서 지원하지는 않는다.  
세일즈맨 대상으로, 전체 급여에서 본인이 차지하는 비율을 구하는 예제

```sql
SELECT ENAME, SAL,
       ROUND(RATIO_TO_REPORT(SAL) OVER (), 2) P_R
FROM EMP
WHERE JOB = 'SALESMAN'
```

```
[실행 결과] 
ENAME  SAL   R_R 
------ ---- ---- 
ALLEN  1600 0.29 (1600 / 5600) 
WARD   1250 0.22 (1250 / 5600) 
MARTIN 1250 0.22 (1250 / 5600) 
TURNER 1500 0.27 (1500 / 5600) 

4개의 행이 선택되었다.
```