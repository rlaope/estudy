# SYN Cookie

SYN Cookie is a kernel technique that protects servers from SYN Flood attacks (which overload by sending many SYN requests) by maintaining TCP connections without storing state.

When TCP receives a SYN, it stores the corresponding connection in the SYN queue to identify it when the SYN-ACK is received. The problem is that if a SYN flood attack occurs, many connections can accumulate in this queue, leading to an overload.

Of course, this can also happen during high-volume traffic, not just attacks, but that's a different problem to solve. SYN flood attacks, however, must be prevented.

1.  Client SYN reception
2.  Store state in SYN queue
    1.  ISN (Initial Sequence Number)
    2.  MSS, Window Scale
    3.  Timer
3.  SYN-ACK transmission
4.  Upon ACK, state matching -> ESTABLISHED

The problem is SYN flood: attackers continuously send only SYN packets without sending ACKs, causing the SYN queue to fill up. This prevents legitimate clients from establishing connections.

What's needed here is SYN Cookie. The core idea is not to store anything in the SYN queue.

Instead, when the server sends a SYN-ACK, it encrypts information within the SEQ number that can verify if the SYN was legitimate.

In other words, instead of storing state, it encodes the state into a single number.

### Operation Flow

Client -> Server

```
SYN (client_isn)
```

Server does nothing

Server -> Client

```
SYN-ACK
SEQ = SYN_COOKIE(...)
```

The information contained within this SEQ value includes client IP, port, server IP, port, time slot (time window), and a secret key (internal to the kernel), generated using a hash function.

Client -> Server

```
ACK
ACK = SEQ + 1
```

Upon receiving the ACK number, the server recalculates the SYN Cookie. If the values match,

it identifies this client as a legitimate SYN client. You can think of it as a mechanism similar to an access token.

Only then does it create the connection state and send it to the accept queue.

| Attack            | Result                          |
| ----------------- | ------------------------------- |
| Sends only SYN    | No impact as no state is stored |
| Doesn't send ACK  | Server incurs no loss           |
| IP spoofing       | Cannot receive ACK → Invalid    |

### Limitations

However, SYN Cookie is not a panacea; it has the limitation of losing TCP options.

Because the number of bits that can be stored in the cookie is limited, information like window scale, SACK permitted, and some MSS details are often restricted, which can degrade performance and is disadvantageous for high-bandwidth connections.

Even with normal traffic, TCP performance degrades, RTT increases, and congestion control optimizations are lost.

Therefore, Linux is configured to use SYN Cookie only when the SYN queue is full.

```
net.ipv4.tcp_syncookies = 1
```

Meaning: Normally: Use SYN Queue, Upon Queue overflow: Activate SYN Cookie

Setting it to 2 always uses it, but this is not recommended.

The key insight is that '1' enables it, but not for constant use.
