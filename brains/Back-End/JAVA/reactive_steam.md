# Reactive Streams 

## Reactive Streams

논블로킹 백프레셔를 이용한 비동기 데이터 처리의 표준.

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

생각보다 간단한 조합의 API들로 이루어져있다.

<br>

## Reactive Stream 동작 방식

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FtYKV4%2FbtqOuGMA5wf%2FACu9FA1bEY2477O6bawaH0%2Fimg.png)

중요하게 봐야하는 점은 Publisher가 Subscriber에게 데이터를 Push하는 것이 아니라,  
**Subscriber가 Publisher에게 데이터를 요청하는 Pull 방식**이라는 점이며, 이런 방식을 백프레셔라고 한다.

### 백프레셔
Publisher에서 발행하고, Subscriber에서 구독할 때, Publisher에서 데이터를 Subscriber로 Push하는 방식이 아니라, Pull 방식으로 Subscriber가 Publisher로 처리할 수 있는 양의 크기 만큼 데이터를 요청함으로써 장애를 방지하기 위함이다.

즉, 다이나믹 풀 방식의 데이터 요청을 통해서 Subscriber가 수용할 수 있는 만큼 데이터를 요청하는 방식이다.

<br>

## 구현

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

request size를 2로 제한하고 데이터를 request size와 bufferSize가 같다면 buffer를 clear하고 다시 publisher에게 subscription을 통해 요청을 한다.

> subscription은 일종의 리모컨이라고 생각하면 편하다.