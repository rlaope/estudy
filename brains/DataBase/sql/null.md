# SQL NULL 관련 함수 - NVL, NVL2, NULLIF, COALESCE

## SQL NULL 관련 함수
SQL의 NULL 관련 함수인 NVL, NVL2, NULLIF, COALESCE를 한번 알아보자.

### NVL
NULL이면 다른 값으로 변경하는 함수
  
NVL(K, 0)은 K 컬럼이 NULL이면 0으로 바꿈

### NVL2
NVL 함수와 DECODE 함수가 하나로 합쳐진 함수  
NVL2(K, 1, -1)은 K컬럼이 NULL이 아니면 1을 NULL이면 -1을 반환

### NULLIF
두개의 값이 같으면 NULL을 같지 않으면 첫번째 값을 반환하는 함수  
  
NULLIF(exp1, exp2)은 exp1과 exp2가 같으면 NULL을 같지 않으면 exp1을 반환

### COALESCE
NULL이 아닌 최초의 인자 값을 반환하는 함수
  
COALESCE(exp1, exp2, exp3, ...)은 exp1부터 그 뒤로 차례대로 NULL인지 확인, 만약 exp2가 최초의 NULL이 아닌 값이면 exp2를 반환