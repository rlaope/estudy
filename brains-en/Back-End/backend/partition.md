# Data Partitioning


### Partitioning

We store data in databases. But what if the data grows in size and complexity, straining performance? The solution is to distribute the data.

By distributing data, we can gain the following advantages:

1.  Database request load balancing
2.  Ability to build scalable systems
3.  Ability to utilize closer data centers
4.  Availability


## Partitioning Types

### **Range-based Partitioning**

![](https://substackcdn.com/image/fetch/w_1456,c_limit,f_webp,q_auto:good,fl_progressive:steep/https%3A%2F%2Fbucketeer-e05bbc84-baa3-437e-9518-adb32be77984.s3.amazonaws.com%2Fpublic%2Fimages%2F99382b01-a54b-42e2-8702-513ecb4bebda_1200x628.png)

### **Hash Partitioning**

![](https://substackcdn.com/image/fetch/w_1456,c_limit,f_webp,q_auto:good,fl_progressive:steep/https%3A%2F%2Fbucketeer-e05bbc84-baa3-437e-9518-adb32be77984.s3.amazonaws.com%2Fpublic%2Fimages%2F6a6cc6c8-d39c-49d3-ab1a-b4520f23901a_1200x628.png)


1.  List Partitioning
2.  Key Partitioning
3.  Round-Robin Partitioning


## Rebalancing

When query throughput increases and we need to handle more load by adding CPUs, or when the dataset size grows and we want to add disk and RAM, or when we need additional equipment for fault tolerance to overcome failures, we add more nodes. Also, when nodes are removed, rebalancing occurs, where data is appropriately moved to other nodes to maintain balance.

Regardless of the partitioning method used, the minimum requirements typically expected to be met when rebalancing is performed are as follows:
1.  After rebalancing, the load distribution should be evenly distributed among the nodes.
2.  Reads and writes must be accepted even during rebalancing.
3.  Rebalancing should be performed quickly, and data should not be moved between nodes more than necessary to minimize network and disk I/O load.

To briefly look at methods that should be avoided, performing an **n modulo** operation on hash values is one such example. This method causes results to change as the number of nodes increases, leading to frequent data movement upon node addition or deletion, which makes rebalancing costs excessively high. A method that avoids moving data more than necessary is required.

### Let's fix the number of partitions

This involves creating many more partitions than the number of nodes in advance and assigning partitions to each node.

As a result, when a node is added, it can operate by taking partitions from existing nodes until they are uniformly redistributed.

Of course, this method makes it difficult to find a consensus because the number of partitions is fixed and doesn't change during initial setup.

It becomes even harder to find if there are frequent changes.

### Dynamic Partitioning

Databases like HBase or RethinkDB, which use key-range partitioning, create partitions dynamically. That is, when a partition exceeds a certain size, it is split into two, with each new partition containing roughly half of the data.


## What if a request comes to a non-existent data partition?

### Request Routing

A dataset can be partitioned across multiple nodes running on various machines, but how does a client know which node to connect to when sending a request?

As partitions rebalance, partitions on nodes change. If someone wants to read or write 'foo', which address should they connect to? When partitions rebalance, the partitions assigned to nodes change, so what then?

This is a type of service discovery issue. Any software that connects over a network, especially software aiming for high availability with redundancy, faces this problem even more acutely.

There are various ways to solve this problem.
1.  The client connects to any node, and if the data isn't there, the request is forwarded to the correct node, which sends a response back to the client. (This requires each node to know information about other nodes to forward requests.)
2.  All client requests are sent to a routing layer, which determines the node responsible for handling each request and forwards it.
3.  The client is made aware of the partitioning method and which node each partition is assigned to.

![](https://goodgid.github.io/assets/img/sd/SD-Partitioning-Request-Routing_1.png)

There are limitations: this problem is difficult to handle because information must be consistent across all participating entities; otherwise, requests will be sent to the wrong nodes and not processed correctly.

Additionally, while there are protocols used for achieving consensus in distributed systems, they are tricky to implement.

### Coordination

![](https://goodgid.github.io/assets/img/sd/SD-Partitioning-Request-Routing_2.png)

To overcome these limitations, many distributed data systems use a separate **coordination service** like ZooKeeper to track cluster metadata.

Each node registers itself with ZooKeeper, and ZooKeeper manages reliable information about partitions and nodes.

Thus, when a partition owner changes or a node is added or removed, ZooKeeper notifies the routing layer to keep routing information up-to-date. Other components that rely on this routing layer subscribe to the information in ZooKeeper.

HBase, SolrCloud, and Kafka also use ZooKeeper to track partition assignments.

MongoDB has a similar architecture but relies on its own config servers and uses a daemon called mongos as the routing layer.

Cassandra, Riak, and Redis use a different approach, spreading cluster state changes between nodes using a gossip protocol. They have a full mesh architecture where any node can receive a request, and the receiving node forwards the request to the correct node that owns the partition responsible for handling it.
