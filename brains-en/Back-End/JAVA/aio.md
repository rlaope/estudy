# Java AIO

It is an Asynchronous I/O API provided by the Java NIO package.

It does not block when performing I/O operations, allowing other tasks to be executed.

> Unlike NIO, it uses the CompletionHandler interface to receive notifications when I/O operations are complete.


### AIO Support
- AsynchronousChannel class
- AsynchronousSocketChannel class
- AsynchronousServerSocketChannel class
- AsynchronousFileChannel class
- callback, future support


It uses event notification `system calls` such as `Thread pool`, `epoll`, and `kqueue`.

When I/O is ready, asynchronous logic can be processed using Future or a callback.


### AsynchronousSocketChannel

An asynchronous channel for client sockets.

### AsynchronousServerSocketChannel

An asynchronous channel for server (TCP) sockets.

### AsynchronousFileChannel

An asynchronous channel for file read and write operations.

<br>

## CompletionHandler


An interface in the Java AIO API that handles the completion of asynchronous I/O operations.

By implementing this interface, you can define callback methods that are invoked when an I/O operation completes.

CompletionHandler has generic types, which are:
1. The result type of the I/O operation
2. The type of the object to be used when processing the I/O operation result

These are the two types.

By implementing this interface, you can define the `completed()` method, which is automatically called when an I/O operation finishes, and the `failed()` method, which is called when an exception occurs during an I/O operation.

This allows asynchronous I/O operations to be handled using a **callback approach**.
