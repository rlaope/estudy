# IPsec

IPsec is a technology that ensures security (confidentiality, integrity, authentication, etc.) at the IP layer.

1. Is the source authentic? (Peer authentication)
2. Has the packet been altered in transit? (Integrity)
3. Can it be prevented from being eavesdropped on? (Confidentiality)
4. Prevention of Man-in-the-Middle (MITM) attacks and replay attacks (Replay protection)

In other words, it's a technology that makes existing IP packets secure.

### IPsec Modes

There are two IPsec modes.

#### Transport Mode

It keeps the IP header as is and encrypts only the payload (ID payload). Primarily used for End-to-End communication (e.g., host <-> host).

#### Tunnel Mode

It encrypts the entire IP packet and adds a new IP header. Primarily used for tunneling between gateways, such as S2S VPNs and DX.

AWS S2S VPN also internally uses IPsec Tunnel Mode.

### Two IPsec Components

**AH (Authentication Header)**
- Provides integrity + authentication
- Confidentiality (encryption provided)
- Doesn't work well in NAT environments, so it's rarely used nowadays.

**ESP (Encapsulating Security Payload)**
- Confidentiality (encryption)
- Integrity
- Authentication
- NAT Traversal possible
- De facto standard for modern IPsec

Almost all AWS / router / firewall devices use only ESP.

### SA (Security Association)

SA = A security policy/key/cryptographic rule applied in one direction
- It's unidirectional (Inbound and Outbound are each required)
- Identified by SPI (Security Parameter Index)
- Includes encryption algorithms, keys, anti-replay windows, etc.

The reason it's important is that when an IPsec packet arrives, the SPI determines which SA rule to use for decryption.

### Overall Flow of IKE (Internet Key Exchange) (Focus on IKEv2)

An IPsec tunnel proceeds in the following order: IKE -> SA negotiation -> key exchange -> ESP data transmission/reception.
1. **IKE_SA_INIT**: After Diffie-Hellman (DH) key exchange, selects encryption algorithms, performs NAT detection (NAT-D), exchanges nonces (random values) from both sides, and then creates an IKE SA. (Secures an encrypted channel for both sides)
2. **IKE_AUTH**: Peer authentication (PSK, certificates, EAP, etc.), ID verification, Child SA creation (SA for ESP). Finalizes two IPsec SAs (in/out).
3. **After IKE: CHILD_SA Creation/Renewal**: Creates additional SAs or rekeys them for the data channel (ESP).

> A DH key is a shared key value, a material that both endpoints use to create a session key (symmetric key). One might think that if this is compromised, the secret key would be stolen. However, it's not as simple as solving an equation; theoretically, it can be found through other means, but the mathematical complexity is so immense that it would take longer than the universe ending, so it's considered secure. It might be good to look into this further.

### NAT Traversal (NAT-T)

Because IPsec cannot directly pass through NAT environments, ESP is encapsulated using UDP 4500.

Afterward, the NAT-D payload is used to check for a NAT environment, and a Keepalive (1-byte packet) maintains the NAT session.

**Actual ESP Packet Structure**

```
Outer IP Header
  ↓
ESP Header (SPI + Sequence Number)
  ↓
IV (Initialization Vector)
  ↓
Encrypted Payload (Original IP header + payload)
  ↓
Padding / Pad Length / Next Header
  ↓
Integrity Check Value (ICV)
```

- SPI -> Identifies which SA to use for decryption
- Sequence Number: Prevents Replay Attacks
- Encrypted Payload: The entire original IP packet
