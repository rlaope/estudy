# Spring WebFlux vs MVC, 내부 작동 방식

Spring WebFlux는 Spring 5. 부터 지원하는 Reactive Web Application을 구현하기 위한 프레임워크 모듈이다.

### 탄생 배경

Spring MVC는 서블릿 기반에 Blocking I/O 방식이고 요청당 하나의 스레드를 사용하게 되었다. 
그리고 스레드의 작업이 끝날 때까지 스레드가 차단된다.

대용량 요청 트래픽을 MVC가 처리하기에는 한계가 있었다.

트래픽이 많아지면 그 만큼 스레드도 많이 사용이 된다. 스레드풀에 스레드 200개가 default로 존재하고, 만약 만명이 동시에 접근하게된다면 대기 큐에 쌓이는 유저들이 정말 많을 것이다. 그리고 스레드 스위칭 비용도 그만큼 많이 들게 된다.

### Spring WebFlux를 이용한 극복
대용량 트래픽을 감당하기 위해선, 비동기/Non-Blocking 방식의 I/O를 사용해야했으며
이 방식이 적용되어, 대용량도 안정적으로 처리할 수 있는 Spring WebFlux가 생겨났다

> WebFlux 오해
> Spring WebFlux라고 항상 빠르고 효율적인 것은 아니다.  
> 결국 WebFlux는 비동기 넌블러킹 방식으로 자원을 효율적으로 사용하기 위해 탄생한 프레임워크이고 트래픽의 수, 프로젝트 도메인의 요구사항에 따라 성능 효율이 달라질 수 있다.
> 토비님은 서비스간의 호출이 많은 마이크로서비스에서 웹플럭스가 많이 사용된다고 한다. 


<br>

## MVC vs WebFlux


![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FOaGyE%2FbtsiNaY3bw1%2FRybskf9MqtE1FY6akYh7F1%2Fimg.png)

클라이언트가 서버로 요청을 보내고 백엔드 서버는 외부 서버에 요청을 보내게 되는데 이때 동작 시간이 5초가 걸린다고 가정을 해보았을때, 클라이언트의 요청이 5번 들어오면 어떻게 될까

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbaU14e%2FbtsiOIVkKd7%2FzOjvyGG1v3LqmsIvKIMXi1%2Fimg.png)


### MVC

MVC는 서블릿 기반 동기식 처리를 한다.

외부 서버로 요청을 보낼 때 동기 처리를 하는 RestTemplate를 사용한다 했을 때 5번의 요청 x 5초 해서 총 25초라는 시간이 걸릴 것이다.

### WebFlux
웹플럭스는 비동기 넌블러킹 방식의 리액티브 프로그래밍을 지원한다.
이때 외부 서버로 요청을 보내면 WebClient를 사용했다 했을때 5번의 요청에서 블로킹이 발생하지 않는다.

즉 외부 서버에서 5초가 걸리는 것을 백엔드 서버에서 기다리지 않는 것이다.

그렇기에 블로킹이 발생했던 MVC에서 25초 걸리던 작업이 WebFlux에서는 5~6초정도 빠른 처리를 할 수 있다.


> 항상 WebFlux가 좋나요?
> 그건 아니다. WebFlux는 넌블러킹 비동기 방식으로 작업을 처리하기때문에 적은 수의 스레드로 작업을 처리한다. 그러나 만약 여기서 동기 로직이 추가가 되버린다면 다른 작업들은 스레드를 대기하면서 여러 작업들을 처리하지 못하는 치명적인 단점이 생길 수 있다.

<br>

## WebFlux 내부 동작 원리

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FKYr6V%2FbtsiOgR7YUP%2FNu5jVoifG6TFrobhCnbNcK%2Ftfile.svg)

MVC와 사용 스택이 매우 다르기때문에 리액티브 프로그래밍 모델과 MVC 모델은 나누어져있다.

1. tomcat, jetty가 아닌 netty라는 기본 서버 엔진을 사용한다.
2. 리액티브 스트림즈 어댑터를 통한 리액티브 스트림즈를 지원한다.
3. WebFilter를 지원한다.
4. NoSQL 모델을 사용한다(Spring Data R2DBC, Non-Blocking I/O)

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FchXykt%2FbtsiPQdZwqP%2FTkBRb9XMtzORTIPtJgoeK0%2Fimg.jpg)


클라이언트의 요청이 들어오게 된다면 서버엔진인 Netty를 거치게된다.

#### HttpHandler가 서버 API를 추상화한다.

네티 뿐만 아니라 다양한 서버엔진이 지원이 된다.

HttpHandler는 ServerWebExchange를 생성한다. 이 친구한테 ServletHttpRequest, ServletHttpResponse가 포함되어있다.

#### WebFilter

필터 체인으로 구성된 웹 필터이다.

ServerWebExchange의 전처리 과정을 실천한다.

이후 WebHandler의 구현체인 DispatchHandler에게 전달이된다.


#### DispatchHandler

MVC에 DispatcherServlet하고 유사한 역할을 한다.

HandlerMapping에게서 핸들러매핑 리스트를 받아 원본 Flux의 소스로 전달받는다.

#### getHandler

ServerWebExchanger을 처리할 핸들러를 조회한다.

#### HandlerAdapter

핸들러 어댑터는 핸들러를 호출한다.

이때 호출되는 핸들러의 형태는 Controller, Handler Function 형태이며, Mono\<HandlerResult>를 반환한다.

#### 반환받은 응답데이터를 처리할 HandlerResultHandler를 조회한다.

HandlerReulstHandler는 응답 데이터를 적절히 처리한 후 return한다.