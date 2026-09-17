# Resolving SNAT Port Exhaustion and DNS Resolution Latency Issues

**SNAT (Source Network Address Translation)** is a technology that converts the private IP address of a source server located in a private subnet to a public IP address (e.g., of a NAT Gateway) to enable communication with the external internet or other VPCs.

An **Ephemeral Port** is a temporary port that the operating system arbitrarily assigns as a source port when a server establishes an outbound connection (acting as a client) to an external destination. In Linux environments, this typically uses the range 32768 ~ 60999, providing approximately 28,000 ports.

**DNS Resolution** is the process of converting a domain name, such as api.github.com, into an actual IP address that can be used for communication. Applications must undergo this process before initiating communication.

### Problem Definition: Timeouts Occur Despite Ample Resources

When thousands of external API calls or database queries occur per second, communication failures, regardless of server resources, are **primarily due to limitations in the OS network stack**.

1.  **SNAT Port Exhaustion**: After an HTTP communication, when a connection is closed, the socket does not get immediately deleted but remains in a `TIME_WAIT` state for about 60 seconds to ensure the stability of the TCP protocol. If external API calls are made and closed with a new connection each time (connection disconnection), all 28,000 ephemeral ports can quickly enter the `TIME_WAIT` state. The OS then has no more ports to allocate, preventing new external communications from even starting, leading to timeout exceptions at the application level.
2.  **DNS Resolution Latency and Drops**: If connections are not reused, a DNS query must be sent every time communication is attempted. Linux's default DNS queries are UDP-based, and during traffic spikes, local DNS resolvers or DNS servers from cloud providers (e.g., AWS Route 53) might hit query limits or experience packet loss. If Linux does not receive a DNS response, it waits for a default of 5 seconds before retrying, causing critical latency.

### Example

**Application-level Connection Pool and Keep-Alive Settings**

The most reliable solution is to reuse pre-opened connections instead of opening and closing ports every time.

```java
import org.apache.http.impl.client.CloseableHttpClient;
import org.apache.http.impl.client.HttpClients;
import org.apache.http.impl.conn.PoolingHttpClientConnectionManager;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;

@Configuration
public class HttpClientConfig {

    @Bean
    public CloseableHttpClient httpClient() {
        // Create connection pool manager
        PoolingHttpClientConnectionManager connectionManager = new PoolingHttpClientConnectionManager();
        
        // Total maximum connections in the pool
        connectionManager.setMaxTotal(500);
        // Maximum connections per single destination (Route/Domain) (key setting to prevent port exhaustion)
        connectionManager.setDefaultMaxPerRoute(100);

        return HttpClients.custom()
                .setConnectionManager(connectionManager)
                // Maintain Keep-Alive for 20 seconds by default, even if the server doesn't specify
                .setKeepAliveStrategy((response, context) -> 20 * 1000) 
                .build();
    }
}
```

**Essential OS Commands for Network and DNS Troubleshooting**

These commands allow for immediate identification of the cause from the server's terminal during a failure.

1.  When SNAT port exhaustion occurs, check the `TIME_WAIT` state. Count the total number of sockets waiting without closing after external communication. If this number approaches the OS's ephemeral port limit (typically around 28,000), port exhaustion is in progress.

```bash
$ ss -tan state time-wait | wc -l
> 28451
```

2.  Command to check the summary of all TCP socket states. This provides an overview of the server's network connection status. Check if the `timewait` item is abnormally high.

```bash
$ ss -s

Total: 29102
TCP:   28900 (estab 120, closed 28550, orphaned 0, timewait 28451)

Transport Total     IP        IPv6
RAW       1         0         1
UDP       15        10        5
TCP       350       300       50
INET      366       310       56
FRAG      0         0         0
```

3.  Check DNS resolution latency. This command verifies the IP address of the target domain and the `Query time` taken to receive a response from the DNS server.

```bash
$ dig api.github.com

; <<>> DiG 9.16.1-Ubuntu <<>> api.github.com
;; global options: +cmd
;; Got answer:
;; ->>HEADER<<- opcode: QUERY, status: NOERROR, id: 54321
;; flags: qr rd ra; QUERY: 1, ANSWER: 1, AUTHORITY: 0, ADDITIONAL: 1

;; ANSWER SECTION:
api.github.com.         60      IN      A       140.82.112.5

;; Query time: 5012 msec  <-- (Note: If it takes more than 5 seconds, DNS delay/timeout is occurring)
;; SERVER: 127.0.0.53#53(127.0.0.53)
;; WHEN: Sun Mar 29 14:10:00 KST 2026
;; MSG SIZE  rcvd: 59
```

4.  Real-time DNS packet sniffing (tracking throttling and loss). This captures all DNS query (UDP port 53) packets going in and out at the OS level. Check if the application is unnecessarily repeating the same DNS query hundreds of times, or if retransmissions occur even when the DNS server does not respond.

```bash
$ sudo tcpdump -i any -n port 53

tcpdump: verbose output suppressed, use -v or -vv for full protocol decode
listening on any, link-type LINUX_SLL (Linux cooked v1), capture size 262144 bytes
14:10:01.102341 IP 10.0.1.5.49152 > 168.126.63.1.53: 1234+ A? api.github.com. (32)
14:10:06.102500 IP 10.0.1.5.49152 > 168.126.63.1.53: 1234+ A? api.github.com. (32) <-- Same query retried after 5 seconds due to no response
14:10:06.115000 IP 168.126.63.1.53 > 10.0.1.5.49152: 1234 1/0/0 A 140.82.112.5 (48) <-- Delayed response arrives
```

<br>

### Trade-off Monitoring Points

After resolving issues by implementing connection pools and keep-alive, the following aspects need to be managed.

*   **Timeout and Retry Optimization**: In case of DNS latency or temporary network outages, immediate infinite retries place a greater load on the target server and network infrastructure. Retry traffic should be distributed over time using Exponential Backoff and Jitter (random delay).
*   **Connection Pool Monitoring**: Setting the connection pool too large can not only waste application memory but also cause other failures due to connection limits on the target DB or API server. Utilize APM tools to monitor the ratio of `Active Connections` to `Idle Connections` to derive an optimal `MaxPerRoute` value.
*   **DNS Caching (JVM Level)**: In Java/Kotlin environments, tuning the `networkaddress.cache.ttl` value to cache resolved IP addresses at the application level for a certain period can significantly reduce the number of queries sent to DNS servers, preventing DNS throttling.
