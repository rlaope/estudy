# 인터프리터 패턴 

인터프리터 패턴은 **자주 사용되는 표현식 expression을 별개의 언어로 정의하고 재사용**하는 디자인 패턴이다.

간단한 언어(규칙)의 문법을 정의하고 해석하는 패턴이라고 볼 수 있다 (ex.정규식 분석)



![](https://upload.wikimedia.org/wikipedia/commons/thumb/b/bc/Interpreter_UML_class_diagram.svg/1920px-Interpreter_UML_class_diagram.svg.png)

- TerminalExpression은 문법 규칙에서의 기본 단위, 최하위 단위로 이 클래스는 해석할 문법 규칙의 최소 단위를 구한다.  
- NonTerminalExpression 복합 표현식의 구성 요소다. 이 클래스는 문법 규칙에서의 구성 요소를 해석한다. 문법의 구조를 나타내기 위해 사용된다.

예를 들면 사칙연산 후위 표기식을 보겠다 xyz+-에 대해서 x y z 자제는 TerminalExpression + -를 해석해 관련 TerminalExpression을 사용해 연산을 처리하는 Expression이 NonTerminalExpression이라고 보면 된다.

> Expression 대표 인터페이스를 구현한 클래스들로 여러 Expression들을 구현할 수 있지만 자바 8부터 static 메서드를 사용해서 구현할 수도 있다.

<br>

### 예시

대표적으로 스프링에서도 찾아볼 수 있는데 `@Value("#{}")` 어노테이션이다. #{} 안에 SpEL 표기식을 사용해 properties를 참조할 수 있다. 마찬가지로 SpEL이라는 표현식(Expression)을 해석한 것이다.

그리고 정규표현식 Pattern 클래스를 사용해 regax를 검사하는 것도 인터프리터 패턴을 사용한 것으로 볼 수 있다.

<br>

### 장점

장점으로는 여러 패턴 표헌식에 대한 클래스를 만들어 자유로운 확장이 가능하다. OCP

### 단점
단점으로는 디버깅이 조금 어렵고, 복잡한 문법 또는 많은 표현식을 처리할 때 성능에 영향을 줄 수 있다. 그렇기 때문에 꼭 자주 사용되는 표현에서만 관련 패턴을 적용하도록 하자
