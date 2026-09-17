# DX (Direct Connect)

AWS Direct Connect is a service that directly connects on-premises environments (company, IDC) with AWS via a dedicated line, bypassing the internet.

If S2S VPN is an encrypted tunnel over the internet, DX is a physical dedicated network line that goes all the way to AWS.

### Key Concepts

**Dedicated Line Bypassing the Internet**
- ISPs or carriers connect a dedicated fiber optic line to the AWS Direct Connect location.
- This line connects directly to the AWS network.
- This results in very stable latency and almost no packet loss.

**Accessing AWS Services with VIF (Virtual Interface)**

Direct Connect involves connecting one line and then attaching multiple VIFs (Virtual Interfaces) to it.

- Private VIF -> Connects to a subnet within a VPC
- Public VIF -> Directly connects to public AWS services (S3, DynamoDB, SQS, etc.)
- Transit VIF -> Connects to Transit Gateway, building a multi-VPC hub

| Item          | Site-to-Site VPN | Direct Connect    |
| ----------- | ---------------- | ----------------- |
| Path          | Internet-based   | Dedicated line-based |
| Stability     | Variable         | Very stable       |
| Latency       | Variable         | Consistent and low |
| Bandwidth     | Per tunnel ~1.25Gbps | 1Gbps ~ 100Gbps   |
| Security      | Requires IPsec encryption | Dedicated line is inherently private |
| Cost          | Low (inexpensive) | Very expensive (carrier/line costs) |
| Setup         | Fast             | Slow (requires line activation) |

It's also possible to use both simultaneously. DX can be set as the primary, with S2S VPN as a backup line.

### Architecture

**Direct Connection Location**: Data centers where AWS has a physical presence.
- Seoul: KT Mok-dong, IDC2, LG U+ Pyeongchon, SK Broadband Bundang, etc.
- Tokyo: Equinix, Colocation facilities, etc.

Customers bring a dedicated line to these facilities to connect with AWS.

For L2/L3 configurations, one of the following methods is typically used:
- L2 connection: VLAN Trunk-based
- L3 connection: Routing with BGP Peering

AWS mostly requires the use of BGP.

It is used for purposes such as large-scale traffic services, high-speed connections between on-premises databases and AWS applications, large-scale data migration (TB to PB units), or connecting AWS and corporate internal networks as one large private network.

### Disaster Recovery

Simultaneous configuration of Direct Connect + S2S VPN (Failover)

- DX is primary, and S2S VPN is set as backup.
- Adjust BGP path preference to prioritize DX.
- If DX fails, use the S2S VPN path.

This is how it's configured.

There is also a feature called SiteLink, which is used to create an internal network between multiple DX Locations.

![](https://docs.aws.amazon.com/images/whitepapers/latest/aws-vpc-connectivity-options/images/aws-direct-connect.png)

![](https://docs.aws.amazon.com/images/whitepapers/latest/aws-direct-connect-for-amazon-connect/images/physical-cross-connect.png)
