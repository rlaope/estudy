# 빈 스코프와 발생할 수 있는 여러 문제들과 해결 방법


## 빈 스코프 종류

빈 스코프는 다음과 같은 종류가 있다.
- 싱글톤 빈
- 프로토타입 빈
- 웹 스코프
	- request
	- session
	- application
	- websocket 

스프링에서는 @Scope 어노테이션으로 스코프를 지정해줄 수 있다.

```java
@Scope("prototype")
public class MyPrototypeBean {
	...
}
```



### 싱글톤 빈
말 그대로 싱글톤으로 관리되는 빈이다.

스프링 빈의 디폴트 값이다. 스프링 컨테이너는 기본적으로 빈을 싱글톤으로 관리한다.

스프링 컨테이너는 싱글톤 빈의 인스턴스를 반환할 때 항상 같은 인스턴스를 반환한다.



### 프로토타입 빈
싱글톤 빈과 다르게 스프링 컨테이너에서 빈을 조회할 때마다 항상 새로운 인스턴스를 생성해서 반환한다.

스프링 컨테이너는 프로토타입 빈을 생성하고, DI, 초기화까지만 생성한 후 클라이언트에게 빈을 반환하고 그 이후에는 **생성된 프로토타입빈을 관리하지 않는다.**

즉 @PreDestroy와 같은 종료 메서드가 호출되지 않는다.

<br>

## 싱글톤과 프로토타입 스코프 빈을 같이 사용할 때 문제

싱글톤 빈과 프로토타입 빈을 같이 사용하게 된다면 예상한 것 처럼 동작하지 않을 수 있다

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbVMTvI%2FbtrosV10NmS%2F7Q5Mn8y6h1JiCZqpSvrrYk%2Fimg.png)

1. clientBean은 DI를 통해 사용된다. 주입 시점에 스프링 컨테이너에 프로토타입 빈을 요청한다.
2. 스프링 컨테이너는 프로토타입 빈을 생성해서 clientBean에 반환한다. 프로토타입의 필드 count는 0이다. 이제 프로토타입 빈은 스프링 컨테이너의 손을 떠났고 clientBean이 관리한다. (포로토타입 빈을 내부 필드에 보관한다, 참조값으로)

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FRCZpH%2FbtronJvaOpV%2FD6PCZ0yakVsx55ZzHSEBXK%2Fimg.png)

3. 클라이언트 A는 clientBean을 스프링 컨테이너에 요청하여 받는다. clientBean은 싱글톤이므로 항상 같은 clientBean이 반환된다.
4. 클라이언트는 clientBean의 logic을 호출한다.
5. clientBean은 prototypeBean의 addCount()를 호출해서 count를 증가시킨다. count는 1이 된다.
![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fl1mTK%2FbtrokqXcP2O%2FOvhG1Yg6XFqXynZbRngcQk%2Fimg.png)
6. 클라이언트 b는 clientBean을 스프링 컨테이너에 요청하여 받는다. clientBean은 싱글톤이므로 같은 빈이 반환된다.
7. clientBean이 내부에 가지고있는 프로토타입 빈은 이미 과거에 주입이 끝난 빈이다. 주입 시점에서 스프링이 컨테이너에 요청해서 프로토타입 빈이 새로 생성된 것이지 사용할 때마다 새로 생성되는 것이 아니다.
8. 클라이언트 B는 clientBean.logic()을 호출한다.
9. clientBean은 prototypeBean의 addCount를 호출해서 프로토타입 빈의 count를 증가시킨다 count는 2가된다.

여기서 발생하는 문제는 clientA, clientB는 각각 한번씩 Logic을 호출했다. 그렇기에 PrototypeBean의 카운트는 각 클라이언트마다 다른 값 1로 출력이 되어야 정상이지만 clientB의 결과값은 2가 나오게 된다.

clientBean이 가지고있는 프로토타입 스코프의 빈은 이미 생성시점에 주입이 끝난 빈이다. 주입 시점에 스프링 컨테이너에 요청해서 프로토타입 빈이 새로 생성된 것이지 **사용할 때마다 새로 생성되는 것이 아니다.**


### 어떻게 해결할 수 있을까?
바로 `ObjectFactory`, `ObjectProvider` 클래스를 사용하여 해결할 수 있다.

이 방식은 DI가 아닌 DL(Dependency Lookup) 방식을 사용하여 해결하는 것이다.

DL은 DI와 다르게 생성 시점에 내부에 주입을 받는 것이 아닌 직접 필요한 시점에 필요한 의존관계를 컨테이너에서 찾는 것을 말한다.

```java
@RequiredArgsConstructor
static class SingletonBean {
	private final ObjectProvider<PrototypeBean> objectProvider;

	public int logic() {
		PrototypeBean prototypeBean = objectProvider.getObject();
		prototypeBean.addCount();
		return prototypeBean.getCount();
	}
}
```

> ObjectFactory는 따로 ObjectFactoryBeanCreator를 configuration 파일, 클래스에 정의해줘야하는 번거러움이 있으며, getObject만 갖고있든 단순 클래스고 ObjectProvider는 ObjectFactory에서 상속, 옵션, 스트림 처리 등 편의 기능이 추가된 것이지만, 둘 다 패키지를 보면 스프링 API에 의존하고 있다.

위와 같은 문제를 해결하기 위해서는 JSR-330을 사용하는 방법이 있다.
`Provider<T>` 인터페이스가 이 문제를 해결할 수 있는데. javax.inject 패키지에 있는 인터페이스다. 이 메서드는 관련 FactoryBean 클래스를 스프링에서 만들어주기 때문에 단순하게 get()메서드 하나로 가져올 수 있어 스프링 프레임워크에 의존적이지 않다.


## 컨트롤러에서 Request 스코프 빈을 DI/DL 방식으로 주입받기

프로토타입빈하고 싱글톤타입빈하고 같이 사용했을 때 생기는 문제랑 비슷한데 한번 알아보겠다.

먼저 DL부터 볼건데 scope 타입이 웹 스코프중 request 타입의 빈을 Controller에서 사용한다고 해보자 

여기서 컨트롤러는 싱글톤 타입의 빈이다. request 스코프의 빈을 생성시점에 주입받아 사용할 수 있을까? 아니다 HTTP 요청이 들어와야 request 스코프의 빈이 생성되기 때문에 DI를 받을 수 없다.

그리고 문제 하나 더, Controller는 싱글톤이기 때문에 여러 클라이언트가 같은 컨트롤러 인스턴스를 사용하지만 request 클래스는 그렇지 않다.

### DL로 해결하기
프로토타입 빈 문제와 마찬가지로 `Provider<T>` 인터페이스를 활용하여 사용시점에 빈이 호출되도록 해줄 수 있다.

```java
@RestController
public class MyController {
	Provider<MyRequest> myRequestProvider;

	@RequestMapping("/demo")
	public String demo(HttpServeltRequest request) {
		MyRequest provider = myRequestProvider.get();
		// provider request 새로 값을 세팅하는 로직
	}
}

```

Provider 덕분의 사용 시점까지 빈의 생성을 지연할 수 있다. 사용 시점에 새로 값을 세팅하는 로직을 넣어주었기 때문에 요청마다 각각 다른 request가 생성되는 것이다.

### DI로 해결하기
어떻게 DI로 해결할 수 있을까? 생성시점이 다른데, 여기서는 **프록시**를 도입해서 해결할 수 있다.

@Scope 어노테이션에서 ScopedProxyMode.TARGET_CLASS 옵션을 줄 수 있다.

```java
@Scope(value = "request", proxyMode = ScopeProxyMode.TARGET_CLASS)
public class MyRequest {

}
```


이렇게 되면 MyRequest의 가짜 프록시 객체를 만들어두고 HTTP request와 상관없이 다른 빈에 미리 주입해둘 수 있다. 스프링 컨테이너는 CGLIB 방식을 사용하여 바이트코드 조작을 통해 MyRequest를 상속받은 가짜 프록시 객체를 생성한다. 의존관계 주입에서도 가짜 프록시 객체가 생성이 된다.

그리고 이 프록시 객체는 요청이 들어오면 이제 알맞는 request에게 위임해줌으로써 사용자 입장에서는 진짜인지 가짜인지 구분하지 못한다. 동일하게 사용할 수 있고 각 request마다 다르게 객체가 생성되므로 요청마다 새로운 request가 생긴다. (다형성)


### 총총
프록시 객체 덕분에 클라이언트는 마치 싱글톤 빈을 사용하듯 편하게 Request Scope를 사용할 수 있다.

Provider, Proxy 등 여러 방법이 있지만 핵심은 진짜 객체 조회까지(필요한 시점까지) **지연처리**를 한다는 점이다.

[[Spring 애플리케이션 로직, 인프라 빈, 컨테이너 인프라 빈]]