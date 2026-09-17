# L1, L2, L3, L4, L5, L6, L7 Switches

Let's explore the switches used at each layer from L1 (Layer 1) to L7 (Layer 7) in the OSI 7-layer model published by the International Organization for Standardization (ISO).

Switches are typically categorized by the addresses they handle (MAC, Protocol, Session Protocol), and switches at higher layers (higher numbers) include the functionalities of lower layers.

Let's examine them one by one.

### L1 Switch (Hub, Dummy Hub)
Operating at the lowest layer, Layer 1 (Physical Layer), a hub is referred to as an L1 switch.

A hub transmits all data to every device connected to it.

### L2 Switch (Switching Hub)
L2 switches, the most common type, operate at Layer 2 (Data Link Layer), and typically use switching hubs that have switching capabilities.

Unlike a regular hub that transmits data to all connected devices, a switching hub internally stores the MAC addresses of each device, meaning it reads the MAC address of incoming packets and forwards them to the corresponding device.

L2 is closely related to MAC addresses.

Compared to other methods, it offers advantages such as being inexpensive, high-performing, having a simple structure, and high reliability.

However, it has the disadvantage of performance degradation due to packets (i.e., it cannot perform routing).

### L3 Switch (Router, Home Router)
Layer 3 (Network Layer) typically performs routing functions.

A router acts as a conduit connecting network packets, and its primary function is bandwidth expansion.

So, while an L2 switch only knows MAC addresses, an L3 switch knows both MAC addresses and IP addresses.

In other words, an L3 switch is a device that adds routing capabilities to an L2 switch.

Therefore, it is based on high-performance hardware, and the switch itself is assigned an IP address.

Many home routers also include L3 and Wi-Fi functionalities.

These routers are often called AP devices, but unlike enterprise-grade L3 switches with insane performance and price tags, home routers typically offer reasonable performance and cost.

### L4 Switch
Layer 4 (Transport Layer) is the layer that first receives all incoming requests from outside.

This is where L4 switches play a role, receiving those requests first and distributing them appropriately to internal servers.

In other words, the main role of an L4 switch is to provide **load balancing**, which can group multiple servers to act as a single unit.

It switches based on protocols, prioritizing applications.

Therefore, an L4 switch considers not only IP addresses but also port information.

For example, let's say you're building a web service.

Assume that a single web server with a public IP (125.125.12.5) handles all requests.

As users increase, traffic becomes unmanageable, so you add another server, which has an IP of 125.125.12.6.

Should we then notify users of our web service to also connect to the 125.125.12.6 server?

Absolutely not.

What's needed in this situation is **load balancing**.

There's no need to ask users, nor to manually forward requests to individual servers.

Naturally, security is excellent, to say the least.

L4 switches, with these excellent features, also provide `Active-Active` / `Active-Standby` functionality, where if one server fails and the service is interrupted, another server takes over that service.

However, they do have the disadvantage of complex configuration.

### L7 Switch (L5, L6 Switch)
Typically, L7 switches also include L5 and L6 functionalities, so understanding just L7 is sufficient.

Layer 7 (Application Layer), like Layer 4, also uses switches for load balancing to forward packets returning to the switch to appropriate servers, but there's a difference from L4.

First, L4 switches perform switching based on IP addresses and TCP/UDP port information, which belong to Layers 3-4.

L7 switches, on the other hand, are devices that perform switching by looking at IP addresses, TCP/UDP port information, and even packet content, which belong to Layers 3-7.

Furthermore, while L4 switches use TCP/UDP ports for load balancing (traffic distribution),

L7 switches can perform more sophisticated load balancing that is more advantageous for security by analyzing HTTP URLs, FTP cookie information, and virus patterns at Layer 7 (e.g., Traffic filter, Security, VPN, etc.).

Therefore, L7 switches are sometimes called security switches.

This is because they can defend against DDoS attacks through data analysis and provide functions like infected packet filtering.
