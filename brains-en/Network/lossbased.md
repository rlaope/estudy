# Evolution and Standards of Loss-based Algorithms (Reno & CUBIC)

This note analyzes how algorithms that operate on the principle of reducing speed only when packets are lost have evolved, and why CUBIC is the standard in modern high-speed networks.

### Genealogy of Classic Algorithms.

Early TCP congestion control evolved based on how quickly it could recover.

TCP Tahoe's core feature was Slow Start; upon detecting 3 duplicate ACKs indicating packet loss, it would unconditionally reset the congestion window (cwnd) to 1 and re-enter slow start. (Aggressive speed reduction)

TCP Reno introduced Fast Recovery, reducing cwnd to cwnd / 2 instead of 1 upon packet loss, skipping slow start, and immediately entering the congestion avoidance phase, thereby reducing bandwidth waste.

TCP NewReno is an improved version of Partial ACK; Reno's performance degrades sharply if multiple packets are lost within a single window. NewReno recognizes Partial ACKs (acknowledgments for only some received data) and recovers all multiple losses within a window without a timeout.

> Partial ACK and SACK can be confusing, but the act of detecting lost packets is the same. They are distinguished by whether the receiver or the sender handles it.
>
> Partial ACK is a method used in NewReno, which improved upon Reno's ability to recover only one packet per window. If a sender sends packets 1-5 and packets 2 and 4 are lost, the sender retransmits 2. The receiver receives 2 but still lacks 4, so it responds with ACK 4, indicating that up to 3 has been received. The sender, having originally sent up to 5, infers that recovery is not yet complete. An ACK lower than the highest sequence number sent at the start of recovery is called a partial ACK.
>
> As a result, it doesn't end fast recovery and immediately retransmits 4, realizing that 4 was also lost. The limitation is that it's slow, as it fixes one and then confirms.
>
> What's confusing here is SACK. Selective Acknowledgment. The concept of selecting lost packets is the same, but the operation differs. The receiver sends ACK 1 and waits for 2. In the TCP options field, SACK is written as 3-3 5-5. This means that up to 1 has been received, and 3 and 5 are held, but 2 and 4 are missing. So, the sender can specifically retransmit 2 and 4 based on this.

### TCP CUBIC

It is currently the default TCP algorithm for almost all major operating systems, including Linux, Windows, macOS, and Android.

Unlike Reno's simple linear increase (sawtooth), CUBIC adjusts the window size using a cubic function.

The principle is to remember the point $W_{max}$ where the last packet loss occurred and adjust the increase rate as it approaches that point.

- Concave: It increases rapidly before reaching $W_{max}$, then reduces the growth rate as it gets closer to explore stably.
- Plateau: Near $W_{max}$, it increases very slowly, attempting to maintain maximum bandwidth.
- Convex: If there is no packet loss even after exceeding $W_{max}$, it assumes new bandwidth is available and rapidly increases the window.

RTT Independence and Fairness
- Reno's problem: Since cwnd increases in RTT units, connections with short RTTs (e.g., physically close) can unfairly monopolize bandwidth.
- CUBIC's solution: Window increase is determined as a function of elapsed time since the last loss, not RTT. Therefore, it can fairly secure bandwidth even for connections with long RTTs.

### Disadvantages and Solutions

There is bufferbloat, where loss-based algorithms **only stop when packets are dropped**. However, hardware advancements have created a paradoxical problem.

Modern routers/switches have large buffer queues because memory is inexpensive.

The problems are:
1. CUBIC pushes data until packet loss occurs.
2. The buffer is too large, so packets are not dropped but merely accumulate in a huge queue.
3. Consequently, packet loss does not occur, but latency skyrockets.

I will write about next-generation protocols and algorithms that address this issue in the next post.

Of course, solutions could include routers timing out and dropping packets, or TCP senders pacing transmissions at regular intervals instead of bursting (though this would lower throughput), or NIC drivers dynamically adjusting limits.

There's also CoDel, an algorithm that drops packets if they stay in the buffer too long (an AQM Active Queue Management algorithm), and FQ-CoDel, a killer solution for Bufferbloat that combines Fair Queueing + CoDel, which is a standard in Linux, Android, and modern routers.

FQ CoDel divides traffic into lanes per flow, allowing CUBIC to be used. For heavy traffic like YouTube, even if Bufferbloat occurs, it's managed by CoDel. For other, lighter traffic, it's allowed to pass through CUBIC. That's the general idea.
