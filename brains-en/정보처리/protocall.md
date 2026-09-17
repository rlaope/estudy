# Network Protocols
A network protocol is a formalized set of rules and formats for exchanging messages between computers or telecommunication devices.

> Basic Elements of a Protocol
> Syntax
> Semantics
> Timing

<br>

## IP (Internet Protocol)
- A unique identifier assigned to computers worldwide.
- IPv4 is an addressing system used since the early days of the internet, represented by 12 digits like 000.000.000.000, and can assign up to 4.3 billion addresses. Recently, due to the increase in devices, the number of addresses available with IPv4 may be insufficient, leading to the announcement of IPv6.
- `IPv6` is characterized by more efficient packet processing and enhanced security compared to its previous version.

### IPv4
- 32-bit addressing system
- Address depletion occurs (Number of addresses: 4.3 billion)
- Non-sequential address allocation in class units
- No built-in support mechanisms
- IPsec protocol requires separate installation
- Variable header size
- No PnP support mechanisms
- Webcasting difficulties
- Transmission methods: `multicast`, `broadcast`, `unicast`

### IPv6 (Important)
- IP address expansion (32 bit >> 128 bit)
- Automatic host address configuration
- Packet size extension
- Efficient routing (header format modification)
- Enhanced authentication and security features
- Mobility
- Standard format: 8 fields of 16 bits each, separated by colons
- Transmission methods: `multicast`, `anycast`, `unicast`

<br>

## Network Classes
Due to the increase in network devices, available IPv4 addresses became insufficient. To address this, IP address ranges were divided according to their intended use, making them easier to manage based on scale.

Class | First Octet of IP Address | Purpose | First Byte Range of IP Address
--|--|--|--
A | 0xxx xxxx | Large Organizations (Intercontinental) | 0 ~ 127
B | 10xx xxxx | Medium Organizations (Inter-country) | 128 ~ 191
C | 110x xxxx | Small Organizations (Inter-company) | 192 ~ 223
D | 1110 xxxx | Group Communication, Multicast | 224 ~ 239
E | 1111 xxxx | Research, Experimental Use | 240 ~ 254

<br>

## TCP/IP Protocol

- TCP/IP refers not only to the TCP and IP protocols but also collectively to related protocols such as UDP, ICMP, ARP, and RARP.
- Protocols categorized as TCP and UDP are responsible for communication between the application layer and the internet layer at the transport layer.

### Differences between TCP and UDP
- The biggest difference between the two is the reliability of data transmission.
- TCP transmits data by checking reception status and other factors step-by-step according to the receiver's readiness, whereas UDP merely sends data over the network without performing any verification.

<br>

### TCP
- Ensures reliable transmission through CRC checks and retransmission features.
- Performs Flow Control to check data transmission status step-by-step.
- Supports a logical 1:1 virtual circuit, ensuring data is delivered only through that specific path.
- Representative services: FTP, Telnet, HTTP, SMTP, POP, IMAP, etc.

### UDP
- Can transmit data even if connected. However, it does not verify whether the receiver has received it.
- Does not perform Flow Control or Error Control, making it unsuitable for reliable data transmission.
- Used when multiple recipients need to receive a single piece of transmitted information.
- Representative services: SNMP, DNS, TFTP, NFS, NETBIOS, internet games/broadcasting/stock trading, etc.
