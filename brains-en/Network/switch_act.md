# Switch in Action

A switch operates based on MAC addresses, which are Layer 2 addresses.

It acts as a network intermediary, receiving packets in the middle of the network and forwarding them only to their necessary destinations.

Even without any configuration, a switch can perform its basic operation of **MAC-based packet forwarding** when connected to a network.

In addition, it can perform the following operations:
- VLAN: Logical network separation on a single device
- STP: Network loop prevention
- Various other security features and monitoring

### Switch Action

In Ethernet networks without switches, packet transmission involved contention, leading to significant network performance degradation.

**A switch is a device that eliminates such contention, allowing packets to be communicated simultaneously to multiple devices without interference.**

Using a switch allows multiple terminals to communicate at once, resolving issues of waiting to communicate or delays due to collisions, and improving communication efficiency.

**The core role of a switch is to identify who is at what location and accurately transmit packets to known locations.**

A switch maintains a MAC address table, which maps MAC addresses to interface information.

The switch inspects the Layer 2 destination address within the packet header, consults its MAC table to determine which port the address is on, and then forwards the packet only to that port.

If a packet arrives with a destination not found in the table, the switch transmits the packet to all ports.

The operations of a switch, which vary based on the table state, can be broadly categorized into three types:

1. Flooding
2. Address Learning
3. Fowarding/Filtering

### Flooding

Upon booting, a switch has no network-related information. At this point, the switch cannot act as a network communication intermediary and behaves like a hub. That is, it forwards packets to all ports except the one they arrived on, just like a hub. This mode of operation, where a switch floods all packets like a hub, is called **flooding**.

Furthermore, in the absence of information (when the packet's destination MAC address is not in the table), it floods to all ports. Since a switch operates in a LAN, it performs this action assuming that a device might exist somewhere, even if it doesn't have the information itself. (Aha!)

While this flooding behavior is a normal operation of a switch, if it occurs too frequently, the switch fails to perform its intended role ㅠ. When a packet arrives, the switch inspects the MAC address in the packet information, learns it, builds a MAC address table, and then transmits packets using this table.

#### Abnormal Flooding

In Ethernet-TCP/IP networks, ARP broadcasts are exchanged beforehand, and then data is transmitted. Therefore, when actually sending and receiving data, the switch does not flood packets.

Using a switch helps with security compared to a hub that floods all packets, as it forwards packets only to necessary destinations, making it difficult to maliciously intercept surrounding communications.

- Attack techniques are used to disable switch functionality and monitor surrounding communications, such as the methods below:
  - Teaching the switch incorrect MAC addresses
  - Filling the switch's MAC table to induce flooding

Therefore, if a switch floods packets without any reason, it should raise suspicion that the switch is not operating normally or that an attack is being carried out nearby.

Additionally, ARP poisoning techniques are sometimes used to deceive devices into believing that the IP and MAC addresses to be monitored belong to the attacker, thereby receiving desired communications.

<br>

### Address Learning

For a switch to properly perform its function of checking a packet's destination MAC address and forwarding it to the desired port, it must build and maintain a MAC address table.

This process of building and maintaining a MAC address table is called Address Learning.

When a packet arrives on a specific port, the switch uses the packet's source MAC address information to record that source MAC address and port number in its table.

1. **MAC Address Recording**: Address learning occurs via the source MAC address, so it can learn destination MAC addresses used in broadcast and multicast. This is because their MAC addresses are only used in the destination MAC address field.
2. **Pre-defined MAC Address Table**: In addition to learning the MAC addresses of neighboring devices through MAC address learning, a switch can also have pre-defined MAC address information.
   
The addresses recorded (pre-defined) here are not for packet processing but are used for communication between switches.
   
Addresses processed internally by the switch are not sent out to a specific port; instead, since the switch processes them itself, they are indicated by terms referring to the CPU, management module, or lack of adjacent port information.
   
To view the MAC address table on a switch, the `show mac address-table` command is used.

<br>

### Forwarding, Filtering

When a packet arrives, the switch checks its MAC address. If it's in its table, it forwards the packet to the corresponding port and **filters** it so that the packet is not sent to other ports.

Through such forwarding and filtering, the switch ensures that packets are delivered only to their intended destinations.

Forwarding and filtering operations can be performed simultaneously on multiple ports of a switch.

Since communication does not affect other ports, they can operate independently from existing communication tasks.

Switches generally perform **forwarding/filtering only for unicast** traffic.
> Broadcast, unknown unicast, and multicast, collectively referred to as BUN traffic, behave somewhat differently (flooding).
> Both broadcast and multicast traffic are flooded without filtering. Unknown unicast traffic is also flooded because its MAC address is not in the MAC address table.

Switches rarely flood unicast traffic.

Before creating a packet, an ARP broadcast is sent to determine the MAC address of the destination terminal. This MAC address is learned and stored in the table in advance. When actual unicast communication begins, packets are forwarded and filtered using the already established MAC address table.₩

ARP and MAC tables are not cleared for a certain period, which is called the aging time.

Generally, `mac table aging time > arp aging time`, which allows for efficient operation of an Ethernet network without flooding.
