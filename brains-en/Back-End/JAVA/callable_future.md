# Java Callable, Future

## Future
Future is a class that implements Runnable and is executed by an Executor.
return null as a result of the underlying task. -> returns null, which can lead to NPE issues.
  
Threads have in common that they execute functions implemented by Runnable and Callable, but there are also differences.  

- Runnable: Does not return any object. Does not throw exceptions.
- Callable: Returns an object of a specific type. Throws exceptions.

```java
public class CallableAndFuture {
    public static void main(String[] args) throws ExecutionException, InterruptedException {
        ExecutorService executorService = Executors.newSingleThreadExecutor();

        Callable<String> hello = () -> {
            Thread.sleep(1000L);
            return "Hello";
        };
        Callable<String> hope = () -> {
            Thread.sleep(1000L);
            return "hope";
        };

        System.out.println("========== callable =========");
        Future<String> submit = executorService.submit(hello);
        System.out.println(submit.get());
    }
}
```

By implementing a Callable type function in advance like this, you can add the task via `executorService.submit()` and immediately block for its result using `.get()`.
  
This is, of course, also possible with lambda expressions, as lambdas infer types.

```java
Future<String> submit = executorService.submit(() -> {
    Thread.sleep(2000L);
    return "asdfgh";
});
System.out.println(submit.get());
```

## Checking Process Status
You can check if a process has finished using `submit.isDone();`, which returns a boolean.
You can forcibly terminate a process using `submit.cancel(true);`.
  
If you do this, all subsequent detailed methods related to the task will be disabled.

![](./image/disabled.png)

## Executing Multiple Tasks Concurrently
This is possible using `ExecutorService.invokeAll()`.
```java
public class CallableAndFuture {
    public static void main(String[] args) throws ExecutionException, InterruptedException {
        ExecutorService executorService = Executors.newSingleThreadExecutor();

        Callable<String> hello = () -> {
            Thread.sleep(1000L);
            return "김희망";
        };
        Callable<String> hope = () -> {
            Thread.sleep(1000L);
            return "hope";
        };
        Callable<String> esperer = () -> {
            Thread.sleep(2000L);
            return "esperer";
        };

        List<Future<String>> futures = executorService.invokeAll(Arrays.asList(hello, hope, esperer));
        for (Future<String> f : futures){
            System.out.println(f.get());
        }

        executorService.shutdown();
    }
}
```

`invokeAll()` accepts a Collection of tasks, so `Arrays.asList()` is used to pass the parameters.
This `invokeAll()` method also has a specific characteristic; let's refer to the API docs below.

![](./image/invokeAll-api-docs.png)

As shown, `.isDone()` only becomes true when all tasks (hello, hope, esperer – 3 tasks) are completed.
In conclusion, `isDone` will only return true after even the longest-running task among the three, which takes 2 seconds, has completed.
