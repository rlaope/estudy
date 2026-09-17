# P2P

![](image/p2p.jpeg)

- A distributed architecture where a group of computers in a **peer-to-peer relationship** communicate directly with each other **without a central server**
- Peers participating in the network configuration share resources such as processing power, storage space, and bandwidth
- An overlay network that builds a network on the application layer
- Fast internet connection speed, high performance for end-users. The more peers, the more power
- Scalability, reliability (copies, geographical distribution, No single point of failure), low cost
- **Structured P2P**
  - DHT (Distributed Hash Table): key: hash(filename); value: a node (IP address)
  - assign: giving the key to the closest peer, find: finding nodes corresponding to the key
- Key lookup methods
  - If only knowing the address of the immediate successor peer: ask->ask->ask... query O(N), maintenance O(1)
  - If knowing the addresses of all peers: query O(1), maintenance O(N)
  - Chord Lookup: searches by jumping 2^i, resulting in O(logN)
