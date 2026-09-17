# TCP Session Maintenance Keepalive

**TCP Keepalive ensures that an established session between two endpoints is utilized whenever communication occurs, thereby maintaining the session.**

> If a 3-way handshake were performed for every subsequent request after the initial one, resource consumption would be severe. Therefore, keepalive maintains the session to increase efficiency.

The keepalive application and usage mechanism works by exchanging small packets to confirm keepalive status between each other, after which the timer resets to its original value and the count proceeds.

**As described above, keepalive checks are periodically performed between the two endpoints, ensuring that both sessions remain unbroken.**

```bash
$ netstat -napo | grep -i est
(No info could be read for "-p": geteuid()=1000 but you should be root.)
Active Internet connections (servers and established) 
tcp 0 0 172.31.35.50:22 61.**.2*.7*:51079 ESTABLISHED - keepalive (3684.64/0/0) # Approximately 3684 seconds remaining on the current timer
```


<br>

### Checking Keepalive Timer

```bash
# redis-cli  
127.0.0.1:6379> config set tcp-keepalive 100  
OK  

# redis-cli  
127.0.0.1:6379> config get tcp-keepalive  
1) "tcp-keepalive"  
2) "100"  

$ netstat -napo | grep -i 6379 | grep -i est  
tcp 0 0 172.31.37.40:6379 172.31.35.50:51368 ESTABLISHED 5263/redis-server 0 keepalive (100.00/0/0)
```


Keepalive packets can be observed approximately 100 seconds after the SYN packet is sent.
```bash
tcpdump -i any -A -vvv -nn port 6379 -w tcp_keepalive.pcap
```

TCP keepalive packets are very small, around `68bytes`. Therefore, they don't require many resources to maintain sessions between endpoints, so it's generally better to communicate with keepalive enabled.

<br>

### TCP Keepalive Paramters

```bash
$ sysctl -a | grep -i keepalive  
net.ipv4.tcp_keepalive_intvl = 75  
net.ipv4.tcp_keepalive_probes = 9  
net.ipv4.tcp_keepalive_time = 7200  
  
# Change settings to match the main text  
$ sysctl -w net.ipv4.tcp_keepalive_time="240"  
net.ipv4.tcp_keepalive_time = 240  
$ sysctl -w net.ipv4.tcp_keepalive_intvl="30"  
net.ipv4.tcp_keepalive_intvl = 30  
$ sysctl -w net.ipv4.tcp_keepalive_probes="3"  
net.ipv4.tcp_keepalive_probes = 3
```

- `net.ipv4.tcp_keepalive.time`: **The duration for which a keepalive socket is maintained.** The timer operates based on this time, and after it elapses, a keepalive probe packet is sent.
- `net.ipv4.tcp_keepalive.probes`: **The maximum number of keepalive packets to send.** Network packets can be lost due to various reasons, and there's a retransmission mechanism for this. However, they cannot be sent indefinitely, so this parameter defines the maximum number of retransmissions.
- `net.ipv4.tcp_keepalive_intvl`: **The interval for sending keepalive retransmission packets (ACKs).** After the initially set `tcp_keepalive_time` has passed, a keepalive probe packet is sent. If there's no response to this packet, this value sets how many seconds later a retransmission packet will be sent.

To terminate a connection between two endpoints, a FIN packet is required. It's normal for both sides to exchange FIN packets to properly close the connection, but in system operations, connections often break without FIN packets being exchanged due to various issues.

For example, if a switch connected to a server fails and the connection between the two endpoints is broken, there's no way to deliver a FIN, so the connection remains as if it's still active. However, if the TCP Keepalive option is used, after a certain period, a keepalive probe packet is sent. If there's no response, the kernel determines the session is broken and cleans up the socket.

<br>

### Keepalive Zombie Connections

When MySQL is terminated after intentionally configuring iptables to drop packets, the client's socket state remains ESTABLISHED. However, if communication were normal, the socket state would change to CLOSE_WAIT upon terminating `mysqld`. It remains in a CLOSE_WAIT state rather than CLOSE because `close()` was not explicitly called.

Due to the iptables configured on the DB server, the client does not receive a FIN packet from the server and therefore doesn't know if the connection to the DB server has been broken, remaining in an ESTABLISHED state. However, even these zombie connections are eventually terminated because the keepalive timer expires and no response is received for the keepalive probe, leading to the socket being closed.

<br>

### Zombie Connections

For example, in experiences related to MQ servers and load balancers, sessions without packet flow for 120 seconds are removed from the load balancer's session table due to the load balancer's idle timeout setting of 120 seconds. However, since the load balancer **does not notify the two endpoints that its session table has been cleared, problems arise when the session table is cleared.**

With the load balancer's session table information (client IP, port, timeout, session ID) cleared, the client sends a request to the load balancer. The load balancer distributes requests to the two servers behind it using a round-robin method. If, by sheer luck, the client connects to the server it was originally connected to, there's no issue. However, if it connects to a different server and attempts to continue the request, the newly connected server, from its perspective, sees a client trying to write data without an established connection, so it refuses, and the client experiences a timeout. If such situations repeat, a problem arises where a large number of zombie connections remain on the server side.

To solve this problem, keepalive-related parameters are modified to avoid hitting the idle timeout. **The load balancer's idle timeout was 120 seconds. To ensure that packets flow between the two endpoints within 120 seconds under any circumstances, `net.ipv4.tcp_keepalive_time` is set to 60 seconds, `net.ipv4.tcp_keepalive_probes` to 3, and `net.ipv4.tcp_keepalive_intvl` to 10 seconds.** This configuration allows for sufficient checks within 120 seconds, even accounting for packet loss, thereby preventing the issue of zombie connections on the server side.

- DSR (Direct Server Return): A structure in a load balancer environment where the server's response packets are delivered directly to the client without passing through the load balancer.
- Inline structure: A structure where both requests to the server and response packets from the server pass through the load balancer.

By using TCP Keepalive settings, zombie connections that remain uncleaned because they didn't receive a FIN packet can be eliminated.

<br>

### TCP Keepalive vs HTTP Keepalive

While TCP Keepalive aims to maintain a connection between two endpoints, HTTP Keepalive's purpose is to maintain the connection for as long as possible. The main text demonstrates through two tests that even if TCP Keepalive is configured, if HTTP Keepalive is also configured, it operates according to its own settings.

<br>
