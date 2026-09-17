#### Overview

While looking for Java interview questions, I came across a topic asking to explain the difference between StringBuilder and StringBuffer. Therefore, I'd like to organize my thoughts on this topic.

![](image/stringbuilder.png)

#### Java's String Classes

In Java, there are three string classes: String, StringBuffer, and StringBuilder. These three classes each have their own differences.

#### String vs StringBuilder, StringBuffer

In Java, once a value is assigned to a String object, its allocated space does not change. However, StringBuilder and StringBuffer objects have the characteristic that even if a value is assigned once, and then another value is assigned, the allocated space changes.

Here, the characteristic where the allocated space does not change is called **immutability**, and the characteristic where the allocated space changes is called **mutability**.

**String**

**\-> Possesses immutability.**

**StringBuilder, StringBuffer**

**\-> Possesses mutability.**

When declaring strings using String, StringBuilder, and StringBuffer classes, changing their values, and then printing their addresses using `hashCode`, only the address of the String object changes.

![](./image/stringcode.png)

![](./image/stringrun.png)

#### String Constant Pool

There are two ways to assign values to String variables:

1. Assigning a literal value

2. Using the `new` keyword

If only the values are the same when assigning with a literal versus assigning with the `new` keyword, will the result of comparing their addresses (==) truly be the same?

No. The reason is that the storage method differs depending on how String values are assigned.

**Assigning with a literal value**

When assigning a String with a literal value, it is stored in a special memory space within the Heap memory area called the String constant pool.

If a literal value already exists in the String constant pool, instead of creating a new literal value and storing it, the **existing value** is used.

Therefore, if two variables assigned with literal values have the same value, printing with the `==` operator will result in `true`.

**When assigning values with the `new` keyword**

When assigning a value to a String variable using the `new` keyword, memory space is **dynamically allocated in the Heap area**, just like with regular objects.

For the reason mentioned above, printing with the `==` operator will result in `false`. This is because they refer to different memory spaces.

#### StringBuilder vs StringBuffer

Unlike String, we learned that both StringBuilder and StringBuffer possess mutability.

![](image/stringin1.png)

![](image/stringin2.png)

As shown above, both classes are implemented by inheriting from the abstract class AbstractStringBuilder.

The AbstractStringBuilder abstract class has the following two member variables:

- value: A byte array that stores string values

- count: An int variable that holds the current string size

To modify strings in StringBuilder and StringBuffer classes, the `append()` method is used.

The `append` method is implemented within AbstractStringBuilder, and its internal structure is as follows:

![](image/stringin3.png)

When a string is appended as shown, it is implemented by increasing the space currently storing the string by the size of the string to be added, and then inserting the string to be added into that expanded space.

Through the internal operation examined above, even if the value changes, it **references the same address space**, exhibiting **mutability** where the value changes.

StringBuilder and StringBuffer both share similar characteristics, but they have a difference: the difference in **synchronization**.

StringBuilder does not support synchronization, while StringBuffer does.

Because it supports synchronization, it can operate safely even in a multi-threaded environment.

The reason is that StringBuffer's methods use the `synchronized` keyword.

_In Java, the `synchronized` keyword prevents **other threads from accessing data while one thread is currently using it** when multiple threads try to access a single resource._

For example, if both Thread A and Thread B use the `append` method of the same StringBuffer class object, the following procedure occurs:

1. Thread A: Accesses and executes the `append` synchronized block of the StringBuffer object.

2. Thread B: Cannot enter the object's `append` synchronized block and enters a blocked state.

3. Thread A: Exits the `sb.append()` synchronized block.

4. Thread B: Changes from blocked to running state, then accesses and executes the object's `append` synchronized block.

_You can find a note in the StringBuilder class's Javadoc recommending StringBuffer if synchronization is needed._

#### String Classes to Use Depending on the Situation

**String**

**\->** String possesses immutability. This means that using the String type is better for performance when working with immutable strings.

**StringBuilder**

**\->** While it does not support synchronization, it offers better performance than StringBuffer in terms of speed.

\-> Therefore, it is advantageous for performance to use it in **single-threaded environments where string additions, modifications, or deletions** occur frequently.

**StringBuffer**

**\->** It supports synchronization and can operate safely even in multi-threaded environments.

\-> Therefore, it is advantageous for performance to use it in **multi-threaded environments where string additions, modifications, or deletions occur frequently**.

References

[https://velog.io/@heoseungyeon/StringBuilder%EC%99%80-StringBuffer%EB%8A%94-%EB%AC%B4%EC%8A%A8-%EC%B0%A8%EC%9D%B4%EA%B0%80-%EC%9E%88%EB%8A%94%EA%B0%80](https://velog.io/@heoseungyeon/StringBuilder%EC%99%80-StringBuffer%EB%8A%94-%EB%AC%B4%EC%8A%A8-%EC%B0%A8%EC%9D%B4%EA%B0%80-%EC%9E%88%EB%8A%94%EA%B0%80)

 [What is the difference between StringBuilder and StringBuffer?

In Java, the String class has immutability. Therefore, good performance can be expected when frequently using immutable strings. However, in programs where string modifications occur frequently, Stri

velog.io](https://velog.io/@heoseungyeon/StringBuilder%EC%99%80-StringBuffer%EB%8A%94-%EB%AC%B4%EC%8A%A8-%EC%B0%A8%EC%9D%B4%EA%B0%80-%EC%9E%88%EB%8A%94%EA%B0%80)

[String (Java SE) - Oracle Docs](https://docs.oracle.com/en/java/javase/17/docs/api/java.base/java/lang/String.html)
