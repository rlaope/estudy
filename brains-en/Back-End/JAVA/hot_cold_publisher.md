# Cold Publisher & Hot Publisher

## Cold Publisher

A Publisher emits data anew from the beginning every time a Subscriber subscribes.

A new timeline is created for emitting data.

A Subscriber can receive the emitted data from the beginning, regardless of the subscription time.

```java
void ColdPublisher() {
    Flowable<Integer> flowable = Flowable.just(1, 3, 5, 7);

    flowable.subscribe(data -> System.out.println("구독자1: " + data));
    flowable.subscribe(data -> System.out.println("구독자2: " + data));
}
```

Result

```
구독자1: 1
구독자1: 3
구독자1: 5
구독자1: 7
구독자2: 1
구독자2: 3
구독자2: 5
구독자2: 7
```

<br>

## Hot Publisher

A Publisher emits data only once, regardless of the number of Subscribers.

In other words, there is only one timeline for emitting data.

A Subscriber does not receive the emitted data from the beginning, but only receives data emitted at the time of subscription.

```java
void HotPublisher() {
    PublishProcessor<Integer> processor = PublishProcessor.create();
    processor.subscribe(data -> System.out.println("구독자1: " + data));
    processor.onNext(1);
    processor.onNext(3);

    processor.subscribe(data -> System.out.println("구독자2: " + data));
    processor.onNext(5);
    processor.onNext(7);

    processor.onComplete();
}
```

Result
```
구독자1: 1
구독자1: 3
구독자1: 5
구독자2: 5
구독자1: 7
구독자2: 7
```
