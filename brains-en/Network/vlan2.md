# VLAN

VLAN, or Virtual Local Area Network, is a technology that allows **multiple networks to be divided and used** on a single physical switch.

A single switch can be divided into multiple VLANs, and each VLAN operates like a separate switch.

VLAN is a technology that **logically divides and configures** a LAN, regardless of its physical arrangement.

Recently, network segmentation has become more important because many devices, such as phones, multifunction printers, and smartphones, connect to networks in addition to computers.

Reasons for virtualization also include performance degradation of devices due to excessive broadcasts, blocking for security enhancement, and applying policies based on service characteristics.

Using VLANs allows for network separation regardless of physical configuration, and devices on different physical floors can be grouped into a single VLAN.

Networks can be separated by department on the same floor, or by service or device type, such as PCs, IP phones, or wireless devices.

Communication between these separated devices occurs via Layer 3 devices.

> Dividing into VLANs logically partitions a single device to have different networks, so not only unicast but also broadcast communication between VLANs is impossible.
> If communication between VLANs is needed, a Layer 3 device is required for communication between different networks, as learned previously.

### Types

VLANs are categorized into port-based and MAC address-based VLANs.

**Port-based VLANs** were introduced when switches were expensive and responsible for connecting multiple hubs. At that time, the purpose of using VLANs was to divide switches and use them across multiple networks.

VLANs whose purpose is to logically divide and use switches in this way are called port-based VLANs, and most VLANs generally referred to are port-based.

Regardless of which device connects, if a VLAN is assigned to a specific port on the switch, the device will belong to that assigned VLAN.

ex. If Port 1 is VLAN 10 and Port 2 is VLAN 20, a device connecting to Port 1 belongs to VLAN 10, and to Port 2 belongs to VLAN 20.

**MAC-based VLANs** emerged as users moved their workstations more frequently.

Instead of mapping VLANs to fixed ports on the switch, they are assigned to the MAC address of the device connected to the switch.

When a device connects, the switch recognizes its MAC address and changes that port to the designated VLAN.

Since VLAN information can change depending on the device, it is also called Dynamic VLAN.

ex. If MAC AA is assigned VLAN 10, then no matter which port this device connects to, it will be in VLAN 10. Conversely, even if different devices connect to the same port, the VLAN can vary depending on the device.

<br>

## Trunk/Access

Configuring VLANs on switch ports to separate networks allows for more efficient equipment utilization than physically separating switches.

If multiple VLANs exist and switches need to be interconnected, communication between each VLAN would require connecting as many ports as there are VLANs.

**For example, if a switch has 3 VLANs, 3 ports would be needed for inter-VLAN communication.**

In medium to large networks that use many VLANs, connecting separate ports for each VLAN would waste many ports just for inter-device connections.

To solve this problem, the **VLAN tagging feature** was introduced.

The tagging feature allows multiple VLANs to be transmitted together over a single port.

These ports are called **Tagged Ports** or **Trunk Ports**.

Tagged ports, which need to transmit multiple VLANs simultaneously, insert a **VLAN ID field in the middle of the Ethernet frame** during communication and use this information.

When sending packets through a tagged port, a VLAN ID is attached. The receiving side removes the VLAN ID and can then send the packet to the VLAN corresponding to that ID.

As a result, thanks to tagged ports, the ports previously needed for each VLAN can be bundled into one, solving the port wastage problem.

**With the introduction of tagged port functionality, changes also occurred in the Mac Address Table used by switches for packet transmission.**

Another field was added to the MAC table to specify the VLAN, preventing communication between different VLANs.

In other words, when networks are separated using VLANs on a single switch, it operates as if a separate MAC address table exists for each VLAN.

A regular port is called an untagged port or an access port, while a port that allows multiple VLANs to communicate at once is called a tagged port (trunk port).

- tagged port: Used to transmit multiple networks over a single physical port.
- untagged port: Used only when belonging to a single VLAN.

Generally, tagged ports are used for connections between switches where multiple networks are configured simultaneously, while servers belonging to a single network are configured as untagged.

If a packet arrives on an untagged port, it is transmitted only to the same VLAN. If a packet arrives on a tagged port, the tag is stripped, and the packet is transmitted to the tagged VLAN.

**Even ports connected to servers, not just inter-switch connections, may need to communicate with multiple VLANs** when virtualization servers like VMware or ESXi are connected.

In this case, even if the port on the switch is connected to a server, it should be configured as tagged, not untagged.

The virtualization server's interface must also be configured as tagged. Since a virtual switch exists within the virtualization server, it's easier to think of it as an inter-switch connection.

### Communication

Unicast, broadcast, and multicast packets cannot cross VLAN boundaries.

Different VLANs mean different networks and IP addresses, thus requiring an L3 device.
