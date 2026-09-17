# WebSocket, Difference from HTTP

## WebSocket Protocol
It is an advanced communication protocol that connects clients and servers (browsers and servers) and **enables real-time communication.**
  
WebSocket provides a full-duplex communication channel over a single TCP connection.
  
Simply put, WebSocket is a protocol that allows real-time bidirectional communication or data transfer while maintaining a Socket Connection.
  
Today, it is used in many fields such as chat applications, social media, Google Docs, multiplayer games like League of Legends, and video conferencing.

## Difference from HTTP

Traditional HTTP was a unidirectional communication.
  
It operated by the client sending a Request to the server, and the server sending a Response back to the client.
  
Furthermore, HTTP is fundamentally stateless, so it does not store state.
  
However, with WebSocket's bidirectional communication, once a connection is established, data can automatically come from the server even if the client does not request it.
  
This means data can be received without sending a separate request, unlike HTTP.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FBCkpf%2Fbtr4fVn5KF1%2FTh5ZK8vN5wfKMZE4SwIs11%2Fimg.png)

For example, let's say multiple users are simultaneously editing a document using Google Docs on the web.
  
As users who have used Google Docs know, changes made by other users are automatically applied in real-time without needing to refresh the page.
  
This is a technology that uses WebSocket.
  
Additionally, unlike HTTP, WebSocket is a stateful protocol. This means that once a client and server are connected, they communicate using the same connection, which saves TCP connection costs.

## How WebSocket Works

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbAXq5j%2Fbtr4hmk7b8i%2FqYt4Dhq7ThXCxKJpSIKSzK%2Fimg.png)

WebSocket operates over HTTP port 80 and HTTPS port 443.
  
WebSocket establishes a connection using a handshake, similar to a TCP connection.
  
At this point, it uses the HTTP Upgrade header to switch from the HTTP protocol to the WebSocket protocol.
  
In other words, during the initial connection, a handshake is performed using the HTTP protocol.
  
Once the connection is established, a persistent, identical channel is maintained unless either side closes the connection, and the HTTP protocol is switched to the WebSocket protocol.
  
At this time, protocols like WSS can be used to encrypt data.
