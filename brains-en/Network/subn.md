# Gateway, Subnet

A Local Network can learn MAC addresses using ARP broadcasts and can communicate directly via these MAC addresses.

In contrast, a Remote Network requires the help of other network devices for communication due to the nature of broadcasts, which cannot be transmitted beyond the network.

This device is called a **Gateway**, and the option to configure gateway information for PCs and network devices is called the default gateway.

The default gateway is performed by a Layer 3 device and plays the role of specifying the appropriate path while connecting to multiple networks.

To determine whether the source and destination networks are communicating within the same LAN or between different networks, the source must first check if the destination is within its own network range. The range information used at this time is the **Subnet Mask**.
- Communication within the same network differs in method and equipment from communication between different networks.
- Local communication can find the destination via an ARP request, but remote communication requires the help of devices capable of communicating externally for broadcasts that cannot cross routers.
- **The subnet mask is used to determine if the source and destination are on the same network.** In other words, it can distinguish and identify the network and host portions of an IP address.

<br>

### Proxy ARP

Proxy ARP is a function that acts on behalf of ARP; for remote communication, it's only possible to communicate by sending an ARP request to find the default gateway and then sending packets towards the default gateway.

However, if proxy ARP is enabled on the default gateway, even a remote destination can send an ARP broadcast locally.
- When an ARP broadcast arrives at a default gateway (router) with proxy ARP enabled, it performs a proxy ARP reply itself.
- In this case, packets are sent towards the default gateway, allowing them to be delivered to the remote path.

The proxy ARP feature is often enabled by default on routers, operating without the user's knowledge.

However, it sometimes operates even when there are network configuration errors or missing essential settings, becoming a **troubleshooting challenge** that makes issues difficult to resolve.
