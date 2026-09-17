# WAL, Distributed System Data Consistency, and Disaster Recovery Patterns

Let's explore the core mechanisms of replication protocols based on WAL (Write-Ahead Log).

### WAL

A Write-Ahead Log is a file that records what operations will be performed on a database to disk *before* applying the actual values to the database.

#### Durability

Databases face a dilemma between performance (speed) and safety (preservation). WAL is used to achieve both.

Storing data directly to disk as it's processed is too slow due to random I/O.

Therefore, data is usually processed in RAM first, but if power is lost, all in-memory data is lost.

Furthermore, in a distributed system, if a leader fails after processing a data write but before synchronizing it with replicas,

how would that write be remembered? If it's recorded in the WAL, then that can be used.

In other words, before writing, you **quickly append 'I'm going to write this' to the end of the WAL file**. If the computer shuts down or data is lost, this WAL can be re-read and replayed to restore the memory state.

#### Performance (Converting Random Writes to Sequential Writes)

Finding and modifying actual storage structures like B-Trees in a database involves random I/O, which is slow because the disk head has to move back and forth.

In contrast, WAL appends to the end of a file, making it sequential I/O and incredibly fast.

This means it's a strategy to increase response speed by recording quickly first, and then performing the actual storage slowly later.

### Flow

1.  Client request: Operation to add 100 to 'a'.
2.  WAL record: Append `set a.score = score + 100` to the end of the disk log file. This must succeed for the operation to truly succeed.
3.  Memory modification: Modify user 'a' data in RAM.
4.  ACK: Send success response to the client.
5.  Flush: Write accumulated data from memory to the actual data file (happens later).

In distributed systems, mechanisms like log matching and high watermarks synchronize leaders and replicas, and the log used here is WAL.

When a leader receives a request, it writes the WAL locally, and replicas copy and transmit it identically to followers. The core idea of state machine replication is the belief that if all nodes consistently have the same sequence of WAL entries, the resulting data will also be identical.

<br>

## Data Consistency and Disaster Recovery Patterns

### Time and Order

In distributed environments, physical time (NTP) cannot be fully trusted, so logical order is crucial.

#### Logical Clocks & Lamport Clocks

System times between nodes can differ subtly, making it impossible to definitively establish the temporal order of events using physical time.

It's a counter that defines only the causal relationship of events (happened before relationship).

1.  Each node has a local counter.
2.  When an event occurs, the counter increments by 1.
3.  When sending a message, include its own counter.
4.  The receiving node updates its counter to Max(its own counter, received counter) + 1.

To go beyond the limitations of Lamport clocks, which only indicate order, and also understand concurrency, vector clocks must be used, but for now, just be aware of it.

### Consensus and Replication

This is a method for defining the criteria for success when copying data to multiple nodes.

#### Quorum

In distributed systems, it's the minimum number of nodes that must acknowledge an operation to ensure the validity of read/write operations.

The core formula is $R + W > N$, where N = total number of replicas, W = minimum number of responses considered a successful write, and R = minimum number of nodes to query for a read.

It's designed so that the write set and read set always overlap in at least one node, ensuring that the latest data can always be read.

#### High Watermark hwm

Within a log file, it refers to the **last offset for which replication has been completed to all followers (or a quorum) and the commit is confirmed**.

It ensures visibility, meaning clients can only read data up to the hwm (preventing queries of unstable data still being replicated).

It also serves as a recovery baseline. When a leader dies, it's the baseline for deciding how much data to recover and how much to discard.

LEO (Log End Offset): The last end point of the currently recorded log is always LEO >= HW.

<br>

### Leader Election, Uncommitted Data Handling

Let's examine how consistency is maintained when a new leader is elected after the previous leader fails.

What if the old leader received client request A1, wrote it to the WAL, but died before fully propagating it to followers (before hwm update)?

The process begins with leader election: the node with the most up-to-date log (highest epoch/term + LEO) among the living nodes is elected as the leader. This is to minimize data loss.

After that, the epoch is incremented. The new leader increments this number to signal that it is the leader of a new generation, ensuring that followers ignore the old leader even if it comes back to life.

Log matching and truncation are performed, and during this process, the fate of uncommitted data is decided.

-   **Case A (Preventing Dirty Reads)**: If a follower has a log that is longer than the new leader's log (but uncommitted), that follower truncates and discards logs after the point that matches the new leader.
-   **Case B (Data Recovery)**: If the new leader has logs that a follower does not, the follower fetches and fills in the leader's logs.

After that, the high watermark is updated. Once truncation and synchronization are complete, and the new leader receives confirmation that its current log has been propagated to a quorum or more, only then does it advance the hwm. From this point, clients can read data normally.

Due to this process, distributed systems must be designed with the premise that uncommitted data can be lost.

<br>

### Data Guarantee Semantics

#### Consistency Models

There are various **consistency models**, and we'll look at them one by one.

-   **Strong Consistency**: Always returns the latest data based on the hwm, regardless of which node is accessed. It is slower and requires a quorum.
-   **Eventually Consistency**: Data may temporarily differ, but all nodes will eventually converge to the same state over time. It is faster.
-   **Linearizability**: The highest level of consistency, ensuring the system behaves as if there is only a single copy of the data.

#### Idempotency

When a client sends a request and a timeout occurs, it's unknown whether the server processed it but couldn't respond, or didn't process it at all. Clients can retry to prevent system state corruption.

The problem here is that if the call is not idempotent, duplicate data might accumulate or the state might become even more corrupted. Therefore, it must operate idempotently, meaning that no matter how many times it's attempted, the result is the same as if it were attempted only once.

Implementation patterns include using a unique key or Request ID. The client generates a UUID when making a request and sends it. The server checks if it has already processed this ID. If so, it returns immediately; otherwise, it retries.

Since exactly-once delivery is impossible in distributed systems, combining at-least-once delivery with idempotency effectively achieves an exactly-once effect.

<br>

### Summary

1.  Use **logical clocks** to establish event order and distinguish generations (epochs).
2.  The leader verifies quorum satisfaction and advances the high watermark.
3.  In case of failure, the new leader performs log truncation based on the high watermark and epoch to ensure consistency.
4.  Network **duplication/retry** issues during all these processes are resolved using idempotency keys.
