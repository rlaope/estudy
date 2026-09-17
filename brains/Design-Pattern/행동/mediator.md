# 중재자 패턴

중재자 패턴이란 각 컴포넌트 간의 상호작용을 중재자에게 맡김으로써 상호 의존성을 낮줄 수 있게 해주는 디자인 패턴이다.


## 예시

예를 들어 Customer라는 클래스와 Restaurant, TowelService가 있다고 해보자.

Customer는 restaurant에서 저녁을 먹을 수 있고 TowelService를 통해 수건을 받을 수 있다.

그렇다면 Customer는 저 두 클래스들을 의존하여 getTowel, dinner라는 메서드를 구현할 텐데 이렇게 되면 상호 의존성이 있기 때문에, 결합도가 올라간다.

여기서 FrontDesk라는 중재자를 둠으로써 customer를 인자로 받고 나머지 restaurant, towerService들을 사용해 customer 정보를 토대로 작업을 처리해준다면, customer와 다른 서비스들간의 의존성을 없앨 수 있다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FrkCOt%2FbtqwA3d73qA%2FWzBqVmAnHxKxgR3W3T6Zc0%2Fimg.png)

### 장점

장점으로는 상호간 의존성을 떨어트리기 때문에 결합도가 낮은 코드를 작성할 수 있다.

여러가지 클래스들이 생기더라도 Client는 중재자 하나만 알고있으면 되기 때문에 확장이 용이하다.

### 단점

확장이 용이하고 결합도가 높은 클래스 구성도를 만들 수 있지만, 하나의 단점은 **중재자 클래스의 복잡성**이 있다. 중재자 클래스는 관련 서비스들에 대한 모든 의존성을 가지고 있기 때문이다.

그러나 이러한 문제와 중재자 패턴으로 얻을 수 있는 메리트를 비교해보았을때 중재자 패턴을 사용하는 것이 더욱 이점이 있다고 생각되기 때문에, 결국 중재자 패턴은 적용할 만한 패턴이라고 생각된다.

<br>

### 스프링에서 찾아보는 예시

Spring에서 대표적으로 `DispatcherServlet`(front controller) 클래스가 이러한 패턴을 토대로 개발되었다.

handlerMapping 과 같이 스프링 MVC에서 요청과 응답에 중재자 역할을 해준다.