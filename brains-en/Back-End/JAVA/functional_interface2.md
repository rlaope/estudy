# Built-in Functional Interfaces in Java

It is very cumbersome to create and use functional interfaces every time.

Therefore, Java provides commonly used functional interfaces by default.

In fact, you can create most lambda expressions using only the built-in ones, so developers rarely create functional interfaces themselves.

![](https://velog.velcdn.com/images/sunil1369/post/dd5957da-a47c-4697-a46e-e6fa78341e72/image.png)

## Predicate

```java
@FunctionalInterface
public interface Predicate<T> {
    boolean test(T t);
}
```

A Predicate takes one argument and returns a boolean type.

In lambda expressions, it is represented as T -> boolean.

<br>

## Consumer

```java
@FunctionalInterface
public interface Consumer<T> {
    void accept(T t);
}
```

A Consumer takes one argument and returns nothing.

In lambda expressions, it is represented as T -> void.

As its name 'Consumer' suggests, you can think of it as taking an argument, consuming it, and then finishing.

<br>

## Supplier

A Supplier takes no arguments and returns an object of type T.

In lambda expressions, it is represented as () -> T.

Like its name 'Supplier', it takes nothing and returns a specific object.

<br>

## Function

```java
@FunctionalInterface
public interface Function<T, R> {
    R apply(T t);
}
```

A Function takes an argument of type T and returns a type R.

In lambda expressions, it is represented as T -> R.

Like a function in mathematics, it takes a specific value and transforms it into another value.

T and R can use the same type.

<br>

## Comparator

A Comparator takes two arguments of type T and returns an int type.

In lambda expressions, it is represented as (T, T) -> int.

<br>

## Runnable

```java
@FunctionalInterface
public interface Runnable {
    public abstract void run();
}
```

A Runnable takes no arguments and returns nothing.

In lambda expressions, it is represented as () -> void.

As its name 'Runnable' suggests, meaning 'executable', you can think of it as simply being able to execute.

<br>

## Callable

```java
@FunctionalInterface
public interface Callable<V> {
    V call() throws Exception;
}
```

A Callable takes no arguments and returns an object of type T.

In lambda expressions, it is represented as () -> T.

Similar to Runnable, thinking of Callable as 'callable' might make more sense.

<br>

### Suplier vs Callable
In fact, they are completely identical.

They both take no arguments and return a specific type.

Difference? You can actually think of there being no difference.

However, Callable emerged as a concept for parallel processing alongside Runnable, and functions like `ExecutorService.submit()` take a Callable as an argument.
