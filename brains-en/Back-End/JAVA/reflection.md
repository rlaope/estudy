# Java Reflection

## Reflection
The JVM **reads class information via the class loader and stores that information in JVM memory.**

The information about the stored class resembles a reflection in a mirror, hence the name Reflection.

Using Reflection, you can obtain very detailed information about a class, such as its **constructors, methods, and fields**.

A prime example of Reflection's use is annotations, which are utilized in various libraries and frameworks.

As just mentioned, Reflection allows you to check which annotations are attached to classes and methods.

Annotations themselves do nothing.
Thanks to Reflection, we can use annotations like `@Component` and `Bean` in Spring to leverage framework functionalities.

## The Class Class
The core of Reflection is the Class. The Class is provided in the **java.lang** package. How can we obtain a `Class` instance for a specific class?

### Ways to Obtain a Class Object

```java
Class<Member> aClass = Member.class; // (1)

Member member1 = new Member();
Class<? extends Member> bClass = member1.getClass(); // (2)

Class<?> cClass = Class.forName("hudi.reflection.Member"); // (3)
```

1. The first method is to obtain it via the class property of the class.
2. The second method is to use the `getClass()` method of an instance.
3. The third method is to pass the FQCN (Fully Qualified Class Name) to the static `forName()` method of the Class class to obtain a Class class instance corresponding to that path and class.

Since the Class class does not have a public constructor, there is no way for us to directly create an instance of it.
Instead, Class objects are automatically created by the JVM.

### getXXX() vs getDeclaredXXX()

Among the Class object methods, forms like `getFields()`, `getMethods()`, `getAnnotations()`
can be seen defined as `getDeclaredFields()`, `getDeclaredMethod()`, `getDeclaredAnnotations()`.

These methods are used to retrieve the list of fields, methods, and annotations defined in a class.

For convenience, I'll refer to these two forms as getXXX() and getDeclaredXXX(), respectively. What's the difference between them?

- getXXX(): Retrieves all public elements, including inherited classes and interfaces.
  - For example, `getMethods()` retrieves all public methods for the class, including those inherited and those from implemented interfaces.
- getDeclaredXXX(): Retrieves only the elements directly defined in the class, excluding inherited interfaces. It can also access elements regardless of their access modifier.
  - For example, `getDeclaredMethods()` retrieves all private, protected, and public methods directly defined in that class.

## Constructor
Using Class, you can obtain constructors as `Constructor` types. `Constructor` is a class provided in the `java.lang.reflect` package that provides information about and access to class constructors. Let's directly access constructors and create objects using Reflection.

```java
Constructor<?> constructor = aClass.getDeclaredConstructor();
// 생성자 가져오기

Object object = constructor.newInstance();
// 이렇게 인스턴스를 생성할 수 있다.

Member member = (Member) constructor.newInstance();
// 타입 캐스팅을 사용해서 위와 같이 받아올 수 있다.
```

As shown above, you can obtain a Constructor using the `getConstructor()` method of a Class type object.
Its `newInstance()` method can be used to directly create objects.
If you don't use type casting, it will be received as an Object type, so let's use type casting.

The example above shows how to get a default constructor. What if the constructor has parameters? You can pass the types corresponding to the constructor parameters to the `getConstructor()` method.

```java
Constructor<?> noArgsConstructor = aClass.getDeclaredConstructor();
Constructor<?> onlyNameConstructor = aClass.getDeclaredConstructor(String.class);
Constructor<?> allArgsConstructor = aClass.getDeclaredConstructor(String.class, int.class);
```

Once you've obtained a constructor with parameters, you can create an object just like using a regular constructor, as shown below.
```java
Member member = (Member) allArgsConstructor.newInstance("희망", 18);
```

However, if you create an object with a private constructor using the method above, a `Java.lang.IllegalAccessException` will occur. This can be resolved by using `setAccessible(true)` as shown below.

```java
noArgsConstructor.setAccessible(true);
Member member = (Member) noArgsConstructor.newInstance();
```
> Note that the `newInstance()` method of the Class type is deprecated, so avoid using it.

### Field
Using Reflection, you can obtain a `Field` type object to directly access object fields.

```java
Class<Member> aClass = Member.class;
Member member = new Member("희망", 18);

for (Field field : aClass.getDeclaredFields()) {
    field.setAccessible(true);
    String fieldInfo = field.getType() + ", " + field.getName() + " = " + field.get(member);
    System.out.println(fieldInfo);
}

/*
    class java.lang.String, name = 희망
    int, age = 18
*/
```

As shown below, you can also forcibly change an object's field value even without a Setter, by using the `set()` method.
```java
Class<Member> aClass = Member.class;
Member member = new Member("희망", 18);

Field name = aClass.getDeclaredField("name");
name.setAccessible(true);
name.set(member, "hope"); // 필드값 변경

System.out.println("member = " + member);
// member = Member{name='hope', age=18}
```

### Method
Using Reflection, you can obtain a `Method` type object to access object methods.
```java
Class<Member> aClass = Member.class;
Member member = new Member("hope", 18);

Method sayMyName = aClass.getDeclaredMethod("sayMyName");
sayMyName.invoke(member);
// My name is hope
```

As in the example above, you can directly invoke a method using the `invoke()` method of the Method type.

## Retrieving Annotations
```java
Class<Member> aClass = Member.class;

Entitiy entityAnnotation = aClass.getAnnotation(Entitiy.class);
String value = entityAnnotation.value();
System.out.println("value = " + value);
// Member
```

As shown above, by directly passing the annotation type to the `getAnnotation()` method, you can retrieve the annotation attached to the class.
You can also see that it's possible to access the fields that the annotation possesses.

## Disadvantages of Reflection
Generally, when a method is called, it uses classes analyzed at compile time, but Reflection **analyzes classes at runtime, making it slower.**

This is said to be because the JVM cannot optimize it. And due to this characteristic, **type checking is not possible at compile time**.

There is also the disadvantage that it breaks object encapsulation.

Therefore, general web application developers rarely need to use Reflection; it's typically used when developing libraries or frameworks.
Thus, Reflection **should only be used sparingly where it is truly necessary.**
