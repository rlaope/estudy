# Network Web Communication Flow

Let's explore the flow of communication when you enter a URL in the address bar.

Before looking at the web communication flow, there are some concepts you need to understand.

### IP Address
A special number used by devices in a computer network to recognize and communicate with each other  
-> The currently used IPv4 consists of 32-bit numbers like 128.0.0.1.

### Domain Name
An address that represents an IP address in text form  
-> DNS: A database that contains mapping information between IP addresses and domain names

<br>

### How It Works

![](./image/webflow.png)

1. The user enters a domain name in the browser.
2. The DNS server searches for the domain name entered by the user and finds the mapped IP address. It returns this along with the URL information entered by the user.
3. The IP address uses the HTTP protocol to create an HTTP request message.
4. The generated HTTP request message is transmitted via the internet to the computer (server) at that IP address using the TCP protocol.
5. The server approves the client's request and sends a response message.
6. The received HTTP response message is converted into web page data using the HTTP protocol, and the user can view it through the web browser's output.

### Finding the DNS Server Address

### DHCP
- Dynamic Host Configuration Protocol

![](./image/dhcp.png)

- An application layer protocol that automatically provides the host's IP address and TCP/IP settings to the client

The user receives their IP address, the IP address of the nearest router, and the address of the nearest DNS server from the DHCP server.

### ARP
- Address Resolution Protocol
- A protocol used to bind IP addresses to physical network addresses on a network
- Converts the router's IP address obtained via DHCP to a MAC address

<br>

### Receiving IP Information from the DNS Server

![](./image/dnsserver.png)

1. Send a DNS Query (www.example.com) to the DNS server
 : In Korea, there are designated DNS servers for each telecommunications company.
2. The DNS server queries the root name server with the DNS Query.
 : The root name server returns the IP address of .com.
3. Query the .com name server with the DNS query.
 : The .com name server returns the IP address of example.com.
4. Query the example.com name server with the DNS query.
 : It returns the IP address of www.example.com.

 DNS servers form a hierarchical structure, and there are only 13 DNS servers worldwide that handle the top-level domains (.com, .kr, etc.).

 <br>

 ### Accessing the Web Server

Through the above process, we have obtained the IP address of the server we want to access.
1. Open and connect a TCP socket for the HTTP Request.

![](./image/websocket.png)

2. If the TCP connection is successful, the HTTP Request is sent through the TCP socket
3. The web page information is returned as a response
