# Reactive Stream, Backpressure, API Components

Reactive Stream is a fundamental specification for building asynchronous services using non-blocking and backpressure. Both Java's RxJava and the Project Reactor project, which is at the core of Spring5 WebFlux, adhere to this specification.

Flow, added in Java 9, also adopts and uses the Reactive Stream specification.

Asynchronous systems are effective for processing continuously incoming stream data.

The most crucial issue when performing asynchronous processing is that data processing must be carefully controlled within a predictable range of destination resource consumption.

The main purpose of Reactive Stream is to effectively manage the exchange of stream data by clearly defining asynchronous boundaries. In other words, it makes it possible to predict how much data will enter a system that processes data asynchronously. **Backpressure** is a crucial part that enables this to be achieved.

1. Processing a potentially infinite number of data items
2. Processing sequentially
3. Asynchronously transferring data between components
4. Controlling data flow using backpressure

### Backpressure
I've been mentioning backpressure above, so let's take a look at what it is.

First, the glossary of the Reactive Manifesto describes it as follows:

> When one component struggles to cope with load, the system as a whole must respond in a reasonable way. Critical failures must not occur in overloaded components, nor should messages be lost without control. **Since a component cannot cope and must not fail, it must inform upstream components that it is overloaded, so they can reduce the load.** This backpressure is an important feedback mechanism that allows the system to respond normally without collapsing under load. While backpressure can propagate to the user, leading to reduced responsiveness, this mechanism ensures the system's resilience to load and provides information on whether the system itself can offer other resources to distribute the load.

Simply put, when a component is overloaded, it informs upstream components of its state to reduce the load.

<br>

## API Components

The components of the Reactive Stream API are as follows:

1. Publisher
2. Subscriber
3. Subscription
4. Processor

A Publisher provides an infinite stream of data, and the provided data is consumed by a Subscriber.

Subscribers establish a connection in the format of `Publisher.subscribe(Subscriber)`.

The execution order is as follows: `onSubscribe onNext* (onError | onComplete)?`

`onSubscribe` means that the Subscriber is always ready to receive signals from the data produced by the Publisher, and it receives data via `onNext`.

If there is a failure, an `onError()` signal is called; if there are no more signals available, an `onComplete()` signal is called.

This continues until the Subscription is canceled.

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

The classes are specified as above.

Based on the interfaces above, the following flow is constructed.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FzmVzg%2FbtqFkUlWOkd%2FiKCzp1MgvKthItuXLUCYO1%2Fimg.png)

1. The Publisher implements the Subscription it owns and creates the data to be published.
2. The Publisher registers a Subscriber via the `subscribe()` method.
3. The Subscriber registers a Subscription via the `onSubscribe()` method and begins subscribing to the Publisher. This occurs through the Subscription implemented in the Publisher, establishing a connection between the Publisher and Subscriber via the Subscription. When `Subscription.request()` is called within `onSubscribe()`, data subscription begins.
4. The Subscriber controls the data flow by calling `Subscription.request()` or `cancel()`.
5. Depending on conditions, `Subscription.request()` calls the Subscriber's `onNext()`, `onComplete()`, or `onError()`. The Subscriber then controls the flow with `request()` or `cancel()` according to the logic in its respective method.
