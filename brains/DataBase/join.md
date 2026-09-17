# Join 연산 정리

## Join이란
**두 개 이상의 테이블들을 연결 또는 결합하여 데이터를 출력하는 것**
  
일반적인 경우 행들은 PK, FK 값의 연관에 의해 Join이 성립되지만, 어떤 경우에는 이러한 pk, fk의 관계가 없어도 논리적인 값들의 연관만으로 Join이 성립이 가능하다.

## EQUI Join vs Non EQUI Join

### EQUI JOIN
- EQUI JOIN(등가 조인)은 두 개의 테이블 간에 컬럼 값들이 서로 정확하게 일치하는 경우에 사용되는 방법으로 대부분 PK->FK 관계를 기반으로 한다.
- 하지만, 반드시 PK -> FK의 관계로만 EQUI JOIN이 성립하는 것은 아니다.

### Non EQUI JOIN
- Non EQUI JOIN(비등가 조인)은 두 개의 테이블 간에 컬럼 값들이 서로 정확하게 일치하지 않는 경우에 사용된다.
- Non EQUI JOIN의 경우에는 = 연산자가 아닌 다른 연산자 (Between, >, >=, <, <= 등) 연산자들을 사용해 Join을 수행한다.

## INNER JOIN
INNER JOIN은 OUTER JOIN과 대비된 개념, 내부 조인이라고 불린다.
- Join 조건에서 동일한 값이 있는 행만 반환한다.
- INNER JOIN 표시는 JOIN 조건을 FROM 절에서 정의하겠다는 표시이므로 USING 조건절이나 ON 조건절을 필수적으로 사용해야 하며, INNER는 SQL문 내에서 생략 가능하다.

### NATURAL JOIN
두 테이블 간의 동일한 이름을 갖는 모든 컬럼들에 대해 EQUI JOIN을 수행한다.
- NATURAL JOIN이 명시되면, 추가로 USING 조건절, ON 조건절, WHERE 절에서 JOIN 조건을 정의할 수 없다.
- JOIN에 사용된 컬럼들은 같은 데이터 유형이어야 하며, ALIAS나 테이블 명과 같은 접두사를 붙일 수 없다.
- JOIN이 되는 테이블의 데이터 성격(도메인)과 컬럼명 등이 등일해야하는 제약 조건이 있다.

### USING 조건절
- FRON 절의 USING 조건절을 이용하면 같은 이름을 가진 컬럼들 중에서 원하는 컬럼에 대해서만 선택적으로 EQUI JOIN을 할 수가 있다.
- JOIN + USING 조건절 또한 NATURAL JOIN과 마찬가지로 JOIN 컬럼에 대해서는 ALIAS나 테이블 이름과 같은 접두사를 붙일 수 없다.

### ON 조건절
- JOIN 서술부 (ON 조건절)와 비 JOIN 서술부(WHERE 조건절)를 분리하여 이해가 쉽고, 컬럼명이 다르더라도 조인 조건을 사용할 수 있는 장점이 있다.
- 임의의 조인 조건 지정, 이름을 다른 컬럼명을 조인 조건으로 사용, 조인 컬럼을 명시하기 위해 사용한다.
- JOIN + USING 조건절과 달리, ON 조건절을 사용한 조인의 경우에는 ALIAS나 테이블 명과 같은 접두사를 사용하여 SELECT를 사용되는 컬럼을 논리적으로 명확하게 지정해줘야 한다.

### CROSS JOIN
- 테이블 간 JOIN 조건이 없는 경우 생길 수 있는 모든 데이터의 조합을 출력
- CARTESIAN PRODUCT와 같은 의미

테이블 들의 컬럼 수의 모든 곱셈 연산이 생김

## OUTER JOIN
INNER JOIN과 대비하여 외부 조인이라고 불린다.
  
조인 조건에서 동일한 값이 없는 행도 반환할 때 사용할 수 있.
  
외부 조인또한 조인 조건을 FROM 절에서 정의하겠다는 표시이므로 USING 조건절이나 ON 조건절을 필수적으로 사용해야 한다.

### LEFT OUTER JOIN
- 조인 수행시 먼저 표기된 좌측 테이블에 해당하는 데이터를 먼저 읽은 후, 나중 표기된 우측 테이블에서 조인 대상 데이터를 읽어온다.
- LEFT JOIN으로 OUTER 키워드를 생략해서 사용할 수 있다.

### RIGHT OUTER JOIN
- 조인 수행시 먼저 표기된 우측 테이블에 해당하는 데이터를 먼저 읽은 후, 나중 표기된 좌측 테이블에서 JOIN 대상 데이터를 읽어온다,.
- RIGHT JOIN으로 OUTER 키워드를 생략해서 사용할 수 있다.

### FULL OUTER JOIN
- 조인 수행시 좌측, 우측 테이블의 모든 데이터를 읽어 조인하여 결과를 생성한다.
- FULL JOIN으로 OUTER 키워드를 생략해서 사용할 수 있다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FcE2Cxi%2Fbtq0jNn1Phl%2Fx0YvaIzGmlIf9DZNY8yPb0%2Fimg.png)