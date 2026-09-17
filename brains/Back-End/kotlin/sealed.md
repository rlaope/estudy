# sealed class

sealed class의 등장 배경을 살펴보자 하나의 부모 클래스를 상속받았다고 했을 때 **컴파일러는 부모 클래스를 상속받은 자식 클래스들이 있는지 알지 못한다.**

예를 들어서 런닝 운동을 기록해주는 앱을 만들고 있다 했을 때, 사람의 상태를 클래스로 만들어 보겠다. 상태의 종류는 Running, Waliking, Idle 상태가 있고 이는 다음과 같은 코드로 변환할 수 있다.

```kotlin
abstract class PersonState

class Running : PersonState()
class Walking : PersonState()
class Idle : PersonState()
```

각 PersonState별로 상태 메세지를 얻는 함수를 작성해보자.

```kotlin
fun getStateMessage(personState: PersonState): String {
    return when (personState) {
        is Running -> "Person is running"
        is Walking -> "Person is walking"
        is Idle -> "Person is doing nothing"
    }
}
```

하지만 위의 코드는 아래와 같이 else 분기를 추가하라고 오류를 띄운다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fb2p56T%2Fbtrd5jePobA%2FLnzixISyxpt9dblcUXa58k%2Fimg.png)

이를 해결하기 위해 else를 추가하면 해결이 되기는 한다..

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fdrgxol%2Fbtrd05a3Yar%2FkNxxjUKGR2iMjUmYKrEY0k%2Fimg.png)

else를 추가하라고 한 이유는 무엇일까? 바로 위에 언급한 것 처럼 컴파일러가 PersonState의 자식 클래스 종류를 알지 못해서 그렇다.

이 문제는 단순한 문제가 아니다. 한 번 저 when 문에서 Idle 검사를 지워보겠다. 그러면 오류없이 컴파일 되는 것을 확인할 수 있다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fbf9Ez5%2Fbtrd04JXPSM%2FpnIw4NEQXlvUMOKpnCMmqK%2Fimg.png)

이는 실제 사용되는 애플리케이션 런타임에 올라갔다면 Idle 상태 처리가 되어있지 않아 제대로 동작하지 않을 것이다. 즉, 코드에 문제가 있더라도 컴파일 타임에서 잡을 수 없는 치명적인 문제가 발생하게 된다.

이러한 문제들을 sealed class가 해결한다.

### sealed class

sealed class는 abstract class로 상속 받는 자식 클래스의 종류를 제한하는 특성을 가지고 있다. 즉, **컴파일러에서 sealed class의 자식 클래스가 어떤 것이 있는지 알 수 있다.**

```kotlin
package com.example.demo

sealed class PersonState

class Running : PersonState()
class Walking : PersonState()
class Idle : PersonState()
```

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbLcMR4%2Fbtrd642NmG2%2FmmcKRdFK7aOrgPyqK29FJK%2Fimg.png)

위와 같이 else 분기를 추가하지 않아도 오류가 생기지 않는데 이는 컴파일러에서 sealed class PersonState의 자식 클래스는 Running, Walking, Idle 이라는 것을 알고있기 때문이다. 따라서 when에서 else branch를 사용하지 않고도 필요한 메세지만 수신할 수 있게 된다.

만약 우리가 앱을 확장해서 RunningFast라는 상태를 추가한다고 해보자


![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FVA6FL%2Fbtrd2yjiK9F%2FIr9Mg8QNfZr1YwfOZLWf00%2Fimg.png)

확인해보면 RunningFast라는 상태를 추가하지 않아 오류가 띄워지는 것을 볼 수 있다. RunningFast에 대한 분기를 처리하면 오류는 사라진다.

### sealed class 상속받기

1. class로 상속받기.

위에서 class 키워드를 통해 상속받은 것을 확인했었다. 하지만 이상한점은 노란색 줄로 경고문이 표시되어있다는 것을 눈치 빠른 분들은 벌써 알았을 수도 있다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2F3ftck%2Fbtrd5kSnFLd%2FzPhNXlKhVRTxduqzjfpgQ0%2Fimg.png)

> 'sealed' subclass has no state and no overridden 'equals()'   
> 해석: sealed class의 subclass가 상태가 없고 equals()를 오버라이드 하지도 않는다.

즉, 상태 변수가 있거나 equals를 오버라이드할 경우에만 class 키워드를 통해 상속받으라는 이야기다. 그 외에는 메모리 절약을 위해 object를 이용한다. 따라서 위의 코드를 다음과 같이 바꾸면 warning 구문이 없어진다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FcFKrgq%2Fbtrd05a3Ybw%2FwPI5I1MLumEczD8jZzCjlk%2Fimg.png)


2. object로 상속받기

object는 싱글톤 패턴으로 한 번만 메모리에 올라가고 재사용된다. 따라서 상태가 없는 경우에는 객체를 두 번 이상 생성하여 메모리에 올리는 것은 메모리 낭비로 볼 수 있다. 그렇기에 상태 변수가 없는 값을 모두 object로 바꾸면 warning 구문이 없어진다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FnvAXB%2Fbtrd2wZ6vSa%2FIMumPdG7HSfzcM0XTs1igK%2Fimg.png)

<br>

### sealed class 특징

같은 패키지의 자식 클래스만 상속 가능하다.

컴파일러가 모든 패키지를 탐색하여 자식 클래스들을 찾는 것은 너무 많은 리소스를 소모하는 작업이기 때문에 **sealed class는 자식 클래스에 대한 선언을 같은 패키지 내로 제한한다.**

sealed class가 `com.example.demo.controller` 안에 존재한다면 상속 받는 모든 클래스들도 그 패키지 안에 존재해야하고 그렇지 않는다면 다음과 같이 오류가 나온다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbfmNkU%2Fbtrd03j1moY%2FSOwlL7kQKMk3djjOLvBMlK%2Fimg.png)


> Inheritor of sealed class or interface declared in package com.example.demo.controller but it must be in package com.example.demo where base class is declared  
> 
> **해석: Running class가 com.example.demo에 선언된 경우에만 sealed class를 상속 가능하다.  
> **


그리고 추상클래스로 직접 객체 인스턴스 생성이 불가능하다.

sealed class는 abstract class로 직접 인스턴스 생성이 불가하다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbdjeDu%2Fbtrd026sBQO%2FBEoAuelJGRN14tMUKeXpj1%2Fimg.png)

> Sealed types cannot be instantiated  
> 해석: sealed class는 인스턴스화 될 수 없다.
