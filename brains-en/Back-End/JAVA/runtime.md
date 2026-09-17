# RuntimeException and Exception
Let's create an exception ourselves and see how it can be used.

Consider the following example.
```java
public class Sample {
    public void sayNick(String nick) {
        if("fool".equals(nick)) {
            return;
        }
        System.out.println("당신의 별명은 "+nick+" 입니다.");
    }

    public static void main(String[] args) {
        Sample sample = new Sample();
        sample.sayNick("fool");
        sample.sayNick("genious");
    }
```
The sayNick method terminates the method with a return statement if the string "fool" is entered, preventing the nickname from being printed.

<br>

### RuntimeException
Now, if the string "fool" is entered, instead of simply returning, let's actively throw an exception.

Create a FoolException class like the following in the Sample.java file.
```java
class FoolException extends RuntimeException{

}
```
Then, let's modify the example as follows.
```java
class FoolException extends RuntimeException {
}

public class Sample {
    public void sayNick(String nick) {
        if("fool".equals(nick)) {
            throw new FoolException();
        }
        System.out.println("당신의 별명은 "+nick+" 입니다.");
    }

    public static void main(String[] args) {
        Sample sample = new Sample();
        sample.sayNick("fool");
        sample.sayNick("genious");
    }
}
```
The part that simply returned was changed to the statement `throw new FoolException()`.

Now, if you run the program above with "fool" as input to the sayNick method, the following exception will occur.
```
Exception in thread "main" FoolException
    at Sample.sayNick(Sample.java:7)
    at Sample.main(Sample.java:14)
```
The class that FoolException inherits from is RuntimeException. Exceptions are broadly divided into two types.

1. RuntimeException
2. Exception

RuntimeException is an **exception that occurs at runtime**, and Exception is an **exception that occurs at compile time**. In other words, Exception is used when writing exceptions that are already predictable during program development, while RuntimeException is used for cases that may or may not occur.

Therefore, Exception is also called a Checked Exception, and RuntimeException is called an Unchecked Exception.

<br>

### Exception
Now, let's change FoolException as follows.
```java
class FoolException extends Exception {

}
```
- We changed it from inheriting RuntimeException to inheriting Exception. This will cause a compile error in the Sample class.
- This is because it is a predictable Checked Exception, and the compiler **forces** exception handling.

By changing it as follows, it will compile successfully.
```java
class FoolException extends Exception {
}

public class Sample {
    public void sayNick(String nick) {
        try {
            if("fool".equals(nick)) {
                throw new FoolException();
            }
            System.out.println("당신의 별명은 "+nick+" 입니다.");
        }catch(FoolException e) {
            System.err.println("FoolException이 발생했습니다.");
        }
    }

    public static void main(String[] args) {
        Sample sample = new Sample();
        sample.sayNick("fool");
        sample.sayNick("genious");
    }
}
```
The sayNick method handled FoolException with a try...catch statement.

<br>

### Throwing Exceptions
- In the example above, the sayNick method threw a FoolException and also handled the exception within the sayNick method.
- Instead of doing this, there's a way to throw the exception up so that the caller of sayNick handles the FoolException.

Consider the following example.
```java
public class Sample {
    public void sayNick(String nick) throws FoolException {
        try {
            if("fool".equals(nick)) {
                throw new FoolException();
            }
            System.out.println("당신의 별명은 "+nick+" 입니다.");
        }catch(FoolException e) {
            System.err.println("FoolException이 발생했습니다.");
        }
    }

    public static void main(String[] args) {
        Sample sample = new Sample();
        sample.sayNick("fool");
        sample.sayNick("genious");
    }
}
```
You can send FoolException up by using the throws clause after the sayNick method. (This is also called "deferring the exception.")

If you change the sayNick method as above, a compile error will occur in the main method. This is because the target for handling FoolException has shifted from the sayNick method to the main method due to the throws clause.

Therefore, to resolve the compile error, you need to change the main method as follows.
```java
class FoolException extends Exception {
}

public class Sample {
    public void sayNick(String nick) throws FoolException {
        if("fool".equals(nick)) {
            throw new FoolException();
        }
        System.out.println("당신의 별명은 "+nick+" 입니다.");
    }

    public static void main(String[] args) {
        Sample sample = new Sample();
        try {
            sample.sayNick("fool");
            sample.sayNick("genious");
        } catch (FoolException e) {
            System.err.println("FoolException이 발생했습니다.");
        }
    }
}
```
- The main method handled the exception for the sayNick method.
- Which is better for handling exceptions, the main method or the sayNick method? Handling them in main versus sayNick makes a very significant difference.

If the exception is handled in the sayNick method, both of the following two statements will be executed.
```java
sample.sayNick("fool");
sample.sayNick("genius");
```
Of course, a FoolException will occur when `sample.sayNick("fool");` is executed, but the next statement, `sample.sayNick("genious");`, will also be executed.

However, if the exception is handled in the main method, the second statement, `sample.sayNick("genious");`, will not be executed. This is because an exception already occurred in the first statement, causing control to jump to the catch block.
```java
try {
    sample.sayNick("fool");
    sample.sayNick("genious");  // This statement will not be executed.
}catch(FoolException e) {
    System.err.println("FoolException이 발생했습니다.");
}
```
For these reasons, the location where an Exception is handled in programming is extremely important. It can determine whether the program continues execution and is closely related to transaction processing.
