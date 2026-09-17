# Distributed Consensus Algorithms

### Operating Principles and Limitations of Paxos and Raft Algorithms

These two algorithms are representative consensus algorithms designed to withstand node failures (crash faults) and replicate the same state.

#### Paxos

**Paxos** divides roles into Proposer, Acceptor, and Learner. The consensus process is largely divided into two phases.

1.  **Prepare/Promise**: The proposer generates a proposal number and sends it to the acceptors. The acceptors promise not to accept proposals with a higher number.
2.  **Accept/Accepted**: Once a majority of promises are received, the proposer sends the actual data for acceptance. If the acceptors approve it, consensus is complete.

Its limitations include the algorithm itself being very abstract and difficult to understand. Furthermore, the basic Paxos (single-decree Paxos) covered in the paper is insufficient for continuous log replication, necessitating an extension to multi-Paxos. However, the lack of a standardized architecture leads to fragmented implementations across different systems.

Let's explore the Quorum Intersection principle.

$$N \ge 2F + 1$$

The mathematical basis for maintaining data consistency in distributed systems, an application of the 'Pigeonhole Principle', is Quorum Intersection.

**Proof of Situation**: If R is the read quorum and W is the write quorum, to always read the latest data, there must be at least one overlapping node between the nodes that participated in the previous write and the nodes participating in the current read. That is,

$$R + W > N$$

must be satisfied. To maximize availability, it is usually set to a majority: $R = W = \lfloor \frac{N}{2} \rfloor + 1$.

Expanding the formula, if F nodes out of the total N nodes are down, the remaining N - F nodes must form a majority quorum for the system to continue operating without stopping.

$$N - F \ge \lfloor \frac{N}{2} \rfloor + 1$$

Rearranging this, the minimum node condition for consensus is $$N \ge 2F + 1$$

The actual limitation is that while it can tolerate failures of less than a majority, if more than half of the total nodes in the system go down simultaneously or if network partitioning prevents a majority from forming, Paxos-based systems will refuse all write operations (i.e., sacrifice availability) rather than writing incorrect data.

#### FLP Impossibility Theorem and Livelock

The most important theoretical limitation for understanding Paxos is the FLP impossibility, proven in 1985.

-   In an asynchronous network environment, a deterministic consensus algorithm that simultaneously satisfies 100% stability and liveness does not exist, even if only one node goes down.
-   **Stability**: All nodes agree on the same value (never two different agreements).
-   **Liveness**: The system does not halt and eventually terminates consensus.
-   In real-world scenarios, Paxos chooses stability for integrity and sacrifices liveness. This results in a mathematical dilemma called livelock:
    -   Proposer A receives a majority for phase 1 with proposal number 1.
    -   Before A can request phase 2 acceptance, Proposer B updates the phase 1 promise with proposal number 2 from a majority.
    -   A's phase 2 request is rejected. An annoyed A makes a phase 1 request again with proposal number 3, invalidating B.
    -   B's phase 2 is rejected.

As shown above, if timing is exquisitely misaligned, even without any node failures, consensus may never complete, leading to an infinite loop that consumes resources. This is a mathematical and logical limitation of the pure Paxos algorithm.

If you're curious why Zookeeper still employs this, I'll continue writing in the next post.

#### Raft

**Raft** was created to address the difficulty of understanding and implementing Paxos, and it clearly divides node states into Leader, Follower, and Candidate.

All client requests are received only by the leader. The leader then converts these requests into logs, replicates them to followers (log replication), and commits the data once a majority responds.

Due to its strong leader-based structure, in environments with high write traffic, the network bandwidth or processing capability of the leader node can become a bottleneck for the entire cluster.

### Leader Election and Split-Brain Prevention Techniques in etcd and Zookeeper Implementations

These two systems are widely used for distributed configuration management and service discovery, employing the Raft and ZAB (Zookeeper Atomic Broadcast) protocols, respectively.

#### Leader Election Techniques

-   **etcd (Raft protocol-based)**: If the heartbeat periodically sent by the leader stops, followers transition to candidate status and begin voting. To prevent votes from being scattered when multiple candidates emerge simultaneously, **randomized election timeout** is used. Each node waits for a random time between 150 and 300ms, making it highly probable that the node that wakes up first will be elected as leader.
-   **Zookeeper (ZAB protocol-based)**: It uses the **Fast Leader Election** algorithm. Nodes exchange their `zxid` (transaction ID), which represents the latest state of their data. By default, the node with the highest `zxid` (meaning the most up-to-date data) is elected leader. If the latest states are identical, the node with the larger configured node ID (`myid`) becomes the leader.

#### Split-Brain Prevention

Split-brain occurs when a network partition splits a cluster into two or more parts, and each part elects its own leader, leading to divergent data.

Both systems primarily prevent this through the majority quorum rule.

-   Let's assume a 5-node cluster where the network splits into groups of 3 and 2 nodes.
-   The group with 3 nodes exceeds the overall majority (n / 2 + 1), so it can normally elect a leader and record data.
-   The group with 2 nodes cannot form a majority, so it cannot elect a new leader. Even if the existing leader is part of this group, it cannot commit data because it won't receive a majority response for write requests.
-   Additionally, Raft's Term and Zookeeper's Epoch numbers are used to immediately invalidate the authority of a leader from an old term/epoch when network connectivity is restored.

### BFT (Byzantine Fault Toleration) in Large-Scale Clusters

Beyond simple node crashes, BFT is an algorithm that defends against Byzantine faults, where nodes are hacked or maliciously spread false information.

**Limitations of BFT and Difficulties in Large-Scale Application**: Since participating nodes must cross-verify each other, communication overhead increases exponentially as the number of nodes grows (typically proportional to the square of the number of nodes). Therefore, it is difficult to directly apply traditional PBFT (Practical BFT) to large-scale clusters with hundreds or thousands of nodes.

#### Byzantine Fault Tolerant BFT Operation and Mathematical Limitations

BFT algorithms, such as PBFT, provide a consensus structure that defends against malicious behavior (referred to here as Byzantine faults), where nodes are hacked and send false information, rather than simply going down.

To achieve this, all nodes not only verify messages sent by the leader but also **cross-verify what messages other ordinary nodes have received.**

The fault tolerance condition is

$$N \ge 3F + 1$$

This is the mathematical limit for the total number of nodes N and the allowable number of malicious nodes F for a BFT system to reach consensus normally.

This means that if more than 1/3 of the total nodes are malicious, **consensus is impossible**. The proof is as follows:

Assume a worst-case scenario where F nodes out of N total nodes do not respond due to network latency. Since the system cannot wait indefinitely, it must make a majority decision with a minimum of N - F responses. But what if all F malicious nodes are included in the N - F nodes that responded and are lying? To achieve correct consensus by majority vote (adopting the opinion of honest nodes), **the number of purely honest nodes must always be greater than the number of malicious nodes.** This prevents false statements from becoming accepted as truth when there are more people making false statements than true ones.

Number of honest nodes: $(N - F) - F = N - 2F$
Number of malicious nodes: $F$

$$N - 2F > F$$

$$N > 3F$$

The actual limitation is that to defend against F = 1, a minimum of 4 nodes are required. If the total number of malicious nodes exceeds 1/3, the system's stability breaks, risking two different data points being agreed upon simultaneously.

#### Limitations of Message Communication Complexity

$$O(N^2)$$

One of the biggest performance limitations of BFT is the exhaustion of network bandwidth.

-   **Mathematical Limit**: For cross-verification, each node must broadcast the messages it receives to all other nodes. In PBFT, O(n^2) messages are generated in both the prepare and commit phases, and the total number of messages transmitted over the network explodes quadratically as the number of nodes increases.
-   **Actual Limit**: While 10 nodes might result in a few hundred messages, 1,000 nodes would generate millions of messages for a single data storage operation. This makes it impossible to directly apply traditional BFT to public blockchains with over 10,000 nodes due to physical network bandwidth limitations.

#### **Application and Solution Cases for Large-Scale Clusters**

1.  **Delegated Validators in Public Blockchains (Tendermint/Cosmos)**: Even if there are tens of thousands of nodes in the entire network, a **small group of validators (e.g., 100-150)** is formed, elected based on stake, etc. The BFT process for consensus is performed only within this validator group to prevent performance degradation, and the remaining ordinary nodes receive the determined consensus, thereby achieving large-scale scalability.
2.  **Permissioned Enterprise Environments (Hyperledger Fabric)**: Used when enterprises or organizations with verified identities form a network. Since it's a multi-tenant environment where complete trust isn't possible, BFT is needed. However, because the number of nodes can be controlled, an optimized BFT-family algorithm is used among the consensus nodes (orderers) to process thousands of transactions per second.
3.  **Aerospace and Military Distributed Control Systems**: This is the most classic example. When multi-sensors and control computers in airplanes or spacecraft experience arbitrary malfunctions (Byzantine faults) due to radiation or physical damage, BFT is used to reach consensus on the state among large-scale sensor networks to ensure the entire flight system maintains the correct trajectory.
