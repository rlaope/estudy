# Netty

Netty is an asynchronous, event-driven network application framework based on Java.

### Why use Netty?

Netty allows handling various protocols such as TCP, UDP, HTTP, and WebSockets, providing **high performance, scalability, and flexibility**.

Compared to implementing network applications solely through Java's socket programming, Netty's abstraction demonstrates how concisely Java network programming can be done.

### Directionality of Data Movement in Netty

Netty categorizes events into Inbound events and Outbound events.

If you are interested in events that occur when receiving data from a client, you can place your desired logic in the method responsible for a data reception event, which is one of the Inbound events.

## How it Works

![](https://velog.velcdn.com/images%2Fmonami%2Fpost%2F86203714-fdd3-441a-ba29-16ae9742b9dc%2FKakaoTalk_20210911_222645477.jpg)

Since the operating methods of sockets differ, the methods for I/O and the program call structure also differ.

However, Netty provides an abstracted transport API that allows development regardless of the socket mode.

Therefore, you don't need to modify the data transmission/reception logic to change the socket mode.

## NIO (Non-Blocking I/O)

NIO is one of the I/O processing methods provided in Java, and unlike the existing stream-based I/O processing method, it handles operations using channels and buffers.
