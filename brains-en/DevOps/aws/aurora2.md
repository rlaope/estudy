# AWS Aurora 2nd

I once organized notes on Aurora, but I'd forgotten them, so I'm writing this to study it again.

![](https://docs.aws.amazon.com/images/AmazonRDS/latest/AuroraUserGuide/images/aurora_architecture.png?utm_source=chatgpt.com)

Aurora is not a DB Engine but a cloud-native database system with log-based distributed storage.

The biggest difference from RDS is that the storage layer has been completely redesigned to be separate, distributed, and managed.

### Compute <-> Storage Separation

Traditional RDS (MySQL/InnoDB) DB instances have a structure of query processing + buffer pool + logs + data files.
Replicas supported binlog-based replication, and in case of failure, both storage and instances were recovered simultaneously.

However, Aurora DB instances have a structure of SQL parsing / optimizer / execution / buffer cache.
Aurora Storage contains distributed storage (a completely separate system) based on Redo Logs.

In other words, compute is lightweight, while storage is always alive and supports automatic recovery.

![](https://d2908q01vomqb2.cloudfront.net/887309d048beef83ad3eabf2a79a64a389ab1c9f/2023/05/10/DBBLOG-2505-002-1024x507.png?utm_source=chatgpt.com)

- 6-way replication + 3 AZ
  - Data is 3 AZ x 2 copies = 6 copies
  - Always maintained independently of instances.
- Quorum-based Write/Read
  - write quorum: 4 out of 6
  - read quorum: 3 out of 6
  - Some storage nodes can persist immediately in case of node failure.
- No Page Writes, Only Log Writes
  - InnoDB
    - Dirty page flush
    - Double write buffer
    - High fsync cost.
  - Aurora
    - Only Redo Logs are sent over the network.
    - Page assembly is performed in storage.
    - Write path is short and predictable.

```
Client
  ↓
Aurora Writer Instance
  - SQL 실행
  - Redo Log 생성
↓
Aurora Storage
  - 6-way 분산 기록
  - 4개 ACK 도달 시 commit
```

The important points are that there is no data page flush, network I/O is performed only in Redo Log units, and commit latency is stable.

### Aurora Read Path (Read Flow)

If it's in the buffer cache, it's read from instance memory; otherwise, only the necessary pages are fetched over the network from storage using a Redo + Base Page combination.

Replicas read directly from storage and do not require binlog replay. Replica speed is very fast (within minutes).

### Writer / Reader Architecture and Endpoint

![](https://d2908q01vomqb2.cloudfront.net/887309d048beef83ad3eabf2a79a64a389ab1c9f/2018/05/14/Figure-11.jpg?utm_source=chatgpt.com)

The Writer has a single structure and is responsible for transaction commits.

Readers, up to 15, share the same storage. Dedicated for Read Scaling.

Endpoints include Writer Endpoint, Reader Endpoint (load balancing), and Custom Endpoint (workload separation).

### Reasons for Fast Failover

RDS performs replica promotion, re-verification of storage status, and crash recovery.

Aurora's storage is already in a consistent state, so a new instance attaches to storage, minimizing redo replay.

| Category     | Aurora           |
| ------------ | ---------------- |
| Write Latency | Low and Stable   |
| Read Scaling | Horizontal Reader Scaling |
| Replica Lag  | Almost None      |
| Failover     | Fast             |
| Large Data   | Advantageous     |
| OLTP         | Very Strong      |

Compared to single-transaction ultra-low latency, single AZ, local SSD-based MySQL, Aurora is weak in write-heavy + small data environments.
Aurora is a DB optimized for distributed reliability + scalability.

### Quorum

Writer Reader (1, max 15) refers to the number of compute instances.

Above, there was content about 6-way replication / 4 out of 6 write / 3 read quorum. Here, quorum refers to the consensus rules of storage nodes and is a completely different subject.

Quorum is an internal rule for 6-way replicated storage.

```
Storage Node A (AZ1)
Storage Node B (AZ1)
Storage Node C (AZ2)
Storage Node D (AZ2)
Storage Node E (AZ3)
Storage Node F (AZ3)
```

The write quorum of 4 out of 6 means that when the writer instance sends a redo log, the commit succeeds if 4 storage nodes acknowledge it.

The read quorum of 3 out of 6 means that when reading a page / log from storage, reading is possible if only 3 match.

The important thing is that even if there are 15 reader instances, the quorum does not become 15. Quorum is based on storage nodes.

```
Client
  ↓
[Writer Instance]  ← 단 1개
  ↓
[6 Storage Nodes]
  ↳ 4개 ACK → commit

Client
  ↓
[Reader Instance #1]
[Reader Instance #2]
...
[Reader Instance #15]
  ↓
[동일한 6 Storage Nodes]
  ↳ 3개 일치 → read
```

The reason for not having multiple writers is that it would require distributed locks and distributed transactions, leading to write-write conflicts, so this choice was made due to the surge in cost and complexity.
Writes are centralized, and reads are horizontally scaled.

| Misconception              | Reality                 |
| -------------------------- | ----------------------- |
| More Readers mean more quorum | ❌                      |
| Writer = Storage           | ❌                      |
| Replica is a copy          | ❌ (Same storage)       |
| Aurora is a multi-writer DB | ❌ (Single writer by default) |
