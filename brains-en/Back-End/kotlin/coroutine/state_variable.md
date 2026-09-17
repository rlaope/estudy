# Coroutine State Variables

### Job's State Variables: isActive, isCancelled, isCompleted

A Job has three state variables, which can be accessed as shown in the figure below.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Flo6k2%2FbtrcHfsBY18%2F7GZyCKAPzy2uT7ELFpRIZK%2Fimg.png)

- `isActive`: Indicates whether the Job is running.
- `isCancelled`: Indicates whether a Job cancel request has been made.
- `isCompleted`: Indicates whether the Job's execution is complete or cancellation is complete.

#### Changes when a Job is created with the CoroutineStart.LAZY option

Let's explore the Job's state variables using the `CoroutineStart.LAZY` option to prevent the Job from automatically transitioning from created to running state.

When a Job is created with `CoroutineStart.LAZY`, it remains in the New state. At this point, it is neither running nor canceling, and `isActive`, `isCancelled`, and `isCompleted` are all false.

Now, when the Job transitions to the running state via `start()` or `join()`, `isActive` changes to true.

<br>

#### State Changes when a Job is Cancelled

Let's find out what happens to the Job's state when `cancel` is called.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FU4uFO%2FbtrcMHaTK6C%2F6avfsllmHHqyRKDqXb1Nxk%2Fimg.png)

First, the Job transitions to the **Cancelling** state. `isCancelled` is a variable indicating whether cancellation has been requested, so it always becomes true when `cancel` is called. However, since cancellation is not yet complete while in the cancelling state, `isCompleted` is false.

If cancellation is completed, `isCompleted()` changes to true. `invokeOnCompletion` is a method that observes the state of `isCompleted` and is called when `isCompleted` changes from false to true. Therefore, it is also called when cancellation is complete.

<br>

#### State Changes when a Job is Completed

When a Job is completed, `isActive` changes from true to false, and `isCompleted` changes from false to true.

Since `isCompleted` changed from false to true, the lambda expression set via `invokeOnCompletion` is called, just as with cancellation.
