# TCP & UDP

4 Layers of the Internet Protocol Stack
Application Layer - HTTP, FTP
Transport Layer - TCP, UDP
Internet Layer - IP
Network Interface Layer

> EX) When using a chat program

1. Program writes "hello world" message
2. Delivered via socket library.
3. `TCP information created`, message data included.
4. `IP packet created`, TCP data included
5. Sent to server via LAN card (Ethernet frame)

TCP/IP Packet Information: TCP packet information is contained within IP packet information.
TCP Segment: Source port, destination port, transmission control, sequence, checksum information

### TCP Features
Transmission Control Protocol
- Connection-oriented - TCP 3-way handshake (virtual connection): Not a true connection, but conceptually connected.
- Data delivery guarantee: When a client sends data to a server, the server sends an acknowledgment to the client that the data was received successfully.
- Order guarantee: If the order is incorrect, a retransmission request is sent.
- Reliable protocol
- Currently, most applications use TCP

### UDP Features
User Datagram Protocol
- Compared to a blank canvas (has very few features).
- Connectionless - No TCP 3-way handshake
- No data delivery guarantee
- No order guarantee
- Data delivery and order are not guaranteed, but it is simple and fast.
- Almost identical to IP, with only PORT checksum added; additional work required at the application layer.

<br>

port: When multiple applications (games, music, etc.) are used on a single IP, it distinguishes which packets are for games and which are for music.

TCP is good, but it cannot achieve high transmission speeds, and the amount of data also increases. Furthermore, because TCP cannot be modified, you can build what you want on top of UDP. HTTP/3 uses the UDP protocol.
