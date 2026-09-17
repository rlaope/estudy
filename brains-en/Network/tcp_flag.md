# TCP Flag, ECN

![](https://postfiles.pstatic.net/MjAxOTA1MTVfMTkz/MDAxNTU3ODk4MjkyNzMw.ne0R09ZKfbH0hn5w3wvEcf-m469wWfBu8kSAnHdFI2sg.Q75Gl2BL0cBW_Soo2si9xWMaetDowy-Q5yT8OvWLNzMg.JPEG.ak0402/1.jpg?type=w773)

Before the 3-bit flag fields (NS, CWR, ECE) for the ECN mechanism were introduced, the Reserved field (a field pre-reserved for future use) was 6 bits, and the Flag field also had 6 bits: URG, ACK, PSH, RST, SYN, FIN.

With the introduction of the ECN mechanism and RFC 3168, 3 bits from the existing 6-bit Reserved area were utilized as Flag bits, and NS, CWR, ECE were added to the Flag field.

<br>

### Background of the ECN Mechanism

In the era of slow networks, network collapse due to excessive traffic -> Proposal/application of TCP congestion control algorithms -> Packet loss -> To improve TCP congestion control efficiency and prevent packet loss, routers proactively signal network congestion.

- Routers perform congestion control using the RED (Random Early Detection) algorithm.
- RED is an algorithm that enables smooth communication by detecting congestion before a communication burst occurs. Upon detecting congestion, RED prevents buffer overflow by discarding packets. The RED algorithm's procedure is as follows: when a router's queue approaches a threshold set by the administrator, it selects an arbitrary flow and discards its packets, allowing the sender to reduce its transmission rate.
- If packets are lost at an intermediate router, the receiver waits for a certain period and then requests retransmission from the sender.
- Through this process, the receiver's waiting time and the sender's retransmission time difference are used to lower the limit of the congested router queue.
- However, discarding packets is inefficient because it is only necessary to slow down the transmitting computer. To compensate for this and detect delays and slow down traffic without packet loss, ECN emerged.

**Unlike the RED algorithm, which discards arbitrary packets, the ECN algorithm, upon detecting delay, sets the CE (Congestion Experienced) flag in the IP header of an arbitrary packet and forwards it to the receiver.**

<br>

### ECN (Explicit Congestion Notification)

The operating principle of ECN is as follows:
- If congestion occurs at an intermediate router, the router monitors its queue and detects that packets are being stalled.
- The congested router marks the packets passing through it as evidence of congestion and forwards them to the next router or the receiver.
- Finally, the receiver, upon receiving the packet, **checks the congestion mark left on the packet and informs the sender that the packet experienced congestion during its delivery.**

> ECN Operating Principle: "ECN is not a basic supported feature; both the receiver and sender must be capable of using it for it to be utilized."

Since the ECN mechanism is not a natively supported feature, both the sender and receiver must use ECN.

During the 3-way handshake, the sender sets the ECE bit to 1 in the SYN packet sent to the receiver, **indicating that the sender host can currently use ECN.**

The receiver host, upon receiving the SYN packet, checks it. If ECE is supported, it sets the ECE bit to 1 when sending the SYN-ACK packet to the sender host, indicating that the receiver host can also use ECN.

If both the sender and receiver hosts support ECN in this manner, they will signal or receive congestion notifications via the ECN mechanism if congestion is experienced during their communication.

The fields used in the ECN mechanism exist not only in the TCP header but also in the IP header. In the case of TCP, NS, CWR, and ECE are used to establish the ECN mechanism connection. In the IP header, the last two bits within the Type Of Service field are used, allowing for 2 to the power of 2, or 4, possible representations, which will be explained later.

Previously, the ECE field was used in the ECN mechanism during the 3-way handshake when establishing a connection.

**Detailed Operation**
1. First, when an intermediate router that has experienced congestion forwards a packet to the next router, it sets the ECN-CE bit in the IP header.
2. Here, the ECN-CE bit represents the last two bits of the Type Of Service field in the IP header and is transmitted as '11'.
3. Next, the receiver host, upon receiving the packet, verifies that the ECE-CE bit in the IP header is marked '11'. (If '11', congestion occurred)
4. To inform the sender host that congestion occurred during the packet's delivery, the receiver host sets the ECE bit in the TCP header when sending an ACK packet, signaling congestion.
5. The sender, upon receiving this, reduces its window size to adjust the packet transmission rate.
6. Subsequently, segments used by the sender to the receiver are transmitted with the CWR bit set. Here, CWR is used to indicate that congestion has been received.

- `NS`: Nonce Sum, exists to prevent malicious ECN flag settings and to modify incorrect CE flag settings.
- `CWR`: Congestion Window Reduced, indicates that the TCP sender has reduced its transmission rate due to a congestion control algorithm.
- `ECE`: ECE Echo, a TCP receiver's congestion notification to the sender.

<br>

### Other Flags

- `URG`: Urgent data pointed to by this pointer is processed with high priority. (Not widely used nowadays)
- `PSH`: A flag requesting the receiver to deliver this data to the application as quickly as possible. If this flag is 0, it waits until the buffer is full. If this flag is 1, it means there are no more connected segments after this one.
- `RST`: Means to forcibly reset the connection to a peer that is already in an ESTABLISHED state.
- `ACK`: Acknowledgement, a flag indicating that the field contains a value. If this flag is 0, the acknowledgement number field itself is ignored.
- `SYN`: Synchronize, a segment used to synchronize sequence numbers when establishing a connection with a peer.
- `FIN`: Finish, a segment that is a request to terminate the connection with a peer.
