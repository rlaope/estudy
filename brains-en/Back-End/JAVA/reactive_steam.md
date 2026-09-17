# Reactive Streams

## Reactive Streams

A standard for asynchronous data processing using non-blocking backpressure.

### API

```java
public interface Publisher<T> {
    public void subscribe(Subscriber<? super T> s);
}

public interface Subscriber<T> {
    public void onSubscribe(Subscription s);
    public void onNext(T t);
    public void onError(Throwable t);
    public void onComplete();
}

public interface Subscription {
    public void request(long n);
    public void cancel();
}
```

It consists of a simpler set of APIs than one might expect.

<br>

## How Reactive Streams Work

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FtYKV4%2FbtqOuGMA5wf%2FACu9FA1bEY2477O6bawaH0%2Fimg.png)

The important thing to note is that the Publisher does not push data to the Subscriber; instead, it's a **pull-based approach where the Subscriber requests data from the Publisher**, and this method is called backpressure.

### Backpressure
When a Publisher publishes and a Subscriber subscribes, instead of the Publisher pushing data to the Subscriber, the Subscriber requests data from the Publisher in a pull-based manner, asking for an amount it can handle, thereby preventing failures.

In other words, it's a method where the Subscriber requests data through a dynamic pull mechanism, asking for as much as it can accommodate.

<br>

## Implementation

```java
public class ReactiveStreamTest {

  public static class PublisherImpl implements Publisher<Integer> {
    @Override
    public void subscribe(Subscriber<? super Integer> subscriber) {

      Queue<Integer> queue = new LinkedList<>();
      IntStream.range(0, 10).forEach(queue::add);

      subscriber.onSubscribe(new Subscription() {
        @Override
        public void request(long n) {
          System.out.println("request:" + n);

          for (int i=0; i<=n; i++){
            if(queue.isEmpty()) {
              subscriber.onComplete();
              return;
            }
            subscriber.onNext(queue.poll());
          }
        }

        @Override
        public void cancel() {
          System.out.println("publish cancel");
        }
      });
    }
  }

  public static class SubscriberImpl implements Subscriber<Integer> {

    private Subscription subscription;
    private long requestSize = 2;
    private List<Integer> buffer = new ArrayList<>();

    @Override
    public void onSubscribe(Subscription s) {
      subscription = s;
      subscription.request(requestSize);
    }

    @Override
    public void onNext(Integer integer) {
      System.out.println("  onNext - " + integer);
      buffer.add(integer);
      if(buffer.size() == requestSize) {
        buffer.clear(); //flush
        subscription.request(requestSize);
      }
    }

    @Override
    public void onError(Throwable t) {
      System.out.println("error:" + t.getMessage());
    }

    @Override
    public void onComplete() {
      System.out.println("subscribe complete");
    }
  }

  public static void main(String[] args) {

    Publisher<Integer> publisher = new PublisherImpl();
    publisher.subscribe(new SubscriberImpl());

  }
}
```

Result
```
request:2
  onNext - 0
  onNext - 1
request:2
  onNext - 2
  onNext - 3
request:2
  onNext - 4
  onNext - 5
request:2
  onNext - 6
  onNext - 7
request:2
  onNext - 8
  onNext - 9
request:2
subscribe complete
```

We limit the request size to 2, and if the data's request size and bufferSize are equal, we clear the buffer and request again from the publisher via the subscription.

> It's convenient to think of a subscription as a type of remote control.
