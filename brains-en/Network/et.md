# Token Ring, Ethernet

### Token Ring

Unlike Ethernet, where a PC can send data freely if no one else is transmitting, in a Token Ring network, **only the PC holding the token can send data**.

Once it finishes sending data, it passes the token to the next PC. This transfer direction is unidirectional. Therefore, collisions do not occur in Token Ring, and it's easy to predict network performance in advance.

However, a disadvantage is that if you have data to send but don't have the token, you must wait for it, even if other PCs have no data to send. This Token Ring method disappeared after Ethernet emerged.

### Ethernet

Ethernet is **one of the methods for building a network**, and most of Korea uses this method.

A characteristic of Ethernet is that it communicates using the **CSMA/CD** method. In this method, a computer that wants to communicate checks the network; if no one is using it, it immediately sends its data and then verifies if it was successfully transmitted.
- CS(Carrier Sense): A network device detects whether the network is in use before transmitting data.
- MA(Multiple Access): Multiple devices can access the network simultaneously. All devices share the same communication channel.
- CD(Collision Detection): If two or more devices transmit data simultaneously during data transmission, a collision occurs. CSMA/CD resolves such collisions by detecting them and attempting retransmission after a certain period.

If two computers try to send their data simultaneously, a collision occurs, which is called a collision. If a collision occurs, the PC waits for a random period and then retransmits the data it intended to send.

This eliminates the situation where a PC has data to transmit but must wait because it doesn't have the token.
