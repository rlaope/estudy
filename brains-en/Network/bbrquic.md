# Modern Networks and Next-Generation Congestion Control (BBR & QUIC)

Loss-based algorithms, because they rely on packet loss, had a problem where a large number of packets would accumulate in router/switch buffers, not being lost but only increasing latency.

Therefore, this time, the goal is to abandon reliance on packet loss and explore the latest technologies and next-generation protocols that actively model network conditions.

### Paradigm Shift: Loss != Congestion

In the past, if packets were lost, it was always assumed that the network was congested and the router dropped them.

However, modern networks are different.

*   Noise in wireless sections: In Wi-Fi or LTE/5G, packets can be corrupted by radio interference even when there is no congestion. Loss-based algorithms make the mistake of reducing speed even in such cases.
*   Bufferbloat: Router buffers are too large, so even when congested, packet loss does not occur, and only latency increases. Loss-based algorithms fail to detect congestion in such cases and continue to push data.

In conclusion, it's no longer possible to determine congestion solely by packet loss; the conclusion is to directly measure bandwidth and latency.

### Google BBR (Bottleneck Bandwidth and Round-trip propagation time)

It is a model-based congestion control algorithm developed by Google.

BBR assumes the network is a pipeline through which water flows and tries to find two key variables.
1.  BtlBw (Bottleneck Bandwidth): The narrowest section of the pipe (maximum transmission speed)
2.  RTprop (Round-Trip Propagation): The length of the pipe (minimum physical delay)
3.  BDP (Bandwidth-Delay Product): The ideal amount of data that fills the pipe without overflowing = $BtlBw * RTprop$

#### Kleinrock's Optimal Operation Point

BBR's goal is to achieve maximum speed while keeping the router buffer (queue) empty.

Probing (Exploration Process)
*   ProbeBW: Periodically sends more data. If the speed increases, bandwidth is available; if only latency increases, it's full.
*   ProbeRTT: Periodically reduces data significantly to empty the queue and measure pure communication delay.

#### Efficiency in Long Fat Networks

In networks with high bandwidth and long distances, like international submarine cables, CUBIC drastically reduces the window upon a single packet loss, taking a long time to recover speed.

Even if packet loss occurs, BBR considers it just noise and assumes sufficient bandwidth, maintaining high throughput without reducing speed.

BBR vs CUBIC

BBR's problem is that while it tries not to occupy buffers, it has a tendency to ignore packet loss and push data. CUBIC, on the other hand, retreats immediately upon loss.

As a result, when BBR and CUBIC are mixed on the same network, BBR tends to monopolize the bandwidth, pushing CUBIC into a corner. (To improve this, BBRv2 considers packet loss to some extent.)

### QUIC Congestion Control http3

To address the difficulty of modifying TCP, which is embedded in the OS kernel, QUIC, a protocol running over UDP, emerged.

While changing TCP's congestion control algorithm required updating Windows and Linux OS kernels worldwide, QUIC allowed new algorithms to be deployed instantly just by updating Chrome browsers and YouTube apps. This made it very easy for companies like Facebook and Google to experiment with proprietary algorithms for their services.

It has a pluggable congestion control architecture, allowing for interchangeable algorithms. For example, BBR can be used in Wi-Fi environments where packet loss is frequent, and CUBIC in corporate wired networks. All these transitions can be configured at the application level.

More accurate RTT measurement is possible. With TCP, when an ACK for a retransmitted packet arrived, it was ambiguous whether it was a response to the original packet or the retransmitted one.

QUIC assigns a unique packet number to every packet, and this number increments upon retransmission, allowing for accurate RTT calculation.
