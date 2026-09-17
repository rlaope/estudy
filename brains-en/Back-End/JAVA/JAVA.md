# Java

## What is Java?
Java is an `object-oriented programming language` released by Sun Microsystems in 1995.
Java was designed to have as few dependencies as possible, and it is a general-purpose language that allows code written once to run on any platform, embodying the principle of `Programmers write once, run anywhere (WORA)`.

It is the language of choice for many Back-End developers worldwide, with 9 million reported developers globally. It is also the sole official language for Android app development.

Java is used in many services like Amazon, Twitter, and Netflix, and can run in various environments such as game consoles and supercomputers.

The South Korean e-Government Standard Framework is Spring, a Java framework.

<br>

## Code Example
```java
class HelloJava{
  public static void main(String[] args){
    Sysem.out.println("hello java");
  }
}
```
- Class Name: The class name is the same as the saved file name.
- Main Method: This is the first part of a Java application to be executed when it runs.
- Body: Write code according to Java syntax.

<br>

## How Java Works

Let's write and save 'hello java' as shown in the code above.
The file named HelloJava.java is created.
```
javac HelloJava.java
```
Compile it using a compiler.

```java
> java HelloJava
hello java
```
You can see that it executes successfully when run using the `java HelloJava` command.

- When you save Java code, a file named `OOO.java` is created. Compiling that file with `javac (compiler)` creates an `OOO.class` file, and the compiler converts it into bytecode.
- However, bytecode cannot be directly interpreted by the computer yet. The `JVM (Java Virtual Machine)` then performs internal processing to convert it into binary code that the computer can interpret.

<br>

## Principles of Java
1. It must be simple, object-oriented, and familiar. It must be simple, object-oriented, and familiar
2. It must be robust and secure. It must be robust and secure
3. It must be architecture-neutral and portable. It must be architecture-neutral and portable.
4. It must execute with high performance. It must execute with high performance.
5. It must be interpreted, threaded, and dynamic. It must be interpreted, threaded, and dynamic
