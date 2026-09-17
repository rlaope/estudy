# TCP Congestion Control

TCP Congestion Control is a technique that prevents network collapse by adjusting data transmission so that it does not exceed the network's processing capacity.

Before diving into specific algorithms (like Reno, CUBIC, etc.), let's first understand the fundamental operating principles.

### Flow Control vs Congestion Control

Both concepts involve **regulating data transmission volume**, but they differ in their protected entities and criteria.

| Comparison Item | Flow Control | Congestion Control |
| :--- | :--- | :--- |
| **Protected Entity** | **Receiver** | **Network** |
| **Purpose** | Prevent receiver buffer overflow | Prevent congestion collapse of network routers/links |
| **Agent** | Sender considers receiver's state | Sender infers network conditions |
| **Key Variable** | $rwnd$ (Receive Window) | $cwnd$ (Congestion Window) |
| **Operation Method** | Receiver informs `window size` in header | Sender detects packet loss/delay and adjusts |

The core formula states that the actual window size (Sending Window) that the sender can transmit follows the smaller of the two values. ActualWindow = `min(rwnd, cwnd)`

### Key Variables

These three variables are key to driving the congestion control state machine.

- cwnd (Congestion Window)
  - The estimated size of data (in MSS units or bytes) that the sender believes the **network can currently handle**.
  - Dynamically changes based on network conditions.
- rwnd (Receive Window)
  - A value that the receiver informs, indicating how much buffer space it has left, included in the TCP header.
- ssthresh (Slow Start Threshold)
  - The threshold for transitioning from the slow start phase to the congestion avoidance phase.
  - Generally, if packet loss occurs, it is reduced to half of the current cwnd. We will explore this phase below.

### 4 Key Phases

TCP congestion control cycles through the following four phases to find the optimal transmission rate.

#### 1. Slow Start

- **Principle**: At the beginning of a connection, since the network capacity is unknown, it starts with a very small size (1 MSS) and exponentially increases the window.
- **Operation**: `cwnd` increases by 1 for each ACK received (doubles every RTT: 1, 2, 4, 8, 16...).
- **Termination Condition**: When `cwnd` reaches `ssthresh`, it terminates and transitions to the Congestion Avoidance phase.
  - Despite its name, this is actually the phase where the speed increases most rapidly.

#### 2. Congestion Avoidance

- **Principle**: Once the `ssthresh` threshold is exceeded, the risk of network congestion is high, so the window is increased cautiously and linearly.
- **Operation**: `cwnd` increases by only 1 MSS per RTT (increases by 1/`cwnd` for each ACK).
- AIMD (Additive Increase / Multiplicative Decrease):
  - AI (Additive Increase): If there is no congestion, it increases linearly by 1.
  - MD (Multiplicative Decrease): If congestion is detected (packet loss), the window is halved.

#### 3. Fast Retransmit

- **Problem**: Waiting for a timeout (RTO) when a packet is lost is too long.
- **Solution**: When the receiver receives an out-of-order packet, it continuously sends ACKs for the last correctly received packet.
- **Operation**: If the sender receives three duplicate ACKs, it immediately retransmits the corresponding packet without waiting for a timeout. This is considered mild congestion.

#### 4. Fast Recovery

- **Principle**: After Fast Retransmit, the network has not completely stopped, so it does not return to the beginning (Slow Start).
- **Operation**
  - `ssthresh` is reduced to half of the current `cwnd`.
  - `cwnd` is set to the new `ssthresh` value (the halved value) + 3 MSS (to account for duplicate ACKs).
  - It immediately enters the Congestion Avoidance phase and begins linear growth.
  - Note: If a timeout occurs, it is considered severe congestion, `cwnd` is reset to 1, and the slow start phase is re-entered.

### ACK Clocking

The sender does not rely on a timer to send packets but rather uses incoming ACKs as a signal (clock) to transmit the next packet.

Regarding the **Conservation of Packets**, an arriving ACK means that **one packet has exited the network pipe and reached the receiver**.

This means that a vacant space has opened in the pipe, allowing the sender to push a new packet into that spot.

Through this, the sender can naturally adjust the transmission rate to match the network's processing speed (bottleneck link speed) without complex rate calculations, which is called self-clocking.

### Summary

Start: `cwnd` = 1, slow start (exponential increase)

Threshold reached: `cwnd` >= `ssthresh`, Congestion Avoidance (linear increase)

Packet loss detected:
- 3 Dup ACKs: Fast Retransmit/Recovery -> Halve `cwnd` and enter Congestion Avoidance
- Timeout -> Considered severe congestion, `cwnd` = 1, re-enter slow start

### Is the starting `cwnd` of 1 based on the connection?

Yes. Independent management: TCP congestion control variables like `cwnd` and `ssthresh` are maintained independently for each TCP connection.

For example, if I open Google and Naver simultaneously on my computer, the `cwnd` for the Naver connection does not increase just because the Google connection is in good condition. Each starts from 1 or its initial value and operates independently.

In older TCP theories like Tahoe/Reno, starting with 1 MSS (approx. 1.5 KB) was the standard.

However, in modern web environments, it is common to start with IW10 (Initial Window = 10 MSS), i.e., around 15 KB, to increase initial speed.

### Related to the window size exchanged during the 3-way handshake?

This is a common misconception: the value exchanged during the handshake is `rwnd`, not `cwnd`.

The window field in the TCP header: This is `rwnd` (receive window).

In other words, it means the receiver is saying, 'I have this much buffer space left, so don't send more than this.'

This value exists for flow control, not congestion control.

`cwnd` (congestion window) is a secret variable that is not written anywhere in the packet and exists only in the sender's OS kernel memory.

The sender compares the `rwnd` provided by the receiver with its calculated `cwnd` and sends data based on the smaller of the two values.

### Does increasing by 1 mean bytes?

No, it doesn't. The unit for increasing by 1 is MSS, which stands for Maximum Segment Size.

When discussing the congestion window in TCP, increasing by 1 means the amount of one packet.

MSS (Maximum Segment Size)
- This is the maximum data size that can be purely contained, excluding headers.
- In an Ethernet environment, it's usually 1,460 bytes (MTU 1500 - IP header 20 - TCP header 20).
- A `cwnd` of 1 means that 1460 bytes can be sent at once. Of course, this can be changed.
- If `cwnd` goes from 1 to 2 after receiving an ACK, it means 2940 bytes (double) can be sent.
- Network devices (routers) process and buffer data in packet units, not byte units, so calculating congestion control in terms of packet count is more efficient.

### Congestion Collapse

Also known as congestion collapse, congestion control exists to prevent this collapse.

It is a chain reaction that occurs if the sender, without considering the network state at all, floods data up to the receiver's buffer size (`rwnd`).

Without congestion control, a vicious cycle called a positive feedback loop occurs, causing the network to halt.

1. Unlimited Transmission: All senders, regardless of network conditions, flood packets at maximum speed (up to the limit allowed by the receiver).
2. Bottleneck: Traffic exceeding the processing capacity of intermediate routers accumulates, filling the **router buffer (Queue)**.
3. Packet Drop: Routers indiscriminately discard packets they can no longer receive.
4. Retransmission Storm
   1. The sender does not receive an ACK because the packet is lost.
   2. The sender thinks, "Oh, it didn't go?" and retries.
   3. The network is already congested, and with retransmitted data added to existing data, traffic increases severalfold.
5. Goodput converges to 0
   1. 99% of network bandwidth is filled with retransmitted packets, sending already discarded packets again.
   2. Data that actually reaches the destination and is meaningfully used (goodput) becomes 0.

As a result, the cables are full of data and hot, but users can't detect this, and nothing loads. The network crashes.

Although it was a long time ago, in 1896, the speed of the NSFNET internet backbone reportedly dropped from 34 kbps to 40 bps due to this. About a thousandfold decrease... This happened because there were no congestion control algorithms. To address this, the Slow Start and Congestion Avoidance algorithms for congestion control were developed and, after being incorporated into TCP, the internet recovered.
