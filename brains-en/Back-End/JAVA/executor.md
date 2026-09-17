# Java Executors

Executors delegate the task of creating and managing threads to a high-level API.

They are responsible for creating, processing, executing, and terminating threads.

## ExecutorService
First, ExecutorService is an interface that inherits from Executor, capable of executing Callables, terminating the Executor, or providing concurrent execution of Callables.

```java
public class ExecutorEx {
    public static void main(String[] args) {
        ExecutorService executorService = Executors.newSingleThreadExecutor();
        executorService.submit(() -> {
            System.out.println("Thread " + Thread.currentThread().getName());
        });
    }
}
```
If you run this code,
![](./image/newSingleThreadExecutorEx.png)

As such, `newSingleThreadExecutor()` creates an Executor that uses a single thread operating on an unbounded queue.
Therefore, you can observe that the process runs, continues to persist, and does not terminate.

If you want to terminate the process, you must close it using a shutdown method: `executorService.shutdown();`

You can also leverage multi-processing using the Fork/Join framework.
