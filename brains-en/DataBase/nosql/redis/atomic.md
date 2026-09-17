# How to Ensure Atomicity for Multiple Redis Commands

Redis operates in a single-threaded manner, so the processing of a single command is atomic and does not lead to race conditions. However, the story changes when multiple commands are executed at once. To perform functions from different Redis clients, **if multiple commands are sent simultaneously, the commands from each client can interleave, leading to race conditions and unexpected issues.**

To prevent this, there are two main solutions: **Redis Transaction and Lua Script.**

### Redis Transaction

Redis supports transactions. You can use transactions to execute multiple commands atomically.

Redis allows you to execute multiple Redis commands at once. The five main commands used in Redis are listed below:
- `MULTI`: When called, it initiates a Redis command sequence. This command always responds with OK, and from the moment it's called, users can input multiple commands. **Commands entered after `MULTI` is called are not executed immediately but are instead queued.**
- `EXEC`: Executes all commands entered after `MULTI` was called **at once and ends the transaction.** When `EXEC` is called, it returns an array containing the elements returned by all individual commands in the queue, in order.
- `DISCARD`: If `DISCARD` is called instead of `EXEC`, **all queued commands are flushed (discarded) and the transaction ends.**
- `WATCH`: This command is used to implement optimistic locking. I will explain it in more detail below.
- `UNWATCH`: Used to cancel `WATCH` on keys that were previously watched.

> All commands executed after the `MULTI` command will respond with a `QUEUED` status.

**What Redis Transactions Guarantee?**
1. **It guarantees that Redis commands sent by other clients will not be executed while a Redis transaction is running.** This means that requests from other clients cannot intervene between Redis transaction commands. This, in turn, guarantees that the transaction is executed as a serialized, isolated set of commands.
2. If the `EXEC` command is not executed, none of the transaction's commands will be executed. If, for example, the connection is lost due to a network issue before `EXEC` is called during a transaction, no operations will be performed.

**When an Error Occurs in a Transaction**
1. An issue might occur before executing `EXEC`, i.e., before queuing the commands. For example, a command might be syntactically incorrect, or there might be a severe issue like out of memory.
2. An issue might occur after executing `EXEC`. For example, executing an incorrect command (like performing a list-only command on a string).

(As of version 2.6.5 and later) If an error occurs during a transaction, specifically while commands are being queued, the transaction will not be executed at the `EXEC` stage.

```kotlin
127.0.0.1:6379> MULTI
OK
127.0.0.1:6379(TX)> SET a
(error) ERR wrong number of arguments for 'set' command
127.0.0.1:6379(TX)> SET b 1
QUEUED
127.0.0.1:6379(TX)> SET c
(error) ERR wrong number of arguments for 'set' command
127.0.0.1:6379(TX)> SET d 1
QUEUED
127.0.0.1:6379(TX)> EXEC
(error) EXECABORT Transaction discarded because of previous errors.
127.0.0.1:6379> GET b
(nil)
```

<br>

### Optimistic Locking using CAS Algorithm

Let's first understand the CAS (Compare-And-Set or Swap || Check-And-Set) algorithm.

The CAS algorithm can achieve synchronization between threads in a multi-threaded environment without using locks, through the following three steps:

1. **Retrieve Data**: Retrieve the data to be modified.
2. **Compare Data**: Compare the target data with the expected data. (Versioning)
3. **Update Data**: If the data matches the expected data, update it with the new value; otherwise, do not change the data.

In Redis, you can implement optimistic locking using the CAS algorithm within a transaction by using the `WATCH` command.

Redis monitors changes to keys specified by the `WATCH` command. What if one or more watched keys are detected to have changed before `EXEC`? The transaction is aborted, and `EXEC` returns a null response, indicating that the transaction failed.

WATCH can be called multiple times, and watched keys can be removed from change detection using the `UNWATCH` command.

> Before version 6.0.9, there were cases where a transaction would be aborted if a watched key expired, but this behavior was changed in version 6.0.9 and later.

<br>

### Redis Does Not Support Rollback

Redis does not support rollback due to performance concerns. Once a transaction is executed using the `EXEC` command, its commands cannot be stopped or reverted while they are running or failing. **It might be thought that if an error occurs during a transaction, and the queued commands are not executed and the transaction is canceled, this constitutes a rollback.**

Traditional RDBMS, depending on the transaction isolation level, might allow other transactions to read data changed by an uncommitted transaction, for example, in READ UNCOMMITTED. This means that actual queries like insert and update are executed and reflected in the database. In such a situation, if an error occurs during the transaction, a complex rollback mechanism reverts the database to its state before the commands were executed.

In contrast, Redis transactions do not execute commands until `EXEC` is run. Therefore, when an error occurs during a transaction, the fact that its contents are not reflected is **closer to an execution refusal or cancellation rather than a rollback.** This can be seen as different from a rollback mechanism that reverts to a state prior to execution.

<br>

### Scripting with Lua

The Redis server can execute a scripting language called Lua. As far as I know, Lua is widely used in game development and modding, but in Redis, it's also used to execute multiple Redis commands atomically.

In Redis, Lua scripts are used to achieve the following three objectives:
1. **Locality**: Lua scripts are executed on the Redis server, not on the client. When a client retrieves data, performs operations, and then reflects them back to the server, multiple communications occur. In contrast, using Lua scripts means operations are executed where the data is stored (on the Redis server), making it more efficient.
2. **Atomicity**: When a Lua script is executed in Redis, all server activity is blocked, making it impossible for other commands to intervene until the script's execution is complete.
3. **Composition of simple capabilities**: You can create and execute simple functionalities that Redis does not natively support directly as scripts.

This method, without much complexity, ensures atomicity by **blocking Redis server activity when executing commands written in a Lua script.** Lua is also used in other ways, but let's explore those later.

<br>

### Redis Pipelining Does Not Guarantee Atomicity

Redis has a feature called pipelining, which seems conceptually similar to transactions. It allows multiple commands to be sent to the Redis server in a batch, enabling all responses to be received at once, thereby reducing RTT. Assuming 1000 requests need to be made, without pipelining, 1000 network requests occur, but with pipelining, commands can be executed with just 1 network request. This optimizes RTT. The same applies to transactions and Lua scripts.

While it can be used more elegantly with tools like Spring Data Redis, in practice, it's a simple method of bundling multiple commands and sending them to the Redis server, as shown below.. lol

Unlike Lua script usage, this does not guarantee atomicity. Pipelining should be seen as having a nature similar to simple Bulk or Batch operations, rather than transactions.

```bash
$ (printf "PING\r\nPING\r\nPING\r\n"; sleep 1) | nc localhost 6379
```
