# Circuit Switching and Packet Switching

## Circuit Switching
A dedicated line is pre-allocated between the sender and receiver for data transmission, connecting the two.

Therefore, if the person I want to connect with is already connected to someone else, they are already using a dedicated line with that other party, so I can only connect with them after that connection is terminated.

If a specific circuit is disconnected, the connection must be re-established from scratch.

> Circuit switching suffers from inefficiency due to a lack of immediacy.

## Packet Switching
Packet switching is a method that addresses the shortcomings of circuit switching, which was previously used in telephony, by dividing data into small units called `packets` for transmission.

- Like sending a parcel, each packet contains source and destination information, allowing it to travel to its destination in the most efficient way.
- This allows for efficient data transmission because no specific circuit is allocated as a dedicated line.

> In summary, the Internet Protocol, or IP, represents source and destination information as specific numerical values called IP addresses and transmits data in packet units.
