# MongoDB Sharding

## Sharding

Sharding is the concept of distributing and storing data.

If all data is stored on a single server, all I/O operations occur on that one server.
However, if multiple servers are used, I/O operations occur across several machines, improving efficiency.

It serves the following purposes:
- Data Distribution: Distributing and storing data sequentially allows handling traffic beyond a single machine, thus achieving load balancing.
- Backup and Recovery Strategy: Pre-distributing and storing data for load balancing protects against risks and enables effective system operation.
- Fast Performance: Multiple independent processes perform tasks concurrently in parallel, ideally ensuring fast processing performance.

<br>

## MongoDB Sharding Components

MongoDB has sharding components.

This makes sharding more efficient than other databases (suitable for scale-out).

### Shard

A distributed data storage space, it is the area where actual data is stored across databases.

### Mongos

As a **route server**, it appropriately distributes tasks to shard servers.

In other words, it routes each requested task to the appropriate shard.

### Config servers

It stores and manages metadata (indexing) for shards.

You can think of it as the index for databases within the shards.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbzyU8S%2FbtrmY4M91aK%2FCEMPXOxQvhVCFjQ1lFk2Sk%2Fimg.png)

### Fast Data Processing

Suppose you need to store numbers 1-9 and want to process them quickly.

In such cases, distributed storage is used.

Distributed storage divides the task of storing 1-9 into segments like 123, 456, and 789, distributing the work. This makes it fast.

123, 456, and 789 are not the same data but different data, which is why it's called distributed storage (not replication).

### How does Mongos (Route Server) Distribute Tasks?

If a client wants to know which database contains data for item 5, how does the route server route the client to where item 5's data is located?

The answer is that information is stored (indexed) in the Config server.

The router needs to know what is stored in the shards. Therefore, the router server indexes this information in the Config server.

Since 5 is on server B, which holds data 456, it routes to server B.

**If a request to find item 5 comes in:**
Mongos (Route Server) -> Config Server (requests location of item 5) -> Mongos (responds to the router server that item 5 exists on B)

> Mongos distributes and moves requested tasks, so it cannot directly access data. Even if it could, the speed would be very slow. Therefore, it stores metadata (data information indexing) in the Config server and makes requests there.
