# Network Lines, VPN, DWDM

Networks are composed of LAN, MAN, and WAN depending on their scale and management scope.
- LAN (Local Area Network): User's internal network
- MAN (Metro Area Network): A network connecting roughly a city
- WAN (Wide Area Network): A network connecting geographically distant LANs

In the past, the technologies used in LAN, MAN, and WAN were all different, making it easy to distinguish them by protocol or transmission technology. However, with most technologies now integrated into Ethernet, it has become meaningless for users to differentiate transmission technologies.

So, nowadays, LAN, MAN, and WAN are said to be distinguished based on their management scope.

<br>

## Network Lines

Let's learn about internet lines, leased lines, dedicated internet lines, VPN, and DWDM.

### Internet Lines

A line connecting to a telecommunications provider for internet access is called an internet line. **However, simply connecting a cable to a telecommunications provider does not enable internet access; you must use a line connected to the internet service sold by the provider to access the internet.** Common types of internet lines are as follows:
- Fiber Optic LAN (Ethernet): Gigabits ~ 100Mbps
- FTTH: Gigabits ~ 100Mbps
- Coaxial Cable Internet: Hundreds ~ Tens of Mbps
- xDSL (ADSL, VDSL, etc.): Tens ~ Several Mbps

### Leased Lines

Services that guarantee bandwidth between a subscriber and a telecommunications provider are mostly referred to as leased lines.

A dedicated cable connects the subscriber and the telecommunications provider, and within the provider's network, technologies like TDM (Time Division Multiplexing) ensure communication quality as if there were a direct connection.

### Dedicated Internet Lines

A service that guarantees communication bandwidth for an internet connection line is called a dedicated internet line.

The structure is such that the subscriber connects to the telecommunications provider, and this connection then connects to the internet.

> The line for internet connection is dedicatedly connected between the telecommunications provider and the subscriber.

Unlike the connection technologies used by general home users, it guarantees connection quality between the telecommunications provider and the subscriber without competing with other subscribers.

### VPN

An abbreviation for Virtual Private Network, it is a network technology that, while not physically a leased line, creates the effect of a direct virtual connection.

**Carrier VPN**
Leased lines incur higher costs as connection distances increase. While leased lines guarantee available bandwidth, subscribers don't always use 100% of the contracted bandwidth, leading to significant wasted costs. To reduce this waste and lower expenses, telecommunications providers are using VPN technologies that can directly differentiate subscribers, with MPLS VPN being a prime example.

> MPLS VPN allows multiple subscribers to connect to a single MPLS network, but by applying technology that can differentiate subscribers, it can be utilized like a leased line.
> Through this technology, multiple subscribers connect and communicate on a single network, sharing a common line, which lowers costs.

**Subscriber VPN**
If a general user uses a VPN, it is mostly a subscriber VPN, which allows the user to directly configure a virtual private network using the public internet.

This means an internet line can be configured and used like a private network.

### DWDM

DWDM, also known as Dense Wavelength Division Multiplexing transmission technology, was developed to overcome the problems of high cable laying costs and difficult management when communicating over long distances.

Telecommunications providers need to differentiate many subscribers and provide high-bandwidth communication, which requires laying numerous cables, and laying these physical cables presented difficulties.

WDM and DWDM technologies are **technologies that can create multiple channels using different wavelengths of light within a single optical fiber, simultaneously transmitting a large amount of data.**

In other words, multiple communication channels can be transmitted through a single cable using different wavelengths of light.

DWDM transmission technology utilizes more channels than traditional WDM and is now even used in gigabit internet services for general households.
