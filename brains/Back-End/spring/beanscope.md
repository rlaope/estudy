# Bean Scope, DL(Dependency Lookup), Provider

## Bean Scope
말 그대로 빈이 존재할 수 있는 범위를 뜻한다.
  
스프링은 다음과 같은 다양한 스코프를 지원한다.

- 싱글톤 스코프: 기본 스코프, 스프링 컨테이너의 시작과 종료까지 유지되는 가장 넓은 범위의 스코프다.
- 프로토타입 스코프: 스프링 컨테이너는 프로토타입 빈의 생성과 의존관계 주입까지만 관여하고 더는 관리하지 않는 매우 짧은 범위의 스코프다.
- 웹 관련 스코프
  - request: 웹 요청이 들어오고 나갈 때 까지 유지되는 스코프
  - session: 웹 세션이 생성되고 종료될 때까지 유지되는 스코프
  - application: 웹의 서블릿 컨텍스트와 같은 범위로 유지되는 스코프

### 싱글톤 스코프

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FVD74w%2FbtrxdPtqUWQ%2FIqzmgKBIg6VnaYA7LMdB8k%2Fimg.png)

싱글톤 스코프의 빈을 조회하면 스프링 컨테이너는 항상 같은 인스턴스의 빈을 반환한다.

### 프로토타입 스코프

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbWoFOS%2FbtrxgGvstHm%2FbbaQhbnvyn0lJOEL2mdom1%2Fimg.png)

프로토타입 스코프의 빈을 조회하면 스프링 컨테이너는 항상 새로운 인스턴스를 생성해서 반환한다,
  
프로토타입의 빈을 스프링 컨테이너에 요청하면 스프링 컨테이너는 프로토타입의 빈을 생성하고, 필요한 의존관계를 주입한다.
  
싱글톤 빈은 컨테이너 생성 시점에 같이 생성되고 초기화되지만, 프로토타입 빈은 스프링 컨테이너에서 **빈을 조회할 때**생성되고 초기화 메서드도 실행된다.
  
스프링 컨테이너는 프로토타입 빈을 생성하고, 의존관계 주입, 초기화까지만 처리한다.
  
클라이언트에게 빈을 반환한 이후에는 생성된 프로토타입 빈을 관리하지 않는다.
  
프로토타입 빈을 관리할 책임은 클라이언트에게 있다. 따라서 @PreDestroy와 같은 종료 콜백 메서드가 호출되지 않는다.

### 프로토타입 빈의 문제점
만약 싱글톤 빈과 프로토타입 빈을 같이 사용하면 어떻게 될까?

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FSwAuD%2Fbtrxb2foASt%2FwydOfVZ9Z8rpV5YBBqZOr1%2Fimg.png)

위 그림처럼 clientBean이 프로토타입 빈을 포함한다고 하자. clientBean은 싱글톤이므로, 스프링 컨테이너 생성 시점에 함께 생성되고, 의존관계 주입도 발생한다.

1. clientBean은 의존관계 자동 주입을 사용한다. 주입 시점에 스프링 컨테이너에게 프로토타입 빈을 요청한다.
2. 스프링 컨테이너는 프로토타입 빈에 대한 요청을 받고, 프로토타입 빈을 생성해서 clientBean에게 반환한다. 프로토타입 빈의 count 필드 값은 0이다.

clientBean은 프로토타입 빈을 내부 필드에 보관한다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fb0kVLM%2FbtrxhMWzuvp%2FFaJbGQnfaOKswJAD5AjHKK%2Fimg.png)

클라이언트 A는 clientBean을 스프링 컨테어너에 요청해서 받는다.
  
싱글톤이므로 항상 같은 clientBean이 반환된다.

3. 클라이언트 A는 clientBean.logic()을 호출한다,
4. clientBean은 prototypeBean의 addCount()를 호출해서 프로토타입 빈의 count를 증가시킨다. count 값이 1이 된다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FXWo9K%2FbtrxfvaausD%2FvZCeZmjoF4wRBnpo1v5DC0%2Fimg.png)

클라이언트 B는 clientBean을 스프링 컨테이너에 요청해서 받는다.
  
싱글톤이므로 항상 같은 clientBean이 반환된다. 여기서 중요한 점은, clientBean이 내부에 가지고 있는 프로토타입 빈은 이미 과거에 주입이 끝난 빈이다. 주입 시점에서 스프링 컨테이너에 요청해서 프로토타입 빈이 새로 생성된 것이지, 사용할 때마다 새로 생성되는 것이 아니다.

5. 클라이언트 B는 clientBean.logic()을 호출한다.
6. clientBean은 prototypeBean의 addCount()를 호출해서 프로토타입 빈의 count를 증가시킨다. 원래 count 값이 1이었으므로 2가된다.

각 요청마다 count 값을 1로 반환받는 것을 기대했지만 2를 반환받게 된다. 결국 싱글톤과 프로토타입을 함께 사용하는 경우 기대한 대로 동작하지 않는 것이다.
  
스프링은 일반적으로 싱글톤 빈을 사용하므로, 싱글톤 빈이 프로토타입 빈을 사용하게 된다.
  
그런데 싱글톤 빈은 생성 시점에만 의존관계를 주입받는다.
  
따라서 스프링 싱글톤 빈이 생성되는 시점에 프로토 타입 빈도 새로 생성되어 주입되긴 하지만, 싱글톤 빈과 함께 계속 유지되는 것이 문제다.
  
따라서 프로토타입 빈을 주입 시점에서만 새로 생성하는 것이 아니라, 사용할 때마다 새로 생성해서 사용해야 한다.

<br>

## Dependency Lookup(DL), Provider

싱글톤 빈과 프로토타입 빈을 같이 사용할 때, 프로토타입 빈을 사용할 때마다 새로운 프로토타입 빈을 생성하려면 어떻게 해야 할까?
  
가장 간단한 방법은 싱글톤 빈이 프로토타입을 사용할 때 마다 스프링 컨테이너에 새로 요청하는 것이다.

```java
@Component
class ClientBean {
    
    //ClientBean에서 applicationContext 자체를 주입받아서 직접 요청한다.
    @Autowired
    private ApplicationContext ac;
    
    public int logic() {
        PrototypeBean prototypeBean = ac.getBean(PrototypeBean.class);
        prototypeBean.addCount();
        return prototypeBean.getCount();
    }
}

@Component
@Scope("prototype")
class PrototypeBean {
    
    private int count = 0;
    
    public void addCount() {
        count++;
    }
    
    public int getCount() {
        return count;
    }
    
    @PostConstruct
    public void init() {
        System.out.println("PrototypeBean.init" + this);
    }
    
    @PreDestroy
    public void destroy() {
        System.out.println("PrototypeBean.destroy");
    }
}
```

위 코드는 실행 시 ac.getBean()을 통해서 항상 새로운 프로토타입 빈이 생성된다.
  
이처럼 의존관계를 외부에서 주입(DI) 받는 것이 아니라 직접 필요한 의존관계를 찾는 것을 Dependency Lookup DL, 의존 관계 조회(탐색) 이라고 한다.
  
그런데 이렇게 스프링 애플리케이션 컨텍스트 전체를 주입받게 되면, 스프링 컨테이너에 종속적인 코드가 되고, 단위 테스트도 어려워진다.
  
따라서 스프링 애플리케이션 컨텍스트 전체를 주입받는 대신, ObjectProvider를 주입받아서 사용하는 것이 더 나은 방법이다.
  
ObjectProvider는 지정한 빈을 컨테이너에서 대신 찾아주는 DL(Dependency Lookup) 서비스를 제공하는 것이다.
  
(참고로 과거에는 ObjectFactory가 있었는데, 여기에 편의 기능을 추가해서 ObjectProvider가 만들어졌다)

```java
@Autowired
private ObjectProvider<PrototypeBean> prototypeBeanProvider;

public int logic() {
    PrototypeBean prototypeBean = prototypeBeanProvider.getObject();
    prototypeBean.addCount();
    int count = prototypeBean.getCount();
    return count;
}
```

실행하면 prototypeBeanProvider.getObject()를 통해서 항상 새로운 프로토타입 빈이 생성되는 것을 알 수 있다.
  
ObjectProvider의 getObject()를 호출하면 내부에서는 스프링 컨테이너를 통해 해당 빈을 찾아서 반환한다.
  
스프링이 제공하는 기능을 사용하지만, 기능이 단순하므로 단위 테스트를 만들거나 mock코드를 만들기는 훨씬 쉬워진다.
  
스프링에 의존하지만 별도 라이브러리는 필요 없다.
  
ObjectProvider는 DL 정도의 기능만 제공한다.

## 웹 스코프

웹 스코프는 웹 환경에서만 동작한다. 
  
웹 스코프는 프로토타입과 다르게 스프링이 해당 스코프의 종료 시점까지 관리한다.
  
따라서 종료 메서드가 호출된다. 웹 스코프는 다음과 같은 종류가 있다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2F129Na%2FbtrxgwsVB9N%2FKUY0UipIpOKojT3JOl0VR1%2Fimg.png)

- request: HTTP 요청이 하나 들어오고 나갈 대 까지 유지되는 스코프, 각각의 HTTP 요청마다 별도의 빈 인스턴스가 생성되고 관리된다.
- response: HTTP Session과 동일한 생명 주기를 가지는 스코프
- application: 서블릿 컨텍스트와 동일한 생명주기를 가지는 스코프
- websocket: 웹 소켓과 동일한 생명주기를 가지는 스코프

스프링 빈 등록 시 웹 스코프를 그대로 주입받으면 오류가 발생한다.
  
싱글톤 빈은 스프링 컨테이너 생성 시 함께 생성되어서 라이프 사이클을 같이하지만, 웹 스코프(여기서는 request 스코프)의 경우 HTTP 요청이 올 때 새로 생성되고 응답하면 사라지기 때문에, 싱글톤 빈이 생성되는 시점에는 아직 생성되지 않았다. 따라서 의존관계 주입이 불가능하다.
  
이때 프록시를 사용하면 문제를 해결할 수 있다.

```java
@Component
@Scope(value = "request", proxyMode = ScopedProxyMode.TARGET_ClASS)
public class MyLogger {
}
```

@Scope 속성에 `proxyMode = ScopedProxyMode.TARGET_CLASS`를 추가해준다.
- 적용대상이 인터페이스가 아닌 클래스면 TARGET_CLASS를 선택
- 적용대상이 인터페이스면 INTERFACES를 선택

이렇게 하면 MyLogger의 가짜 프록시 클래스를 만들어두고 HTTP request와 상관없이 가짜 프록시 클래스를 다른 빈에 미리 주입해둘 수 있다.
  
Scope의 proxyMode = ScopedProxyMode.TARGET_CLASS를 설정하면 스프링 컨테이너는 CGLIB라는 바이트코드 조작 라이브러리를 사용해서, 해당 웹 스코프 빈 클래스를 상속하는 가짜 프록시 객체를 생성한다.
  
위의 결과를 보면 내가 등록한 순수 자바 클래스 빈이 아니라 가짜 프록시 객체가 등록된 것을 확인할 수 있다.
  
그리고 스프링 컨테이너에 원래 만들었던 진짜 클래스 이름으로 (첫 문자 소문자, 빈 이름 규칙)(여기서는 myLogger) 진짜 대신에 이 가짜 프록시 객체를 등록한다.
  
ag.getBean("myLogger", MyLogger.class)로 조회해도 프록시 객체가 조회된다.
  
그래서 의존관계 주입도 이 가짜 프록시 객체가 주입된다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fsqofr%2FbtrxezqufXQ%2Fj7ZCbeg6dOh50PQsLbEYc1%2Fimg.png)

- 가짜 프록시 객체는 요청이 오면 그때 내부에는 진짜 빈을 요청하는 위임 로직이 들어있다.
- 가짜 프록시 객체는 내부에 진짜 myLogger를 찾는 방법을 알고 있다.
- 클라이언트가 myLogger.logic()을 호출하면 사실은 가짜 프록시 객체의 메서드를 호출한 것이다.
- 가짜 프록시 객체는 request 스코프의 진짜 myLogger.logic()을 호출한다.
- 가짜 프록시 객체는 원본 클래스를 상속받아서 만들어졌기 때문에 이 객체를 사용하는 클라이언트 입장에서는 원본인지 아닌지도 모르게 동일하게 사용할 수 있다(다형성)