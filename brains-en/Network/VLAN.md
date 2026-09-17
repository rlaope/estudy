# Understanding VLANs - VLAN, Trunk, Access Port

When a network device performs a broadcast operation, the broadcast sends a request to all connected network devices.

Here, broadcasting is a method of sending a message simultaneously to all connected networks. When the request reaches the appropriate network, a response is received.

One thing to consider here is that **as the number of networking devices increases, the amount of broadcast traffic also increases. This will cause the network bandwidth to be saturated with broadcasts, and data transfer speeds will naturally slow down significantly**. Furthermore, computers constantly processing incoming broadcasts won't be able to handle other tasks.

Also, what if you want to use multiple IP ranges with different subnet masks? Should you use a router for each IP range? That would be too costly.

The feature that emerged to comprehensively solve these problems is **VLAN (Virtual LAN)**.

<br>

## VLAN
VLAN is a feature that helps create broadcast domains (simply put, to separate areas).

Network devices (switches) that support VLANs can create multiple VLANs.

Through these VLANs, broadcast domains can be divided. VLANs can be distinguished using numbers from 1 to 4096, and these are called **VLAN IDs**.

Simply put, VLANs can be defined as creating multiple logical LANs where broadcasts do not affect each other.

### Example
ARP can be seen as a representative protocol that uses broadcasting.

Let's assume VLAN 10 and VLAN 20 are assigned to 10.10.10.x/24 and 10.10.20.x/24, respectively. (Since virtual interfaces are created on VLANs and IPs are assigned for L2 communication, rather than physical interfaces, logical interfaces, Switched Virtual Interfaces (SVIs), are used.)

If an ARP Request is sent LAN-wide from 10.10.10.x/24 (i.e., a broadcast is sent), the broadcast will only be delivered to VLAN 10 PCs connected to the same VLAN.

The switch determines whether to forward traffic based on the VLAN configured on each port. For example, if port e2 on the switch connected to PC1 has VLAN 10 configured, it can only forward broadcasts to ports configured with the same VLAN 10.

This aligns with the primary purpose of VLANs: broadcast domain segmentation.

<br>

However, a problem can arise here.

What if an enormous number of VLANs are used? Let's assume there are a total of 10 VLANs between a router and a switch: VLAN 10, 20, 30, ..., 100. If so many VLANs are used, how should VLANs be configured between the router and the switch? Should a port be added between the router and the switch and a VLAN added every time an additional VLAN is needed? We'd first need to check if the router even has that many available ports...

Adding a port and assigning a VLAN every time a VLAN is added is highly inefficient, and since VLAN IDs range from 1 to 4096, theoretically, 4096 ports would be needed.

Not only do switches not have that many ports, but assigning separate ports is also very inefficient.

There's a solution to these problems: **Trunk (Tagging)**.

<br>

## Trunk (Tagging)

A trunk is a **link** that allows multiple VLANs to pass through a single port when frames are transmitted between switches.

The ports on both switches where this trunk setting is applied are called **Trunk Ports**. (They are also called Tagged Ports.)

As an aside, which we'll discuss further below, a regular port where Trunk is not applied, and only a VLAN is configured, is called an **Access Port**.

### VLAN Traffic Flow

Communication is possible between VLAN 1 and VLAN 2 on Switch 1, and VLAN 1 and VLAN 2 on Switch 2, all through Trunk Ports configured with Trunk.

This way, multiple VLANs can pass through a single port, saving ports. The traffic flow is as follows:

VLAN 1 End Device -> Switch 1 Access Port -> Switch 1 Trunk Port -> Switch 2 Trunk Port -> Switch 2 Access Port -> VLAN 1 End Device

VLAN 2 End Device -> Switch 1 Access Port -> Switch 1 Trunk Port -> Switch 2 Trunk Port -> Switch 2 Access Port -> VLAN 2 End Device

Now, VLANs can communicate with a router via trunk ports, and inter-VLAN communication is possible through the router.

**This is called Inter-VLAN Routing.**

<br>

## IEEE 802.1Q
IEEE 802.1Q is a network standard that supports VLANs and is one of the trunking protocols.

It's a method for transmitting VLAN ID information on trunk ports between switches. When an Ethernet frame enters a port configured as a trunk, the tag containing the VLAN ID for that frame is **inserted into the Ethernet frame itself**. This is a way of extending the Ethernet frame.

Thus, when the switch on the other side receives this frame, it can identify the VLAN ID by looking at the tag recorded in the Ethernet frame.

<br>

## The Truth About Access Ports

An Access Port has no relation to VLAN Tags (VLAN IDs) whatsoever. VLAN Tags (VLAN IDs) are applied only when an Ethernet frame exits through a trunk port.

In other words, an **access port cannot distinguish which VLAN a frame belongs to, whether it's egressing or ingressing**. The criterion for an access mode port to determine a VLAN is simply the VLAN configured directly on that port.

It doesn't distinguish the VLAN ID of the Ethernet frame; it simply assumes the Ethernet frame belongs to the VLAN ID configured on the port.
