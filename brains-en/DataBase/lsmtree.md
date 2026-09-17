# LSM Tree

### Log Structured Storage Engine

Document Databases target use cases where data is self-contained within documents, and individual documents have no relationships with each other.

A prime example is the `Log Structured Storage Engine` method, which writes values to a logfile. Logs operate in an append-only fashion, meaning new values are simply added rather than modifying existing ones. Consequently, write throughput is O(1), and read throughput is O(n). Overall, this indicates fast writes and slow reads.

Therefore, to improve the read throughput of a log-structured storage engine, we need to create an index to enhance query performance.

### Hash Index

It uses a hash function to convert the key value of each item into a hash code, which is then used as an index to quickly locate data. The values within each key store the byte offset, which is the actual location where the data is stored on disk. By using a hash index, we can quickly access the byte offset and improve query performance.

### Speed
- **Read Speed**: The read speed of a Hash Index is, on average, O(1) if there's only one element associated with a key. It can retrieve data very quickly because it uses a hash function to convert the key value into a hash code, which then points to the data's location.
- **Write Speed**: The write speed of a Hash Index is, on average, O(1). When adding new data, it can be processed quickly because the data is stored at a location calculated by a new hash function. However, if a **hash collision** occurs in the hash table, additional time may be required, which can increase to O(n) in the worst case.

However, Hash Indexes also have drawbacks.

1.  Because hash values are used as indexes, operations that utilize the original data (e.g., sort, range operations) are difficult.
2.  It can be seen as an index optimized for unique data.
3.  All keys must be stored in memory within the HashMap.

These issues are addressed by the LSM Tree, but before diving into that, let's first understand segments.

<br>

### Segment

When data is stored in an append-only manner in a log-structured storage engine, disk space will eventually run out, leading to significant performance degradation for full scans every time data is read.

Therefore, if data is stored in a logfile of a certain size, it is divided into segments of a specific size. This also involves a compaction process for inactive data and segments.

Here, an active segment refers to the segment where data is currently being written (the latest one). Compaction, when multiple data entries for the same key arrive in an append-only manner, **involves merging or deleting the space occupied by data that is no longer valid or can be replaced by the latest value, leaving only the most recent data.**

<br>

### LSM Tree(SSTable)

Drawbacks of hash indexes include the inability to perform range queries and the requirement to load all indexes into memory. To compensate for these, we use an LSM Tree, which employs SSTable (Sorted String Table) and a memtable.

**Unlike segments, an SSTable has a structure where the keys of the stored hashmap, which contain the byte offset locations, are sorted.** Because of its sorted structure, it supports range queries, and once written to disk, an SSTable is immutable.

![](https://velog.velcdn.com/images/salgu1998/post/721a61d9-8e90-4c47-acc9-1c17f4ba5a36/image.png)

The memtable in an LSM Tree is structured as a balanced binary tree and is sorted. When it grows to a certain size, it flushes to an SSTable.

All write operations are performed through the memtable. Since the memtable resides in memory, database write operations can be processed very quickly because there's no disk I/O!

But what if the memory crashes while data is in the memtable and before it's flushed to an SSTable? Would all the data be lost?

No, that's not the case. When writing to the memtable, data is also simultaneously recorded in a regular logfile, not just the SSTable. At this point, it's not sorted like an SSTable. In the event of a crash, data is recovered using this logfile. The logfile in an LSM Tree is used solely for data backup purposes.

Furthermore, SSTables are structured in a segmented fashion. If there are too many separate SSTables, it leads to more disk I/O. Therefore, once they reach a certain number, individual SSTables are merged for management. Because they are sorted, their merge performance is excellent.

Consequently, an LSM Tree offers:
-   Fast writes. Only the memtable and logfile need to be updated.
-   Sparse Index
    -   Memtable doesn't need to hold all key values, leading to efficient space utilization.
    -   However, if a key is not present, finding a given record will take longer.
-   SSTables are sorted, making them easy to merge during compaction.

However, its drawbacks include:
-   Overhead due to **SSD write amplification** during compaction.
-   A dilemma arises: **should compaction be minimized to reduce SSD write amplification, or should it be performed frequently for read optimization?**
