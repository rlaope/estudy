# Spring WebFlux vs MVC, Internal Working Principles

Spring WebFlux is a framework module for implementing Reactive Web Applications, supported since Spring 5.

### Background of Creation

Spring MVC is servlet-based, uses a Blocking I/O approach, and dedicates one thread per request.
And the thread is blocked until its task is complete.

MVC had limitations in handling high-volume request traffic.

As traffic increases, more threads are used. If a thread pool defaults to 200 threads, and ten thousand users access simultaneously, many users would pile up in the waiting queue. Consequently, thread switching costs would also increase significantly.

### Overcoming with Spring WebFlux
To handle high-volume traffic, an asynchronous/non-blocking I/O approach was necessary,
and with this approach applied, Spring WebFlux emerged, capable of stably processing large volumes.

> Misconceptions about WebFlux
> Spring WebFlux is not always fast and efficient.
> Ultimately, WebFlux is a framework born to efficiently use resources through an asynchronous non-blocking approach, and its performance efficiency can vary depending on the traffic volume and project domain requirements.
> Toby says that WebFlux is frequently used in microservices with many inter-service calls.


<br>

## MVC vs WebFlux


![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FOaGyE%2FbtsiNaY3bw1%2FRybskf9MqtE1FY6akYh7F1%2Fimg.png)

Let's assume a client sends a request to a server, and the backend server sends a request to an external server, taking 5 seconds to complete. What happens if 5 client requests come in?

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbaU14e%2FbtsiOIVkKd7%2FzOjvyGG1v3LqmsIvKIMXi1%2Fimg.png)


### MVC

MVC performs servlet-based synchronous processing.

When sending requests to an external server using RestTemplate for synchronous processing, 5 requests x 5 seconds would take a total of 25 seconds.

### WebFlux
WebFlux supports asynchronous non-blocking reactive programming.
When sending requests to an external server using WebClient, blocking does not occur for 5 requests.

In other words, the backend server does not wait for the external server's 5-second operation to complete.

Therefore, a task that took 25 seconds in MVC due to blocking can be processed much faster, in about 5-6 seconds, with WebFlux.


> Is WebFlux always better?
> Not necessarily. WebFlux processes tasks using a non-blocking asynchronous approach, thus utilizing a small number of threads. However, if synchronous logic is introduced, a critical drawback can arise where other tasks cannot be processed as they wait for threads.

<br>

## WebFlux Internal Working Principles

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FKYr6V%2FbtsiOgR7YUP%2FNu5jVoifG6TFrobhCnbNcK%2Ftfile.svg)

Because the technology stack used is very different from MVC, the reactive programming model and the MVC model are separated.

1. It uses Netty as the default server engine, not Tomcat or Jetty.
2. It supports Reactive Streams through a Reactive Streams adapter.
3. It supports WebFilter.
4. It uses a NoSQL model (Spring Data R2DBC, Non-Blocking I/O).

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FchXykt%2FbtsiPQdZwqP%2FTkBRb9XMtzORTIPtJgoeK0%2Fimg.jpg)


When a client request comes in, it goes through Netty, the server engine.

#### HttpHandler abstracts server APIs.

Various server engines are supported, not just Netty.

HttpHandler creates a ServerWebExchange. This object includes ServletHttpRequest and ServletHttpResponse.

#### WebFilter

It is a web filter composed of a filter chain.

It performs preprocessing for ServerWebExchange.

It is then passed to DispatchHandler, an implementation of WebHandler.


#### DispatchHandler

It plays a similar role to DispatcherServlet in MVC.

It receives a list of handler mappings from HandlerMapping and passes them as the source for the original Flux.

#### getHandler

It looks up the handler to process the ServerWebExchange.

#### HandlerAdapter

The HandlerAdapter invokes the handler.

The invoked handler can be a Controller or a Handler Function, and it returns a Mono<HandlerResult>.

#### Looks up HandlerResultHandler to process the returned response data.

The HandlerResultHandler processes the response data appropriately and then returns it.
