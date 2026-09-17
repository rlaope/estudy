# Switches and Routers, Switching and Routing

## Switches and Switching

### Switch
A switch is a device that performs switching.

When data destined for a particular location passes through a switch, the switch identifies the data's destination and switches it to the appropriate path.

Since a switch operates at the data link layer, it functions based on MAC addresses.

It performs switching by using a MAC table that records MAC address information for the ports connected to it.

A switch performs its role by repeatedly executing the following five functions:

- `Learning`: The process of recording a newly encountered MAC address in its MAC table.
- `Flooding`: If the destination of incoming data is unknown, it broadcasts the data to all ports except the incoming port.
- `Forwarding`: If the destination of incoming data is known, it sends the data to that destination.
- `Filtering`: It blocks all unnecessary ports except for the port leading to the destination path.
- `Aging`: It deletes MAC addresses that have not been used for a certain period.

## Routers and Routing

### Router

A router is a device that performs routing.

Similar to a switch, it checks the information of incoming data and performs a `routing` function, which **finds the appropriate path to the destination** based on the information it possesses.

A router operates based on IP addresses and belongs to the network layer. As mentioned, all the information it possesses is in the routing table, which includes the network protocol type, the destination network range, and the interface through which it will be sent.

It's easy to understand if you think of it as similar to a switch.

## Differences Between Switches and Routers

### Layer Differences

Generally, switches belong to the data link layer, and routers belong to the network layer.
- Switch: MAC address-based
- Router: IP address-based

### Broadcast Domain
Simply put, it can be described as a single network.

A `switch` distinguishes broadcast domains, connecting and separating different network segments per port.

A `router` does not distinguish them.

### Handling Data with Undefined Destinations
- As mentioned, if the destination of incoming data is unclear, a switch broadcasts the data to all ports. This is broadcasting, spreading it widely across the network.
- However, if a router's data destination is unclear, it **ruthlessly discards** that data.

### Learning Function

A switch has a learning function, allowing it to be used without separate configuration by an administrator.

A router requires administrator configuration to create a routing table and enable communication.

<br>

Recently, the functionality of switches has expanded significantly, leading to the emergence of models like L3 switches and L4 switches, which perform roles beyond the data link layer, at higher layers.

So, what is the **difference between a router and an L3 switch, which both perform network layer functions**?

First, there's a physical difference in price. Most routers are more expensive, and it's generally understood that routers offer better performance than L3 switches.

In reality, there are almost no functional differences, and looking at examples of actual deployed networks, they are often used interchangeably. Therefore, it can be said that the more appropriate device is chosen based on the network's deployment conditions and environment.
