# Network
- Refers to the foundational infrastructure for accurately transmitting desired information to the intended recipient or device.
- When transmitting information using a network, agreed-upon rules must be followed, which are called protocols.


<br>

## Network Classification by Distance

### LAN (Local Area Network)
- A network that covers a single building or a small area.

### WAN (Wide Area Network)
- A network that connects vast areas such as countries or continents.
- Compared to LANs, the transmission distance is wider, and routing algorithms are required.
- While there are no distance limitations, information travels through various paths, making it slower and more error-prone than LANs.

<br>

## Data Exchange Methods

### Circuit Switching
- Utilizes a physical dedicated line, and once the data transmission path is established, data is only transmitted along that same path.
- Bandwidth, which refers to the amount of data that can be transmitted simultaneously, is fixed, ensuring a stable transmission rate.

### Packet Switching
- Data is sent and received using units called packets.
- This is the primary method used in current computer networks.
- Information is divided into fixed-size segments, and each packet is embedded with sender/receiver addresses and additional information.

<br>

## OSI 7-Layer Model
It is a network layer representation model developed by the International Organization for Standardization (ISO).
Each layer is independently configured and provides functionality to the upper layers by utilizing the functions of the lower layers.
It is defined from Layer 1, the Physical Layer, to Layer 7, the Application Layer.

| Layer Order | Name | Description | Key Devices & Technologies |
|---|---|---|---|
| 1 | Physical Layer | Connection devices for linking physical equipment | Hub, Repeater |
| 2 | Data Link Layer | Reliable data transmission by eliminating errors and controlling flow | Bridge, Switch |
| 3 | Network Layer | Supports selecting the correct path among multiple intermediary systems | Router |
| 4 | Transport Layer | Connection between sender and receiver processes | TCP/IP, UDP |
| 5 | Session Layer | Logical connection between sender and receiver | Host (e.g., PC) |
| 6 | Presentation Layer | Translates codes and characters for consistent transmission, handles compression, decompression, and security functions | Host |
| 7 | Application Layer | Provides user-friendly environment (e.g., email, web) | Host |

<br>

## Key Network Devices

### Hub
- A device used to connect multiple computers to send data over a network or to transmit information received by one network to multiple computers.
- It transmits received frames to all ports except the receiving port.
- Representative types of hubs include dummy hubs and switch hubs.
  - `Dummy Hub` : A hub configured in a star topology that simply connects data.
  - `Switch Hub` : A hub with switching capabilities, an intelligent hub that controls data presence and flow, and is used in most modern hubs.

<br>

### Repeater
- A regenerative relay device that regenerates and retransmits attenuated transmission signals.
- When communicating via connections like hubs, it amplifies attenuated digital signals, ensuring that the signal does not weaken before being received by the computer.

<br>

### Bridge, Switch
- Bridges and switches are networking devices that connect two systems.
- They connect two LANs to create a much larger LAN.

1. Bridge
   - Software-based
   - Low speed
   - Ports transmit at the same speed
   - 2 to 3 ports
   - 1:1 connection based on destination address
   - Uses only the Store and Forwarding method, which processes data after receiving it entirely

2. Switch
   - Hardware-based
   - High speed
   - Ports transmit at different speeds
   - Hundreds or more ports
   - 1:N connection based on destination address
   - Uses both Cut Through, which transmits immediately after checking the destination address, and Fragment Free, which combines the advantages of Cut Through and the bridge's Store and Forwarding method

<br>

### Router
- A router is an interconnection device that connects networks with different structures at the network layer.
- It allows local hosts, such as PCs, to access a LAN, and enables access to a WAN using a WAN interface.
- Routing protocols establish paths to ensure that specified data is securely delivered to the desired destination.

<br>

### Gateway
A device that allows protocols to connect to different communication networks or enables various types of networks to interconnect and exchange information.

<br>

### NIC (Network Interface Card)
A device installed within a computer to connect to an external network and exchange data at the fastest possible speed.
