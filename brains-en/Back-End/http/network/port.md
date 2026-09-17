## PORT
English meaning: harbor  
Used to distinguish between multiple server connections from a single client.  
TCP/IP packet information includes source and destination ports.  
  
Example: Client 100.100.100.1 Game (8090), Video Call (21000), Web Browser Request (10010)  
Server 200.200.200.2 Game (11220), Video Call (32202), Server 200.200.200.3 Web Browser Request (80)  
  
100.100.100.1 Game (8090) - 200.200.200.2 Game (11220)  
100.100.100.1 Video Call (21000) - 200.200.200.2 Video Call (32202)  
100.100.100.1 Web Browser Request (10010) - 200.200.200.3 Web Browser Request (80)  
  
- 0-65535 assignable
- 0-1023 well-known ports, best not to use


FTP - 20,21  
TELNET - 23  
HTTP - 80  
HTTPS - 443  

<br>

## DNS
1. IP addresses are difficult to remember.
2. IP addresses can change.

**DNS**: Domain Name System  
- Phone book
- Converts domain names to IP addresses

### Internet Network Summary
TCP was introduced to overcome the limitations of IP. UDP is similar to IP but with the addition of ports, and functionality can be extended by applications if needed.  
Ports are used to distinguish applications running within the same IP address. If an IP is an apartment building, a port is an apartment number.  
DNS is used in place of IP addresses, which can change and are difficult to memorize.
