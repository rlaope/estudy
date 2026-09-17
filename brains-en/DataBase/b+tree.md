# B+Tree

Unlike B-Tree, **B+Tree is characterized by storing all records in leaf nodes and indexes in intermediate nodes.** This compensates for the shortcomings of B-Tree in range searches, as all leaf nodes are connected by a linked list.

### Data Storage

Sometimes, writing data once requires multiple disk writes due to the B+Tree structure. This is called write amplification. Due to this characteristic, there is a possibility of data corruption if a DB crash occurs during a write operation.

To solve this problem, most systems permanently pre-write what to write in a WAL Log (also called Redo Log) in the disk area and then process the B+Tree write operation. (It's easy to think of it as the Logfile role in LSM Tree.) This ensures DB durability.

When the root node contains an index and three leaf nodes contain data 1, 2, 3, and 7, inserting data 6 additionally shows that three more B+Tree nodes have been added.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FRZeZ1%2FbtsvktSIOec%2FseUagR1ArCNVCnIUZx2BFK%2Fimg.png)

All leaf nodes are interconnected, leading to better query efficiency compared to B-Tree. (B-Tree continues range searches by traversing multiple nodes via binary search, whereas B+Tree only needs to scan leaf nodes.)

Since all records reside at the leaf level, space utilization is also better than B-Tree.

However, B+Tree is more complex to implement due to the added structure at the leaf level, and like B-Tree, it has overhead caused by write amplification.

<br>

### LSM Tree vs B+Tree

Generally, B+Tree is good for Read performance, and LSM Tree is good for Write performance.

**Data Reading**
- B+Tree queries within a maximum of O(logN) by traversing down the tree (maximum depth of the tree).
- If the data to be queried does not exist in the memtable, LSM Tree must check SSTables, and background operations (compaction) can also affect performance.

**Write Amplification**
- LSM Tree only needs to write to memtable and logfile (+ merge). (Sorting, sequential writing, and merging do not take much time.)
- B+Tree may require multiple Disk Writes (+ WAL Log) (Page splitting).
- Therefore, LSM Tree has relatively lower write amplification than B+Tree.

**Fragmentation**
- LSM Tree periodically updates data through compaction, resulting in good compression. Thus, it creates fewer files on disk than B+Tree.
- B+Tree has some unusable space due to fragmentation.

**Concurrency Control**
- B+Tree is easy to manage because data exists in only one place.
- LSM Tree can have multiple replicas in different segments. In high write throughput scenarios, during the (logging + flush to sstable) process, background compaction must be shared, and if it cannot keep up with the incoming rate, duplicates may occur.
- Therefore, **B+Tree is primarily used for data that prioritizes transactional features.**
