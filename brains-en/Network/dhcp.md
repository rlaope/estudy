# DHCP(Dynamic Host Configuration Protocol)

The process of assigning IPs is called static allocation when manually entered, and dynamic allocation when automatically received from an external system.

Typically, in internal networks or small-scale networks, IPs are statically allocated, but in wide-ranging networks like those for personal computers, IPs are allocated dynamically. (Of course, some places still use static allocation for security settings, but dynamic allocation's security is being strengthened and improved nowadays.)

And the protocol that facilitates this dynamic allocation is **DHCP (Dynamic Host Configuration Protocol)**.

DHCP is an improved version of BOOTP (Bootstrap Protocol) and is compatible with it.

DHCP operates with two components: a DHCP Client (port 67) and a DHCP Server (port 68).

<br>

### How DHCP Works

I will explain the process of IP allocation via DHCP.

1. `DHCP Discover`: The client sends a broadcast request to find a DHCP Server.
2. `DHCP Offer`: The server that received the request sends the relevant network configuration information (IP, Subnet Mask, Gateway, Lease Time, DHCP ID) from its IP Pool.
3. `DHCP Request`: The client, having received the offer, decides which host to use and responds.
4. `DHCP Ack`: Upon receiving a response from the client, the server records the host information.

DHCP operates in these four stages.

DHCP manages multiple IPs in an IP Pool. One must be careful of **DHCP Starvation Attack**, an attack that intentionally allocates IPs to meaningless systems, causing the IP Pool to become empty and unable to respond, creating a **DHCP Starvation** condition.

The IP allocated by DHCP is usually said to be leased. The lease continues for the LeaseTime, but requesting and re-allocating an IP again requires a broadcast, which incurs significant overhead.

Therefore, the IP lease time is renewed through a renewal process. If 50% of the lease time has passed, a DHCP Request is sent to the server again, and the lease time is rewritten, followed by a DHCP Ack.

<br>

### DHCP Relay

Because DHCP operates via broadcast, it generally only works within the same network.

Therefore, to use DHCP across multiple networks, multiple DHCP servers are deployed, divided by bandwidth.

Here, a DHCP Relay Agent acts as an agent that enables DHCP to be used across different network segments.

![](http://www.iorchard.net/_images/dhcp_relay_1.png)

Unlike standard DHCP, in DHCP relay, Client -> Server requests are unicast, while Server -> Client requests are broadcast.
