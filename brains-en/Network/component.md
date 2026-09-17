# Network Components

We will explore NICs, cables and connectors, hubs, switches, routers, load balancers, security devices, and more.

### NIC - Network Interface Card

Commonly known as a LAN card, its official component name is NIC, and it's also referred to as NC, Network Interface Controller, etc.

A Network Interface Card is a **hardware device for connecting a computer to a network**.

The main roles of a NIC are as follows:
- **Serialization**: It converts electrical signals into data signals and vice versa. It handles this mutual conversion as electrical signals are transmitted via external cables.
- **MAC Address**: A network interface has a MAC address. If the destination address of a received packet does not match its MAC address, it discards the packet; if it matches, it forwards it for processing within the system.
- **Flow Control**: While packet-based networks allow various communications, they use a single channel, which can lead to new data not being received due to ongoing data processing. To prevent this, it requests the sender to pause communication, a process known as flow control.

<br>

### Cables and Connectors

There are twisted pair cables, coaxial cables, and optical fiber cables. Focusing on cables used for Ethernet, which is the most common, there are three widely used standards:
1. 1,000BASE-T / 10GBASE-T: Ethernet standard using twisted pair cables.
2. 1,000BASE-SX / 10GBASE-SR: Ethernet standard using multi-mode optical fiber, suitable for relatively short distances.
3. 1,000BASE-LX / 10GBASE-LR: Ethernet standard using single-mode optical fiber, suitable for relatively long distances.

To explain the naming convention using 1,000BASE-T, one of the Gigabit Ethernet standards: the leading number 1000 indicates the speed (1000 Mbps), the middle character 'Base' refers to the channel type, meaning a single channel (Broad indicates multiple channels), and the last character 'T' denotes twisted pair cable. In summary, names are structured as "Speed + Channel - Cable Type".

Cables are physically divided into several elements such as the cable body, connectors, and transceivers. The cable body is categorized into twisted pair, coaxial, and optical fiber, and the types of connectors and transceivers vary depending on the cable body type (some forms combine all three).

Among these, twisted pair cables are the most commonly used. TP cables include shielded types like STP/FTP and unshielded UTP cables. When these cables are plugged into the LAN port of a computer or server, a network connection is established.

- Coaxial Cable: Similar to the thick black cable used for connecting TVs, it's expensive and less commonly used now, historically for TV and internet connections. Recently, integrated DAC cables with transceivers are often used for high-speed connections above 10G.
- Optical Fiber: More reliable than copper UTP or coaxial cables and can transmit over longer distances, it's widely used for high-bandwidth, long-distance communication between network devices.
  - Single-mode: Supports long distances, is very thin, and uses laser signals.
  - Multi-mode: Thicker cable that uses LEDs as the light source.

**Connectors** are the ends of cables that connect to network devices or cards. While TP cables use RJ-45 connectors, optical fiber cables have various types of connectors.

**Transceivers** convert various external signals into the computer's internal electrical signals. Although higher standards like GBIC, SFP, and SFP+ have emerged, the entire transceiver is sometimes still referred to as GBIC. Transceivers that can accommodate TP pair cables in addition to optical fiber are called GLC-TE.

<br>

### Hub

A device operating at Layer 1, similar to cables. A hub regenerates electrical signals that weaken over distance and, as the name "HUB" suggests, is used to connect multiple devices.

A hub simply broadcasts incoming signals to all ports, causing all connected devices on the network to contend for bandwidth, which degrades network performance. It can also lead to network failures due to infinite packet loops, so it is rarely used in modern network configurations.

<br>

### Switch

A Layer 2 device that, like a hub, connects multiple devices and mediates communication. Although hubs and switches differ in their internal operation, they share the role of connecting multiple devices and consolidating cables, so the term "hub" is sometimes used generically.

It is also called a switching hub, as it combines the roles of a hub and mediating communication.

While a hub merely regenerates electrical signals and sends them to all ports, a switch can understand MAC addresses and **sends electrical signals only to the port connected to the destination MAC address. (This is the key difference from a hub.)**

<br>

### Router

A device that operates at Layer 3 and converts protocols to enable communication over long distances.

A router controls broadcasts and multicasts to prevent unnecessary packets from being sent to remote locations and discards communication attempts to ambiguous addresses.

It designates paths to ensure packets are sent in the correct direction and forwards packets along the optimal path.

By "ping-ponging" packets and setting a TTL (Time To Live), it can prevent infinite loop issues by determining that a path cannot be found if a packet moves beyond a certain number of hops.

<br>

### Load Balancer

Primarily operates at Layer 4, checking port addresses while also being able to change IP addresses. Web services are where load balancers are most commonly used.

A Layer 7 load balancer for application-related tasks is also called an ADC (Application Delivery Controller), and network devices referred to as L4 switches are also a type of load balancer.

This refers to a device that has multiple ports like a switch but also performs the role of a load balancer.

<br>

### Security Devices (Firewall/IPS)

Unlike most network devices that focus on accurate information delivery, **security devices focus on controlling information and defending against attacks**.

The most well-known security device is generally a firewall. A firewall operates at **Layer 4 of the OSI model, inspecting Layer 3 and 4 information of packets passing through it, and comparing them against policies to either discard or forward the packets**.

<br>

### etc. Modem, Router

The routers used in almost every home and office are devices that combine a Layer 2 switch, a Layer 3 router, Layer 4 NAT, and basic firewall functions.

A modem is a device that converts between short-distance communication technology and long-distance communication technology, as they differ.

Since both the LAN and WAN ports of a router are standard Ethernet, they cannot send data over long distances (more than 100m), thus a modem is needed to convert to a technology capable of long-distance communication.

For gigabit internet, an FTTH modem is usually required. For coaxial cable internet, a cable modem is used, and for telephone lines, ADSL or VDSL modems are used.
