# TCP/IP Packet, TCP vs UDP

## TCP/IP Packet
TCP segments include source port, destination port, transmission control, sequence, and verification information, which can supplement the source IP and destination IP information of IP packets.

## TCP
Transmission Control Protocol
TCP is a relatively more reliable protocol compared to UDP, which belongs to the same layer.

- Connection-oriented - TCP 3-way handshake (virtual connection)
- Guaranteed data delivery
- Guaranteed order
- Reliable protocol

### Connection-oriented - TCP 3-way handshake (virtual connection)
TCP is a connection-oriented protocol that uses a 3-way handshake to establish a logical connection between devices.
![](https://velog.velcdn.com/images/mmmdo21/post/969064b5-5772-4b57-9495-112b61816766/image.png)

1. The client sends a SYN packet requesting a connection to the server.
2. The server receives the SYN request, sends a packet with ACK and SYN set, indicating acceptance of the request, and waits for the client to respond with ACK again.
3. The client sends an ACK to the server.
   1. From this point on, the connection is established, and data can be transmitted.
   2. If the server is off, the client sends a SYN, but since there's no response from the server, no data is sent.
   3. Currently, due to optimization, data is sometimes sent along with the 3rd ACK.

> SYN: Synchronize
> ACK: Acknowledgement

### Guaranteed data delivery

![](https://velog.velcdn.com/images/mmmdo21/post/942e43a0-1161-4e3d-8c00-b8c7eeccfe93/image.png)

TCP returns a response if data transmission is successful, thereby compensating for the connectionless nature, which is a limitation of IP packets.

### Guaranteed order
![](https://velog.velcdn.com/images/mmmdo21/post/cfaa75b1-b723-4e2d-b8ab-b1bac832f6a5/image.png)

If packets do not arrive in order, a retransmission of packets can be requested based on the information in the TCP segment.
  
This compensates for the unreliability (no guaranteed order) of IP packets.

<br>

## UDP

User Datagram Protocol
  
UDP is a simple protocol that only adds port and checksum field information to the IP protocol.

- Often compared to a blank canvas (has very few features)
  - HTTP3 uses UDP, and its advantage is that it's like a blank canvas, allowing for more customization than TCP, which already has many features implemented.
- Connectionless - no TCP 3-way handshake
- No guaranteed data delivery
- No guaranteed order
- Data delivery and order are not guaranteed, but it is simple and fast.
- Often used in services where continuity is more important than reliability (e.g., real-time streaming).

To illustrate the difference between TCP and UDP, you can compare them to a heavy library with all good features included versus a lightweight library with only essential features.

> A checksum is a form of redundancy check, a simple method to protect the integrity of data transmitted over space (telecommunication) or time (storage) through error detection.

## TCP vs UDP
![](https://velog.velcdn.com/images/mmmdo21/post/25c362f2-57d3-49ca-8155-83fa1ae070bf/image.png)
