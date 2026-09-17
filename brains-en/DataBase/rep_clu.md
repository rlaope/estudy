# Replication vs Clustering

### What is Replication?

- **It is a method of building multiple databases in a vertical structure based on their roles.**
- In replication, the Master Node handles only write operations, while Slave Nodes handle only read operations.
- Replication synchronizes data between nodes asynchronously, and the detailed process is as follows.

![](Image/리플리케이션.png)
The image above illustrates MySQL's replication method, and the detailed processing steps are as follows.

1. A write transaction is executed on the Master node.
2. The Master node stores the data and records a log of the transaction in a file (BIN LOG).
3. The Slave node's IO Thread copies the Master node's log file (BIN LOG) to a file (Replay Log).
4. The Slave node's SQL Thread reads the file (Replay Log) line by line and stores the data.

Replication synchronizes data asynchronously without performing data integrity checks between the Master and Slave. Understanding this structure, the replication method has the following advantages and disadvantages.

#### Advantages
- Since approximately 60-80% of DB requests are read operations, replication alone can significantly improve performance.
- Operating asynchronously results in almost no latency.

#### Disadvantages
- Data synchronization between nodes is not guaranteed, which may lead to inconsistent data.
- If the Master node goes down, recovery and handling can be challenging.

<br>

### What is Clustering?
- Clustering is **a method of building multiple databases in a horizontal structure.**
- Clustering is used to build a distributed environment and establish a Fail Over system that can resolve issues like a single point of failure.
- Clustering synchronizes data between nodes synchronously, and the detailed process is as follows.

![](Image/클러스터링.png)

1. A write transaction is executed on one node, and COMMIT is performed.
2. Before writing the content to the actual disk, a request is sent to other nodes for data replication.
3. Other nodes send a signal acknowledging the replication request and begin writing to disk.
4. Upon receiving signals from other nodes, the data is stored on the actual disk.

Clustering synchronizes data synchronously by performing data integrity checks (ensuring data consistency) between databases. Due to this structure, the clustering method has the following advantages and disadvantages.

#### Advantages
- By synchronizing data between nodes, consistent data can always be obtained.
- Even if one node fails, other nodes remain active, allowing the system to continue operating without interruption.

#### Disadvantages
- Since time is required to synchronize data among multiple nodes, performance is lower compared to replication.
- Handling can be complex if a failure propagates, and scaling is limited due to data synchronization.
