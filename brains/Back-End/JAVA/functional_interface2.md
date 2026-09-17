# Java에서 기본적으로 제공하는 Functional Interface

함수형 인터페이스를 매번 만들어서 사용하는 것은 매우 번거러운 일이다.

그래서 자바에서는 기본적으로 많이 사용이되는 함수형 인터페이스들을 제공한다.

사실상 기본적으로 제공되는 것만 사용해도 웬만한 람다식은 다 만들 수 있어서 개발자가 직접 함수형 인터페이스를 만드는 경우는 거의 없긴하다.

![](https://velog.velcdn.com/images/sunil1369/post/dd5957da-a47c-4697-a46e-e6fa78341e72/image.png)

## Predicate

```java
@FunctionalInterface
public interface Predicate<T> {
    boolean test(T t);
}
```

Predicate는 하나의 인자를 받고 boolean 타입을 리턴한다.

람다식으로는 T -> boolean으로 표현한다.

<br>

## Consumer

```java
@FunctionalInterface
public interface Consumer<T> {
    void accept(T t);
}
```

Consumer는 인자 하나를 받고 아무것도 리턴하지 않는다.

람다식으로는 T -> void로 표현합니다.

소비자라는 이름에 걸맞게 인자를 받아서 소비만하고 끝낸다고 생각하면 됩니다.

<br>

## Supplier

Supplier는 아무런 인자를 받지 않고 T타입의 객체를 리턴합니다.

람다식으로는 () -> T 로 표현합니다.

공급자라는 이름처럼 아무것도 받지 않고 특정 객체를 리턴합니다.

<br>

## Function

```java
@FunctionalInterface
public interface Function<T, R> {
    R apply(T t);
}
```

Function은 T타입의 인자를 받아서 R 타입을 리턴합니다.

람다식으로는 T -> R로 표현합니다.

수학식에서 함수처럼 특정 값을 받아 다른 값으로 반환해줍니다.

T, R은 같은 타입을 사용할 수 있습니다.

<br>

## Comparator

Comparator는 T타입 인자를 두개 받아 int타입을 리턴합니다.

람다식으로는 (T, T) -> int 로 표현합니다.

<br>

## Runnable

```java
@FunctionalInterface
public interface Runnable {
    public abstract void run();
}
```

Runnable은 아무런 객체도 받지 않고 아무런 객체도 리턴하지 않습니다.

람다식으로는 () -> void 로 표현합니다.

Runnalbe이라는 이름에 맞게 실행 가능한 이라는 뜻을 나타내며 이름 그대로 실행만 할 수 있다고 생각하면 됩니다.

<br>

## Callable

```java
@FunctionalInterface
public interface Callable<V> {
    V call() throws Exception;
}
```


Callable은 아무런 인자를 받지 않고 T 타입 객체를 리턴합니다.

람다식으로는 () -> T로 표현합니다.

Runnable과 비슷하게 Callalbe은 호출 가능한이라고 생각하면 좀 더 와닿습니다.

<br>

### Suplier vs Callable
사실 둘은 완전히 동일합니다. 

아무런 인자도 받지 않고 특정 타입을 리턴해줍니다.

차이? 사실 차이가 없다고 생각하면 됩니다.

단지 Callable은 Runnable과 함께 병렬 처리를 위해 등장했던 개념으로서 `ExecutorService.submit()`같은 함수는 인자로 Callable을 받습니다.