# Cold Publisher & Hot Publisher

## Cold Publisher

Publihser는 Subscriber가 구독할 때마다 데이터를 처음부터 새로 토잊한다.

데이터를 통지하는 새로운 타임 라인이 생성된다.

Subscriber는 구독 시점과 상관없이 통지된 데이터를 처음부터 전달 받을 수 있다.

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

Publisher는 Subscriber 수와 상관 없이 데이터를 한번만 통지한다.

즉, 데이터를 통지하는 타임라인은 하나다.

Subscriber는 발행된 데이터를 처음부터 전달 받는게 아니라 구독한 시점에 통지된 데이터들만 전달 받을 수 있다.

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