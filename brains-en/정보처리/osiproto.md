# Protocols by Network Layer

### OSI 7 Layers
- A model created by `ISO` that structures networks into 7 layers.

### Protocol
- A protocol is a set of agreed-upon rules for mutual connection, transmission methods, communication methods, data formats to be exchanged, error detection methods, code conversion methods, and transmission speeds.

### Protocols by Layer
Refers to the conventions for network communication that exist between the layers of the OSI 7-layer model.

<br>

## Explanation of Major Protocols by OSI 7 Layers

### Application Layer
- HTTP: A protocol for exchanging information on the WWW, primarily used for exchanging HTML documents, uses TCP and UDP, port number: 80
- SMTP: A protocol used for sending and receiving emails over the internet, TCP port number: 25
- FTP: A protocol used for transferring files between computers (data transfer: port 20, control information transfer: port 21)
- TELNET: A network protocol used for internet or local area network connections, standardized as IETF STD 8, its use is declining due to security issues, and it is being replaced by SSH for remote control

### Presentation Layer
- SSL: An encryption method at the network layer, used not only with HTTP but also with NNTP, FTP, etc. A protocol that ensures authentication, encryption, and integrity
- ASCII: Used in many devices that use characters, most character encodings are ASCII-based. 7-bit encoding, 33 non-printable control characters and 95 printable characters including space

### Session Layer
- NetBIOS: A convention that defines basic input/output for networks
- RPC: A remote procedure call protocol used in Windows operating systems
- WinSock: An implementation of the Socket used for TCP/IP communication in Unix and other systems, directly in Windows

### Transport Layer
- TCP: Transmission Control Protocol, a protocol that controls information transfer in a network, guarantees data delivery and ensures it is received in the order sent. Enables reliable transmission using mechanisms like 3-Way Handshaking and 4-Way Handshaking
- UDP: Connectionless, unreliable, and provides an unordered Datagram service (TCP is suitable for programs with low reliability)

### Network Layer
- IP: An information-centric protocol used for exchanging information in packet-switched networks, responsible for host addressing and packet fragmentation and reassembly
- ICMP: A protocol used to notify problems that occur when processing IP packets in TCP/IP. It performs other functions necessary at the IP layer, such as diagnostics
- IGMP: A communication protocol for implementing IP multicast, a convention that notifies routers that a PC can communicate via multicast

### Data Link Layer
- Ethernet: Connectionless mode, transmission speed 10Mbps or higher, refers to a LAN implementation method
- HDLC: A general-purpose data link transmission control procedure suitable for high-speed data transmission and based on bit transmission
- PPP: A protocol that supports communication between two computers using an asynchronous serial link, such as a telephone line, between two endpoints

### Physical Layer
- RS-232: A serial interface for transmission up to 38400bps over short distances, typically 15 meters or less
- X.25 / X.21: X.25 is an access standard for packet-switched networks, and X.21 for circuit-switched networks
