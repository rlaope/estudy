# Java Generic

Generics enhance code safety by performing type checks at compile time.

```java
List<T> // Type parameter
List<String> stringList = new ArrayList<>(); // Parameterized type
```

A list declared as `List<String>` can only contain data of type String. By using generics, the type of data entering the list can be checked at compile time, thereby increasing safety.

Furthermore, using generics also provides type casting. Therefore, if you retrieve data from a `List<String>`, you will get a String type without needing to explicitly cast it.

### Variance

Is this statement possible: `List<Object> objectList = new ArrayList<Integer>();`?

The answer is no, it's not possible. While arrays allow `Object[] objArray = new Integer[1];`, there's a difference between generic types and regular types. This concept is called variance; arrays are covariant, while generic types are invariant.

**Variance describes the relationship between different types.**

- **Invariant (\<T>)**: If type B is not a subtype of type A, then `Category<B>` has no relationship with `Category<A>`.
- **Covariant (\<? extends T>)**: If type B is a subtype of type A, then `Category<B>` is a subtype of `Category<A>`.
- **Contravariant (\<? super T>)**: If type B is a subtype of type A, then `Category<B>` is a supertype of `Category<A>`.

### Generic Methods

```java
public <T> void printClassName(T t) {
  System.out.println(this.t.getClass().getName());
}
```

This way, you define the type to be passed to the function argument, allowing various types of classes to be flexibly accepted.

```java
class Category<T extends Noodle> {
  private T t;
}
```

In a class like the one above, you can set bounded generic types. With this setting, when creating a `Category`, only `Noodle` or its subtypes can be used.

- `<?>`: Any type can be used.
- `<? extends Noodle>`: `Noodle` and its subtypes can be used.
- `<? super Noodle`: `Noodle` and its supertypes can be used.

When `Noodle` and its subtypes can be used (covariant), retrieving elements is possible, but storing them is not.

When `Noodle` and its supertypes can be used (contravariant), storing elements is possible, but retrieving them is not.

Therefore, in Effective Java, `<? extends Noodle>` is used for producers, and `<? super Noodle>` is used for consumers.

### Generic Type Erasure

If a type parameter has no bounds, it is replaced with `Object`; if it has bounds, it is replaced with its bound type.

For `class Category`, `Object` is the bound type, but for `class Category<T extends Noodle>`, `Noodle` becomes the bound type. Type casts are added where necessary to maintain type safety.

Bridge methods are generated to maintain polymorphism in classes that inherit generic types, by erasing method parameters or return types.
