# IP (Internet Protocol)

### What is IP?
- IP (Internet Protocol) is a unique address assigned to each device (computers, server equipment, smartphones, etc.) connected to the internet, enabling them to be identified.

### Role of IP
- Deliver data to the specified IP address
- Deliver data in communication units called packets

### Packet Delivery Process
- IP packet information is required: source IP, destination IP, etc.
- `Client Packet Delivery`: When a client attempts to deliver data to a server, it passes through multiple nodes to deliver the data to the destination IP.
- `Server Packet Delivery`: When a server delivers data to a client, it also sends the information to the client's IP.

<br>

### Limitations of the IP Protocol

#### Connectionless
- Packet transmission even if there is no recipient or the service is unavailable

#### Unreliable
- What if packets disappear midway?
- What if packets arrive out of order?

#### Program Differentiation
- What if there are two or more applications running on a server using the same IP?

#### Resulting Problems
Target service unavailable, packet transmission ->
1. Don't know if the target server can receive packets
2. Packet loss
3. Packet delivery order issues occur

<br>

> Thus, TCP and UDP emerged.
