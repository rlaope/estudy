# State Machine Pattern

State Machine Pattern은 **내부 상태가 변할 때 객체의 동작을 변경하도록 하기 위하는 패턴**이다.

상태와 상태에 대한 행동이 주어지고, 상태가 변할 때 그 상태에 대한 행동을 실행하고자 할때, 객체의 상태와 상태 별 행동을 정의하고 새로운 상태가 추가되거나 기존의 상태에 대한 다른 상태와는 독립적으로 변경하고자 할 때

![](https://user-images.githubusercontent.com/35602698/103554291-00f46180-4ef2-11eb-9457-0efae99578e2.png)

위와 같이 상태와 상태에 대한 행동이 주어지고, 상태가 변할 때 그 상태에 대한 행동을 실행하도록 하는 구조를 구현하는 방법 중 하나는 상태별 행동을 직접 내부 상태에 의존하는 클래스에 정의하는 것이다. 상태와 상태별 행동을 클래스 내부에 정의할 때는 조건문이나 switch 문을 사용하면 된다. 각 조건 브랜치는 상태에 따른 행동들을 구현한다.

하지만 이런 방법은 클래스를 특정 상태별 행동을 정의하고 나중에 클래스로부터 독립적으로 새로운 상태를 추가하거나 기존의 행동을 변경하는 것이 불가능하고 상태별 동작을 포함하는 클래스들은 구현, 변경, 테스트, 재사용등을 기대하기는 더욱 더 어려울 것이다. 이러한 문제 때문에 GoF에서 새로운 상태를 추가하고 기존 상태에 대한 행동이 독립적으로 변할 수 있게 하는 여러 접근법을 제공한다.

![](https://user-images.githubusercontent.com/35602698/103554326-136e9b00-4ef2-11eb-80f7-cb4292feaa3e.png)

위와 같이 상태별 행동을 State 객체로 분리시킨 후 캡슐화 한다.

상태를 가지는 클래스는 직접 상태별 행동을 자신의 클래스 내부에 구현하지 않고 상태별 행동을 현재 State 객체에게 위임한다.

이 패턴의 핵심 아이디어는 객체의 상태별 행동들을 State 객체로 캡슐화 하는 것이다. 이렇게 캡슐화 하게 된다면 이 점은 객체의 상태를 별도의 State 객체로 관리해 다른 State 객체와 독립적으로 변경할 수 있도록 한다.

State 객체의 구현은 다음과 같다.
- 모든 상태에 대해, State interface 정의, state interface에는 상태에 대한 행동을 정의하도록 하는 operation() 메서드를 정의한다.
- 각 상태들은 state interface를 구현하는 concrete class를 정의한다.
- 현재 상태가 변경되기만 하면 행동이 변경되기 때문에 별도의 조건문이 필요 없다.
- 상속 때문에 compile time에 대한 유연성을 가진다. 모든 상태 별 코드는 state 서브클래스로 살아있고, 새로운 상태를 추가하는 것은 새로운 state 서브 클래스를 정의함으로써 쉽게 가능하다.

상태를 가지는 클래스는 상태 별 행동을 실행하는 책임을 state객체로 위임한다. object composition 때문에 run time에 대한 유연성을 가진다. 런타임 시점에서 현재 상태를 변경함으로써 그의 행동을 변화시킬 수 있다.

- Context: 상태를 가지는 클래스 
- State: 상태 클래스
- State N: 상태 구현 클래스

### Dynamic Object Collaboration

Context 객체는 상태 별 행동을 현재 상태 객체에 위임하고 상호작용은 현재 상태 객체애 대한 행동을 호출하는 Context 객체에서 시작된다. Context 객체는 자기 자신을 State 객체에게 전달한다. 각 State는 자신의 행동을 실행한다. 그리고 행동이 실행되고 상태가 변경되어야만 한다면 context.setState()를 호출하여 context 객체를 다른 상태로 변경한다.


이 패턴은 새로운 상태를 쉽게 추가할 수 있으며 조건문을 사용하지 않아도 되며 일관된 상태를 보장한다. Context의 상태는 현재 State 객체에 의해서만 변경될 수 있기 때문에 상태에 대한 일관성이 보장되며 상태변경이 암묵적으로 이루어진다.

그러나 컨텍스트 인터페이스를 확장해야 할 수 도 있으며 State 객체가 Context의 상태를 변경하도록 Context인터페이스를 확장해야할 수도 있다.

> Introduces an additional level of indirection. State achieves flexibility by introducing an additional level of indirection (clients delegate to separate State objects), which makes clients dependent on a State object.

