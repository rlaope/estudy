# MongoDB Data Recovery (UNDO, REDO)

MongoDB does not support transactions.

Therefore, it has the disadvantage of not being able to recover data.

Let's explore how to implement data recovery in various environments and what features are supported.

- UNDO - Recovery, rollback to original state
- REDO - Recovery, retry on failure

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbzyU8S%2FbtrmY4M91aK%2FCEMPXOxQvhVCFjQ1lFk2Sk%2Fimg.png)

UNDO and REDO do not exist in MongoDB -> DB is not guaranteed (information is not 100% accurate).

To solve this? -> In such cases, it can be handled at the programming level. (e.g., using try-catch to update or insert again if it fails)

However, even with the method above, MongoDB does not guarantee data.

So, is programming the only way to handle undo/redo in MongoDB?

Not necessarily, I will explain the method at the very bottom.

<br>

### In the diagram above, there is a problem where if even one server fails, the entire system experiences issues.

Therefore, we can try a replication method called **Replica Set**.

With servers replicated in sets of three:

Problem with server 1 -> Server 2 takes over. -> While server 2 is running, server 1 starts recovery. -> If both server 1 and 2 fail -> Server 3 takes over -? Server 1 and 2 start recovery.

When creating replicas, three are usually made as a pair. These three are grouped together and called a **Replica Set**.

This process is called data RAID.

> RAID (Redundant Array of Independent Disks) - A technology that combines multiple disks to use them as a single disk.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fbxm1dj%2FbtrmWsvjQh3%2FNISd4zrEOIvO7DDBJbcHC1%2Fimg.png)

### OPLog

Here, OPLog emerges. OPLog is an area that stores Operands.

If 10 comes in, it stores the operation "save 10!".

And the OPLogs connected to the Primary and Secondary servers communicate by requesting each other for synchronization.

The communication sequence is:

1. OPLogs request each other
2. Check for new operations
3. If there's a response, store the command

The above process is called **HeartBeat**.

HeartBeat is interconnected like a net.

> Polling?
> A method where one device or program periodically checks the state of another device or program for purposes such as synchronization to avoid conflicts, and processes data like transmission/reception until certain conditions are met.

Through OPLog, we can perform data recovery at the MongoDB level, not just the programming level.

<br>

## REDO

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FcO0n08%2FbtrmY4zGm6j%2FdQhf1j0hZzbkoV0lGhCzwK%2Fimg.png)

In the diagram above, Primary Server has 1 and 2 stored.

Secondary Server only has 1 stored.

The OPLog contains commands to store both 1 and 2. By comparing the data on the Primary and Secondary Servers,

it confirms that 2 is not stored and re-executes the OPLog.

If it fails at this point, the contents of the OPLog linked to the Secondary Server are deleted, and it synchronizes with the OPLog linked to the Primary Server via HeartBeat.

Thus, when the command to store 1 and 2 comes again, it executes the command.

This process is executed until the command succeeds. -> REDO

<br>

### UNDO

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FMLTRT%2FbtrmXRA6hMd%2FRqLmTukKerGSVEThVaYOM1%2Fimg.png)

In the diagram above, let's assume HeartBeat was executed at 19.9 seconds, and a command to change 2 to 5 was received by the Primary OPLog at 20 seconds.

This command was executed, changing 2 to 5 on the Primary Server, and the next HeartBeat, executed at 21.9 seconds, would receive the command to change 2 to 5.

However, let's say the Primary Server and OPLog failed at 21.5 seconds.

In this case, one of the remaining two Secondary Servers is elected as the Primary Server, and the system operates with two servers.

At this point, if another client requests 5, the data cannot be confirmed -> Currently, no data recovery.

After about 10 seconds, if the original Primary Server comes back online, it automatically becomes a Secondary Server.

They synchronize via HeartBeat, and based on the Operand, all commands to change 2 to 5 are received.

After recovery, by making the HeartBeat polling interval very short, such problems can be prevented, but this would cause an overload on MongoDB..

To solve the above problem, recovery is performed through a system called `Journaling`, which stores data that can track changes before they are applied.

**Timing: When a command to change 2 to 5 is received by the OPLog, the command is recorded to a file just before execution.**

If a server dies as in the situation above, it first checks the journaling system to see if the Operand is the same, and if not, it synchronizes. This is called the journaling system. There are no issues with client requests.
