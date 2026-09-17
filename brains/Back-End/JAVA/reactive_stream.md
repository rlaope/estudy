# Reactive Stream, Backpressure, API Components


Reactive Stream은 non-blocking, backpressure을 이용하여 비동기 서비스를 할 때 기본이 되는 스펙입니다. java의 RxJava, Spring5 WebFlux의 Core에 있는 ProjectReader 프로젝트 모두 해당 스펙을 따르고 있습니다.

Java9에 추가된 Flow 역시 Reactive Stream의 스팩을 채택해서 사용하고 있습니다.

계속적으로 들어오는 스트림 데이터를 처리하기위해서는 비동기 시스템이 효과적입니다.

비동기 처리를 하면서 가장 중요한 문제는 데이터 처리가 목적지 리소스 소비를 예측 가능한 범위에서 신중하게 제어할 수 있어야하는 것입니다.

Reactive Stream의 주된 목적은 비동기의 경계를 명확히하여 스트림 데이터의 교환을 효과적으로 관리하는 것 입니다. 즉, 비동기로 데이터를 처리하는 시스템에 어느정도 데이터가 들어올 지 예측가능하도록 하는 것입니다. **BackPressure**이 이를 달성할 수 있게 해주는 중요한 부분이라 할 수 있습니다.

1. 잠재적으로 무한한 숫자의 데이터 처리
2. 순차적으로 처리
3. 컴포넌트간에 데이터를 비동기적을 전달
4. 백프레셔를 이용한 데이터 흐름 제어

### BackPressure
위에서 계속 백프레셔를 언급했는데 한 번 무엇인지 알아보겠습니다.

일단 리액티드 선언문의 용어집을 보면 아래와 같이 설명합니다.

> 한 컴포넌트가 부하를 이겨내기 힘들 때, 시스템 전체가 합리적인 방법으로 대응해야 한다. 과부하 상태의 컴포넌트에서 치명적인 장애가 발생하거나 제어 없이 메시지를 유실해서는 안 된다. **컴포넌트가 대처할 수 없고 장애가 발생해선 안 되기 때문에 컴포넌트는 상류 컴포넌트들에 자신이 과부하 상태라는 것을 알려 부하를 줄이도록 해야 한다.** 이러한 배압은 시스템이 부하로 인해 무너지지 않고 정상적으로 응답할 수 있게 하는 중요한 피드백 방법이다. 배압은 사용자에게까지 전달되어 응답성이 떨어질 수 있지만, 이 메커니즘은 부하에 대한 시스템의 복원력을 보장하고 시스템 자체가 부하를 분산할 다른 자원을 제공할 수 있는지 정보를 제공할 것이다.

쉽게 말해서 컴포넌트가 과부화상태일때 상류 컴포넌트들에게 자신의 상태를 알려 부하를 줄이는 것입니다.

<br>

## API Components

Reactive Stream API의 구성요소는 아래와 같습니다.

1. Publisher
2. Subscriber
3. Subscription
4. Processor

Publisher는 무한한 data를 제공하며 제공되어진 data는 Subscriber가 구독하는 형식으로 이루어집니다.

Publisher.subscribe(Subscriber) 형식으로 구독자가 연결을 맺게 됩니다.

실행 순서는 다음과 같습니다. onSubscribe onNext* (onError | onComplete)?

onSubscribe는 Publisher가 생산하는 data를 Subscriber가 항상 신호를 받을 준비가 되어있다는 의미이며, onNext로 데이터를 수신합니다.

그리고 실패가 있는경우에는 onError() 신호, 더 이상 사용할 수 있는 신호가 없을 경우에는 onComplete() 신호를 호출합니다.

이는 Subscription이 취소될때까지 지속됩니다.

```java
public interface Publisher<T> { 
	public void subscribe(Subscriber<? super T> s); 
}

public interface Subscriber<T> { 
	public void onSubscribe(Subscription s); public void onNext(T t);          public void onError(Throwable t);
	public void onComplete(); 
}

public interface Subscription { 
	public void request(long n); public void cancel(); 
}
```

위와같이 클래스들이 명세되어있습니다.

위의 인터페이스들을 토대로 아래와 같은 플로우가 구성이됩니다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FzmVzg%2FbtqFkUlWOkd%2FiKCzp1MgvKthItuXLUCYO1%2Fimg.png)

1. Publisher는 본인이 소유할 Subscription을 구현하고 publishing할 데이터를 만듭니다.
2. Publisher는 subscribe() 메서드를 통해 subscriber를 등록합니다.
3. Subscriber는 onSubscribe() 메서드를 통해 Subscription 등록합니다. 그리고 Publisher를 구독하기 시작합니다. 이는 Publisher에 구현된 Subscription을 통해 이루어지고 Publisher와 Subscriber는 Subscription을 통해 연결된 상태가 됩니다. onSubscribe() 내부에 Subscription의 request()를 요청하면 그때부터 data 구독이 시작됩니다.
4. Subscriber는 Subscription request()또는 cancel()을 호출을 통해 데이터의 흐름을 제어합니다
5. 1. Subscription의 request()에는 조건에 따라 Subscriber의 onNext(), onComplete() 또는 onError()를 호출합니다. 그러면 Subscriber의 해당 메서드의 로직에 따라 request()또는 cancel()로 제어하게 됩니다.