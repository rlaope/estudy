The nmap command is a tool used in network mapping and is one of the most popular ethical hacking tools.

Nmap is used to discover surrounding networks, and network administrators can make the most useful use of nmap as they always need to map their networks.

Nmap allows you to quickly find specific network devices and is useful for finding services running on systems, including all web services and DNS servers. It helps find devices with open ports, and you can also examine devices to enhance security.

<br>

### Nmap Features

- OS Detection: You can perform an nmap OS scan to detect the OS, OS Version, and other details.
- Service Detection: Multiple service probes in the Nmap-services-probe-file are used to obtain responses from network services and their applications.
- Host Discovery: Network hosts use TCP and UDP protocols to collect data about other hosts on the network.
- Target Specification: Using the target specification feature, you can specify the target IP address you want nmap to scan.
- IPv6 Support: IPv6 stands for Internet Protocol version 6 and can be used in nmap for network scanning.
- NSE Feature: NSE is an acronym for Nmap Scripting Engine and provides Nmap features that can be used for host discovery, network scanning, and targeting.
- TLS/SSL Scanning: With the help of Nmap, you can quickly analyze TLS deployment issues.

<br>

### Obtaining Gateway Address

```bash
route -n get default  
route -n get default | grep 'gateway' | awk '{print $2}'
```

<br>

### Nmap Usage Examples

Various nmap options can be found in the [official nmap documentation](https://nmap.org/book/man.html).

```bash
nmap -sP xxx.xxx.xxx.xxx  
- Indicates that the target host is alive.  
  
namp -sP -PT80 xxx.xxx.xxx.xxx  
- Scans a specific port (80).  
  
nmap -sT xxx.xxx.xxx.xxx  
- Scans all open ports on the target host, not just a specified port.  
- Can identify open ports on the target host, but it's risky as it leaves logs.  
  
nmap -sS xxx.xxx.xxx.xxx  
- Evades detection with a stealth scan.  
  
nmap -sU localhost  
- This is a UDP port scan. It can take a long time.  
  
nmap -sS -O xxx.xxx.xxx.xxx  
- The -O option can be used to detect the operating system.  
  
nmap -v xxx.xxx.xxx.xxx  
- Shows more detailed information.  
  
nmap xxx.xxx.xxx.xxx/xx  
- Scans the entire xxx.xxx.xxx.xxx/xx network.  
  
nmap xxx.xxx.xxx.xxx-xxx  
- Scans multiple consecutive hosts.  
  
nmap -O xxx.xxx.xxx.xxx  
- Shows port scans and the operating system of the host.  
  
namp -sR -p 1-40000 xxx.xxx.xxx.xxx  
- Finds and displays RPC ports from 1 to 40000 on the host.  
  
nmap -sU -PT xxx.xxx.xxx.xxx/xx  
- Sends TCP ACK packets to hosts on the network, waits for a response, and displays open UDP ports.
```
