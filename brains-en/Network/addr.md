# Mac Address, IP Address

### Mac Address

MAC Address, short for Media Access Control, is a unique identifier assigned to a NIC for communication at Layer 2 (Data Link).

MAC is used as a Layer 2 address in IEEE 802 network technologies, including Ethernet and Wi-Fi. All devices connecting to a network must have a physical address called a MAC address, and they communicate with each other using this address.

Since MAC addresses must be fixed in hardware and unchangeable, an addressing scheme is necessary. Each network component has a different address. Manufacturers are given one or more pools, and they assign MACs themselves within these pools. This addressing is managed by the international organization IEEE.

The MAC address scheme consists of OUI (Organization Unique Identifier)m UAA (Universally Administered Address).
- OUI: The portion assigned by the IEEE manufacturer.
- UAA: The portion assigned by each manufacturer to network components.

Thus, a MAC address consists of the equipment manufacturer's code and a value self-assigned by the manufacturer. Since it is hardware-defined when the card or equipment is produced, a MAC address is also called a BIA (Burned in Address).

<br>

### IP Address

In the OSI 7-layer model, only Layers 2 and 3 have addresses. Layer 2 has the physical MAC address, and Layer 3 has the logical IP address.

Layer 3 addresses in other protocol stacks, including IP addresses, have the following characteristics:
1. A logical address that users can change.
2. Addresses have levels, including a network address that signifies a group, and a host address.

The IP address we commonly use is the 32-bit IPv4 addressing scheme. While both v4 and v6 schemes are primarily used, IPv6 is 128-bit.

When representing an IPv4 address, it is divided into four 8-bit units called octets, and each octet is separated by a dot (.).

Since IP addresses are represented in decimal, an 8-bit octet can use values from 0 to 255.

IP's addressing scheme is divided into a network address and a host address. (This differs in purpose from MAC, which is divided into two parts: manufacturer code and serial number.)
- **Network Address**: An address that refers to a network of hosts. Networks with the same network address are called local networks.
- **Host Address**: An address used to distinguish hosts within a single network.

While MAC is divided into two 24-bit halves, the boundary distinguishing the network address and host address in IP is not fixed. This is a major characteristic that differentiates IP from other addressing schemes.

IP introduced the concept of classes, allowing network sizes to be configured differently based on the number of required hosts.

If the delimiter separating the network address and host address were fixed, like in Classes A, B, C, D, and E, all networks would be the same size because they would have the same number of host IPs. However, since the delimiter can move, network sizes can vary.

In other words, the number of hosts that can be allocated within a network is determined by its class, so the concept of classes was introduced to allow usage tailored to scale.

#### Classful, Classless

The class-based IP addressing scheme described in the IP address section is called classful.

However, currently, **subnet masks** are used.

In the early days, networks were built using classes without subnet masks, and communication occurred without them.

However, after the introduction of subnet masks, classless methods are primarily used, which means transmitting network information along with the subnet mask.

> Subnet Mask: A number used to distinguish between the network portion and the host portion of an IP address. It allows defining and managing the range of a specific network.

This means that you can determine the allocation range of an IP address simply by looking at the subnet mask value, without having to configure the network by specifying a class.

- Classful: Does not transmit the subnet mask when sending routing information. Protocols like RIPv1 and IGRP exist, and there are limitations when using subnet masks in routing protocols.
- Classless: Includes subnet mask information when sending routing information. Representative protocols include RIPv2, EIGRP, OSPF, and BGP.

### Subnetting

Subnetting refers to ignoring the criteria of the originally assigned class and using new network-host demarcation criteria to further subdivide a network beyond its original classful unit.

It's called subnetting because it re-cuts and uses the assigned address, and it's the most significant feature of modern classless networks. It divides networks more finely than octet units, splitting them into 1-bit binary units.

Simply put, subnetting means the process of dividing a large network into smaller networks. (It feels like carving out more space within a class-based network.)

For example, if you want to divide the 192.168.1.0/24 network into 4 subnets, you can do it as follows.

1. Default Subnet Mask: 255.255.255.0 (/24)

2. Add 2 bits to create 4 subnets. (2^2 = 4)

New Subnet Mask: 255.255.255.192 (/26)

**Subnet 1:**
- Network Address: 192.168.1.0
- Subnet Mask: 255.255.255.192
- Available Hosts: 192.168.1.1 ~ 192.168.1.62 (62 hosts)
- Broadcast Address: 192.168.1.63

**Subnet 2:**
- Network Address: 192.168.1.64
- Subnet Mask: 255.255.255.192
- Available Hosts: 192.168.1.65 ~ 192.168.1.126 (62 hosts)
- Broadcast Address: 192.168.1.127

**Subnet 3:**
- Network Address: 192.168.1.128
- Subnet Mask: 255.255.255.192
- Available Hosts: 192.168.1.129 ~ 192.168.1.190 (62 hosts)
- Broadcast Address: 192.168.1.191

**Subnet 4:**
- Network Address: 192.168.1.192
- Subnet Mask: 255.255.255.192
- Available Hosts: 192.168.1.193 ~ 192.168.1.254 (62 hosts)
- Broadcast Address: 192.168.1.255

Originally, 192.168.1.0 would have been the network address, but through subnetting, it's further subdivided, and you can see that the network addresses for the 4 subnets are 0, 64, 128, and 192.

If it were class-based, network addresses could only be set at the class level, but subnetting allows for more granular and finer subdivision.

### Public IP, Private IP

To access the internet, an IP address is required, and it must be globally unique and identifiable. Such an address is called a public IP.

However, if you configure a private network without connecting to the internet, it only needs to be unique within that space. It's possible to build such a network without being assigned a public IP, and this is called a private IP address.
