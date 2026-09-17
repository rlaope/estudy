# Exception Handling

When developing programs, countless errors occur. Of course, these errors are Java's way of preventing programs from malfunctioning. However, sometimes we want to ignore these errors, and other times we want to handle them appropriately when they occur. To address this, Java uses `try catch` statements to handle errors.

<br>

### When do exceptions occur?

- Let's try to open a non-existent file.
```java
BufferedReader br = new BufferedReader(new FileReader("나없는파일"));
br.readLine();
br.close();
```
- Executing the code above will result in the following error.

```
Exception in thread "main" java.io.FileNotFoundException: 나없는파일 (지정된 파일을 찾을 수 없습니다)
    at java.io.FileInputStream.open(Native Method)
    at java.io.FileInputStream.<init>(Unknown Source)
    at java.io.FileInputStream.<init>(Unknown Source)
    at java.io.FileReader.<init>(Unknown Source)
    ...
```
Attempting to open a non-existent file will raise an exception named `FileNotFoundException`.

- Now, let's look at the case of dividing a number by zero.

```java
int c = 4 / 0;
```
Executing the code above will result in the following error.
```
Exception in thread "main" java.lang.ArithmeticException: / by zero
    at Test.main(Test.java:14)
```
Dividing 4 by 0 will raise an `ArithmeticException`.

Finally, let's look at one more error. The following error occurs very frequently.

```java
int[] a = {1, 2, 3};
System.out.println(a[3]);
```
The error message is as follows.
```
Exception in thread "main" java.lang.ArrayIndexOutOfBoundsException: 3
    at Test.main(Test.java:17)
```
Since `a[3]` is the 4th value in array `a`, it's a value that cannot be obtained from array `a`. Therefore, an `ArrayIndexOutOfBoundsException` occurred.

When such an exception occurs, Java stops the program and displays an error message.

<br>

### Handling Exceptions
- The following is the basic structure of a `try catch` statement for exception handling.

```java
try{
  ...
}catch(예외1){
  ...
} catch(예외 2){

}
```
- If no exception occurs in the statements within the `try` block, the `catch` blocks will not be executed.
- If an exception occurs while executing a statement within the `try` block, the corresponding `catch` block for that exception will be executed.
- To handle an exception that occurs when dividing a number by zero, you can do the following.

```java
int c;
try {
    c = 4 / 0;
} catch(ArithmeticException e) {
    c = -1;  // 예외가 발생하여 이 문장이 수행된다.
}
```
This handles the exception by assigning -1 to `c` if an `ArithmeticException` occurs. In `ArithmeticException e`, `e` refers to an object of the `ArithmeticException` class, which is an error object. Through this error object, methods of the corresponding exception class can be called.
<br>

### finally
- If an exception occurs during program execution, the program either stops or a `catch` block is executed due to exception handling.
- But what if there's a part that must be executed regardless of which exception occurs?

Let's look at the following example.
```JAVA
public class Sample {
    public void shouldBeRun() {
        System.out.println("ok thanks.");
    }

    public static void main(String[] args) {
        Sample sample = new Sample();
        int c;
        try {
            c = 4 / 0;
            sample.shouldBeRun();  // 이 코드는 실행되지 않는다.
        } catch (ArithmeticException e) {
            c = -1;
        }
    }
}
```
- The `finally` block is always executed, regardless of whether an exception occurred during the execution of the `try` block.
- Executing the code above will cause the `sample.shouldBeRun()` method to be executed, printing the message "ok.thanks".
