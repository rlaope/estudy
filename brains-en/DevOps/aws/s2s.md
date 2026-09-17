# Site to Site VPN

S2S VPN is a dedicated tunnel-based VPN service that securely and directly connects on-premises networks and AWS VPCs using IPsec. It can securely connect customer IDCs, internal data centers, branch networks, etc., with the AWS cloud as if they were a single network.

### Components

**Customer Gateway (CGW)**: The VPN endpoint on the on-premises side; the device type can be one of the following:
- Physical firewall/router (Cisco, Juniper, Fortigate, Palo Alto, etc.)
- Virtual router
- Software-based VPN device

This device establishes a secure tunnel with AWS using IPSec IKEv1/v2.

**Virtual Private Gateway (VGW) or Transit Gateway (TGW)**

This is the AWS-side VPN endpoint.
- VGW is primarily used for a single VPC target.
- TGW is used for multiple VPCs, multi-region connections, and hub-and-spoke architectures.

**Two IPSec Tunnels**

AWS automatically provides two tunnels for redundancy purposes for the same VPN connection.
- Tunnel 1, Tunnel 2
- The on-premises device sends traffic through one of them and automatically fails over if one goes down.

### Operational Structure

1. IPsec-based Encryption
   1. Phase 1 (IKE Phase 1): Mutual authentication and SA (Security Association) creation.
   2. Phase 2 (IKE Phase 2): Creation of the actual data encryption tunnel.
   3. Data is encrypted and transmitted using the ESP protocol.
2. Routing via BGP (Optional)
   1. Static routing is also possible.
   2. However, most use BGP Peering for automatic route exchange.
   3. BGP offers excellent health check and failover performance.

### Purpose of Use

The purpose is secure communication between on-premises data centers and AWS, and

integration with AWS DBs or AI calls, ERP, and business systems from DBAs, internal servers, and internal systems.

Used as a backup link in case of Direct Connect failure.

Establishing internal routing hubs between AWS for global enterprises, etc.

### Limitations

Bandwidth is limited. While it's approximately 1.25 Gbps per tunnel, it's typically considered to be around 500-700 Mbps in practice. If high performance is needed, Direct Connect (DX) is used in conjunction.

Being internet-based, it traverses physical internet paths, leading to latency fluctuations and potential packet loss.

There's also an MTU issue; due to IPsec encapsulation, the effective MTU is reduced, which may require adjustment. Typically around 1436?

### Configuration Example Flow

1. On-premises firewall has a public IP.
2. Create an S2S VPN in AWS.
3. Automatically:
   1. Tunnel 1, 2 endpoint IPs
   2. IKE settings
   3. Pre-Shared Key
   4. Routing information, etc., are generated.
4. Apply these settings to the on-premises device.
5. Tunnel is up.
6. BGP Peering is established, and routing tables are automatically synchronized.
7. Communication between both networks is possible.

![](https://docs.aws.amazon.com/images/whitepapers/latest/aws-vpc-connectivity-options/images/redundant-aws-site-to-site-vpn-connections.png)

![](https://docs.aws.amazon.com/images/vpn/latest/s2svpn/images/cgw-high-level.png)
