# Promise

1.  Producer code: Performs time-consuming tasks, such as loading scripts remotely.
2.  Consumer code: Waits for the result of the `producer code` and consumes it. There can be multiple subjects (functions) involved.
3.  Promise: A special JavaScript object that connects `producer code` and `consumer code`. A promise ensures that all consumer code can use the result once the `producer code` is ready to produce the promised result, regardless of how long it takes.

<br>

A promise object can be created using the following syntax.
```js
promise = new Promise(function(resolve, reject)){
  // executor (producer code, singer)
}
```
> The function passed to new Promise is called an executor (executor, execution function).
> The executor runs automatically when new Promise is created and contains the producer code that ultimately generates the result.
> `resolve` and `reject` are built-in callbacks provided by JavaScript. `Developers only need to write the code inside the executor, without worrying about them.`

-   resolve(value): Called with `value` representing the result when the job finishes successfully.
-   reject(error): Called with an `error` object when an error occurs.

<br>

The promise object returned by the new Promise constructor has the following internal properties.
-   `state`: Initially `pending`, it changes to `"fulfilled"` when `resolve` is called, and `"rejected"` when `reject` is called.
-   `result`: Initially `undefined`, it changes to `value` when `resolve` is called, and `error` when `reject` is called.

```js
let promise = new Promise(function(resolve, reject)){
  // When the promise is created, the executor function runs automatically.

  // After 1 second, a signal is sent that the job finished successfully,
  // and the result becomes '완료' (value).
  setTimeout(() => resolve('완료'), 1000);
};
```
> 1.  The executor is called automatically and immediately by new Promise.
> 2.  The executor receives `resolve` and `reject` functions as arguments. These functions are predefined by the JavaScript engine, so developers don't need to create them. However, either `resolve` or `reject` must be called.

<br>

A promise that has successfully completed its task is called a 'fulfilled promise'.
Now, let's look at the case where the executor rejects the promised task with an error.

```js
let promise = new Promise(function(resolve, reject){
  // After 1 second, sends a signal that execution ended with an error.
  setTimeout(() => reject(new Error("에러 !발생")), 1000);
});
```

After 1 second, when `reject` is called, the promise's state changes to `"rejected"`.

<br>

## Summary
The executor typically performs time-consuming tasks. When the task is complete, it calls either the `resolve` or `reject` function, which changes the state of the promise object.

<br>

### A promise either succeeds or fails.

-   The executor must call either `resolve` or `reject`. Once the state changes, it cannot change again.

```js
let promise = new Promise(function(resolve, reject)){
  resolve("완료");

  reject(new Error(",,,")); // ignored
}; 
```

<br>

### Rejecting with an Error object

-   If something goes wrong, the executor should call `reject`. The argument can be of any type, just like with `resolve`, but it's recommended to use an `Error` object or an object inheriting from `Error`. The reason will be explained later.

### Calling resolve and reject functions immediately

-   The executor usually performs something asynchronously and calls `resolve` or `reject` after some time, but this is not strictly necessary. You can also call `resolve` or `reject` immediately, as shown below.

```js
let promise = new Promise(function(resolve, reject){
  // The task doesn't take time to complete
  resolve(123); // Immediately passes result 123 to resolve
});
```

If a task is started but it turns out the work is already done and saved, you can use this method to call `resolve` or `reject` immediately.
This makes the promise immediately fulfilled.

<br>

### `state` and `result` are internal
-   The `state` and `result` properties of a promise object are internal, so developers cannot access them directly.
-   They can be accessed using the `.then`, `.catch`, or `.finally` methods, which we will explore in detail below.

<br>

## Consumers: then, catch, finally
-   A promise object acts as a bridge between the executor and consumer functions (`fans`) that receive the result or error. Consumer functions are registered using the `.then`, `.catch`, and `finally` methods.

<br>

### then
> The most important and fundamental method in promises

```js
promise.then(
  function(result){/* handles the result */}.
  function(error){/* handles the error. */}
);
```
The first argument of `.then` is a function that executes when the promise is fulfilled, and it receives the result of the execution.
The second argument of `.then` is a function that executes when the promise is rejected, and it receives the error.

```js
let promise = new Promise(function(resolve,reject){
  set Timeout(() => resolve("완료!"),1000);
});
// The resolve function executes the first function in .then.
promise.then(
  result => alert(result); // Outputs 'Done!' after 1 second
  error => alert(error); // Not executed.
)
```

<br>

### catch
If you only want to handle errors, you can pass `null` as the first argument, like `.then(null, errorHandlingFunction)`. You can also use `.catch(errorHandlingFunction)`, which works identically to passing `null` to `.then`.
```js
let promise = new Promise((resolve, reject) => {
  setTimeout(() => reject(new Error("에러 발생!")), 1000);
});
// .catch(f) works identically to promise.then(null , f)
promise.catch(alert); // Outputs 'Error occurred!' after 1 second
```
`.catch(f)` is perfectly identical to `.then(null, f)`, except for its more concise syntax.

<br>

### finally
-   Just as `try{...} catch{...}` has a `finally` clause, promises also have `finally`.
-   Calling `.finally(f)` is similar to `.then(f, f)`.
-   `finally` is useful when cleanup is needed regardless of the outcome, such as stopping a no-longer-needed loading indicator.

```js
new Promise((resolve,reject) => {
  /* Performs some time-consuming task, then calls resolve or reject */
});
// Executed when the promise is settled, regardless of success or failure
.finally(() => 로딩 인디케이터 중지)
.then(result => result와 err 보여줌 => error 보여줌)
```

<br>

### Differences between then(f,f) and finally
1.  A `finally` handler has no arguments. It doesn't know if the promise was fulfilled or rejected.
> It performs universal actions, so it doesn't need to know about success or failure.

2.  A `finally` handler automatically passes the result and error to the next handler.
> Let's check that the result is passed through `finally` to `then`.

```js
new Promise((resolve, reject) => {
  setTimeout(() => resolve('결과'), 2000);
});
  .finally(() => alert("Promise is ready"));
  .then(result => alert(result)); // `then` can handle the result
```
Let's check that if an error occurs in a promise, it passes through `finally` to `catch`.
```js
new Promise((resolve, reject) => {
  throw new Error("에러 발생");
});
  .finally(() => alert("Promise is ready."));
  .catch(err => alert(err)); // `catch` can handle the error object
```
`finally` is not designed to process the promise's result. The promise's result passes through `finally`, and this characteristic is very useful.
