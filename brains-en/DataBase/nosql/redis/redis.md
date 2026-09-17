# Redis


### What is a Cache?

A cache stores results of previously requested or future requests and serves them quickly.

It can be used in web services, RDBMS, and CPUs also have caches like L1, L2, L3.

Even physical hard drives have caches. As such, a cache is a mechanism designed to ensure fast read and write speeds.

A cache generally refers to accessing a memory-resident buffer rather than disk access.

Memory I/O speed is faster than disk I/O speed.

However, increasing memory size has limitations compared to disk.

But if a capacity that a single server cannot handle is needed, cache servers can be clustered to distribute the load.


<br>

## Redis

Redis is a NoSQL cache system that stores and retrieves all data in memory.

In other words, it's an in-memory database. It also provides additional features like persistence and various data structures.

While Redis's biggest feature might seem to be its speed gained by storing data in memory,

the key difference between Redis and other in-memory DBs is its support for various data structures.

![](https://miro.medium.com/max/700/1*tMiZs3RCrmxLGiFZgWRP6g.png)

Supporting various data structures as shown above offers the advantage of improved development convenience and reduced complexity.

Furthermore, when there's a situation where data needs to be sorted, if a DBMS is used, the process of storing data in the DB, sorting the stored data, and then reading it again takes more time because it requires direct disk access.

However, Redis, an in-memory DB, can sort data faster and more simply through its self-provided data structure called ZSet (Sorted Set).

> Redis's lookup speed is O(1).


![](https://miro.medium.com/max/700/1*zArWVI0y5u_WVj0gktm92Q.png)

<br>

### Key Features of Redis


- An in-memory data store that supports persistence.
- Supports server-side replication for increased read performance.
- Supports client-side sharding for increased write performance.
- A proven technology used in various services.
- Supports various data types such as strings, lists, hashes, sets, and sorted sets, and despite being a memory store, it supports many data types, enabling the implementation of diverse functionalities.

Defined, **"Redis is a high-performance key-value store that supports string, list, hash, set, and sorted set data types, making it a NoSQL database."**


<br>

### Redis Persistence

Data stored in memory is volatile, so it is deleted when the system shuts down.

To ensure data persistence, Redis can store data on Disk using the following methods:

1. RDB (Snapshotting) method | A method of transferring snapshots of data in memory to Disk.
2. AOF (Append On File) method | Records all Redis Write/Update operations themselves into a log file.

Due to issues with snapshot loading time and data recovery scope depending on the snapshot frequency in the RDB method, the AOF method is more commonly used.
