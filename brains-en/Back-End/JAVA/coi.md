# Class, Object, Instance

## What is a Class?
- A `blueprint` or template for creating objects.
- A collection of related variables and methods.

<br>

## What is an Object?
- The `target to be implemented` in the software world.
- An entity created exactly as declared in a class.

**Characteristics**
- Also referred to as an `instance of a class`.
- An object has a comprehensive meaning, representing all instances.
- From an OOP perspective, it is called an 'object' when declared as a type of class.

<br>

## What is an Instance?
- A `concrete entity implemented` in the software world based on a blueprint.
  - In other words, when an object is materialized in software, it is called an 'instance'.
  - A materialized instance is allocated in memory.

**Characteristics**
- An instance can be considered to be included within an object.
- From an OOP perspective, it is called an 'instance' when an object is allocated in memory and actually used.
- Used when focusing on the `relationship` between an abstract concept (or specification) and a concrete object.
  - Used in the form of `an instance of ~`.
  - An object is an instance of a class.
  - A link between objects is an instance of an association between classes.
  - A running process is an instance of a program.

<br>

### Example
```java
// 클래스
public class Animal{
  ...
}

// 객체와 인스턴스
public class Main{
  public static void main(String[] args){
    Animal cat, dog; // 객체
  }
  //인스턴스화
  cat = new Animal(); // cat은 Animal 클래스의 인스턴스
  dog = new Animal(); // dog은 Animal 클래스의 인스턴스
}
```

<br>

## Differences between Class, Object, and Instance

### Class vs. Object
- A class is a blueprint, and an object refers to all entities implemented from that blueprint.

### Object vs. Instance
- It is called an object when declared as a type of class, and it is called an instance when that object is allocated in memory and actually used.
- An object is closer to the real world, while an instance is closer to the software world.
- An object focuses on 'entity', while an instance focuses on 'relationship'.
  - An object is also referred to as an instance of a class.
