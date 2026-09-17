# BGP(Border Gateway Protocol) Peering

A structure where two network routers exchange routing information (CIDR) with each other, stating 'I have these routes,' and automatically synchronize routing information.

It is critically used in S2S VPN, Direct Connect, large ISPs, and enterprise networks.

In other words, it's a relationship where two routers connect using the BGP protocol (site-to-site session) to automatically exchange routing information.

This connected state is called Peering.

### Why is it necessary?

- Automatically exchange routes: No need for manual entry like static routes.
- Select optimal path when multiple paths exist: Automatically selected based on criteria like AS-PATH, MED, LOCAL_PREF, etc.
- Automatic failover in case of failure: Automatic switching from Tunnel1 to Tunnel2 is possible. (This is why AWS S2S VPN uses BGP for both tunnels.)
- Essential for large-scale networks: Automatic CIDR updates eliminate the need for manual labor.

```
온프레 라우터
   (ASN 65001)
      │
   BGP Peering
      │
AWS VPN Gateway
   (ASN 64512)
```

As shown above, they establish a BGP session with each other, our CIDR is advertised to AWS, and the AWS VPC CIDR is automatically advertised to us.

### Elements created by BGP Peering

- BGP Session (Neighbor)
```
neighbor 169.254.10.1 remote-as 64512
neighbor 169.254.10.1 ebgp-multihop 2
neighbor 169.254.10.1 activate
```

2. Prefix Advertise: Advertise my network range to the peer.
network 10.0.0.0/16

3. Prefix Receive: Reflect the routes advertised by the peer in the routing table.

### BGP Peering in AWS S2S VPN

Each VPN tunnel is provided with a VTI/30 internal IP.

```
우리 라우터: 169.254.10.2/30
AWS 라우터: 169.254.10.1/30
```

These two establish BGP Peering. AWS typically uses ASN 64512. On-premises usually uses the 65000-65535 range.

As a result:

- AWS VPC CIDR is automatically advertised to on-premises.
- On-premises CIDR is advertised to AWS.
- Automatic failover to Tunnel2 in case of Tunnel1 failure.
- Automatic routing table updates.

> VTI, or Virtual Tunnel Interface, is a technology that makes an IPsec tunnel behave like a virtual network interface.
> ```
> eth0   → 물리 NIC
> eth1   → 물리 NIC
> vti0   → 가상 NIC (IPsec 터널)
> ```
> In other words, it's an interface that allows you to assign an IP address to an IPsec tunnel and add routing table entries.
> With VTI, a router can handle it like `route add 10.10.0.0/16 via vti0`, making routing control easier.
> AWS, FortiGate, and Palo Alto all recommend VTI.

| Item | Static Route | BGP Peering |
|---|---|---|
| Route Addition Method | Manual input by human | Automatic advertisement/reception |
| Fault Tolerance | Manual change by human | Automatic Failover |
| Scalability | OK only for small scale | Automatic management of hundreds to thousands of routes |
| AWS Recommendation | X | O (Site-to-Site VPN default option) |

In summary, BGP Peering is a connection where two routers automatically exchange routing information, and it is a core technology used in Site-to-Site VPN for failover, scalability, and automatic routing management.
