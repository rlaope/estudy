# Router

A router is a device that belongs to Layer 3 of the OSI 7-layer model, and its role is to set paths and deliver packets to other networks.

When a router sends a packet, if it cannot find the destination, it drops the packet.

Because the process of searching for a path to send data to the destination all at once is too difficult for a router, it designates an adjacent router as the destination, and that router then searches for the path again. This is called the **hop-by-hop** method.

Routers manage destinations to be routed in a table, which is called a routing table, and they search for the closest destination using the LPM method. (Longest Prefix Match)

Routers perform the following three functions.

1. Path Determination: After collecting various paths, they send packets along the optimal path.
2. Broadcast Control: If the destination address of an incoming packet is not in the routing table, the packet is discarded.
3. Protocol Conversion: During the packet forwarding process, Layer 2 header information is removed and new header information is created.

### Router Path Configuration

There are three methods for routers to configure paths.

#### 1. Directly Connected

This method naturally obtains information about adjacent networks when an IP address is entered.

#### 2. Static Routing

This is a method where an administrator directly enters the destination IP into the routing table.

### 3. Dynamic Routing

This is a method where routers automatically exchange path information with each other. It is also used for status exchange, indicating whether a router is currently able to receive packets.

<br>

### Broadcast Control

Routers generally do not acquire information via multicast and do not broadcast.

In other words, a router can only send packets if its own information is present in the routing table.

> Networks that are directly connected can send information even without it.

This functionality is called `Broadcast/Multicast Control`.

<br>

### Protocol Conversion

Another role of a router is to connect networks configured with different protocols. However, this method has become less common as most networks currently use Ethernet. Examples include LAN and WAN.

When a packet arrives at a router, it strips off the Layer 2 header information, checks the Layer 3 address, and then creates new Layer 2 header information before sending it out.
