# Distributed Consensus Algorithm Paxos

In distributed systems, ensuring concurrency and consistency is not an easy task because multiple servers can hold different data. To solve these problems, Paxos emerged as an algorithm to achieve consensus among nodes in a distributed environment.

### Introduction

**The Paxos algorithm, proposed by Leslie Lamport, is one of the most widely used algorithms for achieving consensus among nodes in a distributed system.**

Leslie Lamport proposed Paxos through a paper titled 'The Part-Time Parliament'. However, the content of this paper was very abstruse and difficult to understand, so it took some time for the Paxos algorithm to become widely used.

Subsequently, the paper 'Paxos Made Simple' explained its core ideas and operation in an accessible way, leading to its widespread adoption and making it one of the essential algorithms for achieving consensus in distributed systems.

<br>

### Paxos

Paxos is one of the algorithms that achieves consensus among nodes in a cluster within a distributed environment.

Before introducing the algorithm, let's first look at the terminology used in the paper.
- **Majority**: Refers to a number exceeding half of the total nodes in a cluster. The Majority is important because consensus is only reached when more than a Majority of nodes agree.
- **Message**: Refers to data exchanged between nodes through communication. A message is data transmitted between nodes.

The following are characteristics of a cluster in a distributed environment that define Paxos.
1. **Nodes within the cluster can fail at any time.**
2. **Nodes within the cluster have three roles: Proposer, accepter, learner.** A single node can take on more than one role.
3. The message processing speed of nodes varies, they can fail at any time, and nodes can be restarted.
4. Each node has stable storage, such as a disk.
5. Messages can take a long time to be delivered, and they can be duplicated or lost. However, it is not assumed that the message itself is altered in an unintended way.

To understand Paxos, it is important to understand the concept of Majority.

Majority refers to a number of nodes exceeding half of the total nodes in a cluster. In Paxos, if more than a Majority of nodes reach a consensus, that value is considered to be agreed upon by all nodes in the cluster. For example, if a cluster has 7 nodes, a Majority means 4 or more nodes. Therefore, consensus is reached only when 4 or more nodes agree. If more than a Majority of nodes reach consensus on a specific value A, that value is considered to be agreed upon by all nodes in the cluster.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FoMJWC%2Fbtr9QlHh2tw%2FG7pt4J6uQolthrgHeacdY0%2Fimg.png)

Furthermore, it's possible for one Majority to be in the process of choosing A, while another Majority is simultaneously in the process of choosing C. In such a case, the final consensus value can be determined through the nodes included in both the Majority that chose A and the Majority that chose C.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FJzUUz%2Fbtr9N3UJFAH%2FkXo1SvupRCZNkmu7MV1JXK%2Fimg.png)

<br>

### Choosing a Value

Paxos is broadly divided into Phase 1 and Phase 2. Phase 1 involves exchanging prepare and promise messages, while Phase 2 involves exchanging accept messages.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FoBVW2%2Fbtr9NoSznXC%2FB9kBRLKyTuDZ8M3ppxjoQ1%2Fimg.png)

**Prepare**: The proposer proposes a specific value to `n` accepter nodes via a prepare request. These requests include the following data:
- Round ID: A unique value not used in previous consensus processes, which can be used as a monotonically increasing value. The Round ID is used by the Accepter to decide which value to choose when it receives multiple prepare and accept messages.
- Server ID: If multiple nodes perform propose operations simultaneously, their Round IDs might be the same. To distinguish them, each server has a unique Server ID. This is used to uniquely identify each node and to differentiate messages during the consensus process.
- Value: Refers to the value the Proposer intends to propose.

An Accepter might simultaneously receive multiple prepare requests. The Accepter selects the prepare request with the highest round ID and returns a promise response to the proposer, indicating its commitment to use that value.

If the Proposer receives promise messages from more than a Majority of acceptors, it sends an accept message, using the proposed value, to the acceptors.

Unless an Acceptor receives a prepare request with a round ID higher than that of the received accept message, it agrees to use the value from the accept message. The consensus process concludes with an accepted message response.

<br>

### Acceptor Failure

Let's examine situations where an acceptor node fails to process a request.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fc46Mcu%2Fbtr9OytsPje%2FPH6DMw39MSwW6ATSWXgCG1%2Fimg.png)

First, let's consider the case where only one acceptor fails. In this scenario, the proposer receives an accepted message from one acceptor and is considered to have received an accepted message from itself. (In reality, it doesn't exchange accepted messages with itself, but since it proposed the value, it can be considered to have accepted that message.) In such a situation, with a cluster of 3 nodes, a Majority, meaning 2 or more nodes, have completed acceptance, so consensus can be reached normally.

However, what if more than a Majority of acceptors fail? In this case, only one proposer node ultimately accepts, which is not a Majority, leading to a failure in reaching consensus.

> Ultimately, if more than a Majority of nodes are operational, the consensus process proceeds normally.

**Let's also check the process when a promise fails.**

The promise failure case is similar to the accepted case we just examined: if a Majority of nodes return a promise response, the consensus process proceeds normally.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FFS3Vm%2Fbtr9V81v4uI%2FCIMqXo9RsPQupAfAYXcd11%2Fimg.png)

<br>

### Proposer Failure

Having looked at the process of acceptor failure, let's now examine the process of proposer failure.

**Promise Failure**

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FNQoYh%2Fbtr9YlzJab0%2FKOwaiHjZfg9QmGZxkO3Tl1%2Fimg.png)

If a proposer fails during the promise phase, one of the operational nodes is elected as a new proposer and continues the consensus process.

**Accept Failure**

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fea29JT%2Fbtr9RvJF3bM%2FmO8pOMf9u0KdQ0ngrk4VJ1%2Fimg.png)

If no acceptor receives an accept message, the consensus process is halted. However, if an accept message is delivered to at least one acceptor, a new proposer can continue the previous consensus process.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbiQHkC%2Fbtr9N4F79uW%2F6QP7PbRKTKDPTU1XGkhK21%2Fimg.png)

In the figure above, it can be seen that the accept message (1,1) burger was received by only one acceptor. When this acceptor receives the next prepare message (2,2) pizza, it sends the previously received (1,1) burger along with it, indicating that a value was already chosen in a previous consensus process. Therefore, in step 5, when sending the promise response message, the value from the accept message received in the previous consensus process is also sent. If an accept message value already exists from a previous consensus process, the new proposer uses that value to continue the consensus process (the round ID and server ID will be those designated by the new proposer).

Besides Paxos, there is also the Raft algorithm, which we will explore next time.
