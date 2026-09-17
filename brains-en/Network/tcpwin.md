# TCP Window Size, MSS, SACK Permitted

### Window Size

TCP Window Size is the amount of data that can be sent at once without an ACK.

More precisely, it's the receiver's current buffer size, serving as a mechanism for flow control.

```
Window = 64KB
-> Sender can transmit up to 64KB before receiving an ACK
```

TCP must satisfy the following simultaneously:
- If the sender transmits too quickly, the receiver's buffer overflows.
- If it transmits too slowly, bandwidth is wasted.

Therefore, the receiver communicates with each ACK.

It can accept the current request up to the window size.

This is the window size, and the key point is that it's based on the receiver, and the sender must never exceed this value.

```
Actual transmittable amount = min(rwnd, cwnd)
```
- rwnd : Receiver Window (Window Size)
- cwnd : Congestion Window (sender, congestion control)

The window field in the TCP header is 16 bits.

```
Maximum 65,535 bytes ≈ 64KB
```

Thus, this value alone cannot fully utilize bandwidth in networks with high RTT. This led to the development of window scaling.

It can be extended with the window scale option, negotiated during SYN/SYN-ACK. In practice, the window operates as advertised_window << scale.

```
Advertised Window = 64KB
Scale = 7
→ Actual Window = 64KB × 2⁷ = 8MB
```

In Linux, the actual adjustment is performed using kernel auto-tuning.

```
net.ipv4.tcp_window_scaling = 1
net.ipv4.tcp_rmem = 4096 87380 6291456
net.ipv4.tcp_wmem = 4096 65536 6291456
```

`rmem` and `wmem` are in the order of min, default, max, separated by spaces, and the key is to provide a generous range rather than fixed values.

While it's possible to use `socket.setReceiveBufferSize` at the application level, it might be useful if you only want specific tasks to operate that way.

### MSS (Max Segment Size)

MSS is the maximum payload size that can be carried in a single TCP segment. In other words, it's the maximum data size per packet.

```
MTU = 1500
IP Header = 20
TCP Header = 20
→ MSS = 1460 bytes
```

This is typical, and it's used to avoid IP fragmentation, reduce network device load, and decrease retransmission costs.

It's a mechanism for adjusting packet size units.

The key point is that MSS is an upper limit observed by the sender, and both sides exchange it during connection establishment.

If MSS is small, the number of packets increases, and overhead rises.

| Category    | Window Size | MSS           |
| ----------- | ----------- | ------------- |
| Meaning     | Total amount sent at once | Size of one packet |
| Control Target | Flow control | Packet unit   |
| Who determines | Receiver    | Both sides negotiate |
| Impact      | RTT utilization | Fragmentation |

### SACK Permitted

SACK (Selective ACK) provides the functionality to precisely indicate which segments are missing.

Basically, a TCP ACK only states 'everything up to here has been received.' SACK, however, can indicate '10-20KB received, 20-30KB not received, and 30-40KB received.'

#### Without SACK

It's needed when loss occurs; if one segment is missing, subsequent received segments are treated as if they were all lost, leading to mass retransmission.

#### With SACK

With SACK, only the missing segments can be retransmitted, saving bandwidth. RTT waste is reduced.

Negotiation happens as an option during the SYN/SYN-ACK phase. Both sides must have SACK Permitted for it to be activated.

```
MSS  → Size of one packet
Window Size → Total amount sent simultaneously
SACK → Retransmission strategy upon loss
```
