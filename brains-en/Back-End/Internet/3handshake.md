# 3-way / 4-way Handshake

## 3-way Handshake

TCP performs a 3-way handshake to establish a logical connection between devices.

**The TCP 3-Way Handshake refers to the process by which an application communicating using the TCP/IP protocol establishes a session with the other computer in advance to ensure accurate data transmission before sending any data.**

- Client -> Server: TCP SYN
- Server -> Client: TCP SYN, ACK
- Client -> Server: TCP ACK

> SYN: synchronized sequence numbers
> ACK: Acknowlegment

This procedure is essential for successfully establishing a TCP connection.

<br>

## Role of TCP 3-way Handshaking

It ensures that both sides are ready to transmit data and informs one side that the other is ready before data transfer actually begins.

It allows both sides to obtain the initial sequence numbers for the other party.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbNwPCT%2FbtqD0hCftBa%2F4fUpGdt1ddNBtk9RGmfKw0%2Fimg.png)

### Process

1.  The client sends a SYN packet requesting a connection to the server. At this point, the client is in the SYN_SENT state, waiting for a SYN/ACK response, and the server is in the Wait for Client state.

2.  The server receives the SYN request, sends a packet with the ACK and SYN flags set to acknowledge the request, and waits for the client to respond with an ACK. At this point, the server enters the SYN_RECEIVED state.

3.  The client sends an ACK to the server, after which the connection is established and data is exchanged. The server's state at this point is Established.

<br>

## 4-way Handshake

While the 3-way handshake is used to initialize a TCP connection.

The 4-way handshake is **a procedure performed to terminate a session.**

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FqUXSw%2FbtqDWsFNWJw%2FhVdKIneSYb7UK3wc0pj6Z0%2Fimg.png)

### Process

1.  The client sends a FIN flag to indicate it wants to terminate the connection. At this point, the client enters the FIN-WAIT state.

2.  The server receives the FIN flag, sends an ACK acknowledgment, and waits until its own communication is finished. This state is the server's CLOSE_WAIT state.

3.  When it's ready to terminate the connection, the server sends a FIN flag to the client to indicate it's ready for connection termination. At this point, the server's state is LAST_ACK.

4.  The client sends an ACK message confirming that it has received the termination readiness. The client's state changes from FIN-WAIT to TIME-WAIT.

<br>

What if, before the server sends the FIN, a packet sent earlier arrives later than the FIN packet due to routing delays or retransmissions caused by packet loss?

If a packet arrives late after the client has terminated the session, the packet will be dropped, and data will be lost.

To prevent this, even after receiving a FIN from the server, the client keeps the session open for a certain period (default: 240s) and waits for any remaining packets. This process is called TIME_WAIT.

After a certain period, the session expires, the connection is terminated, and it transitions to the CLOSE state.
