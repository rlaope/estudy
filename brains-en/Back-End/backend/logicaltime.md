# Distributed System Logical Clocks (Lamport Clock, Vector Clock)

Even with algorithms like Christian's, Berkeley's, or NTP, it's difficult to guarantee time accuracy down to nanoseconds. Therefore, obsessing too much over time might not be beneficial.

Let's reconsider. Ultimately, what we care about is ensuring the **order of operations**.

Focusing on this aspect, let's examine the following topics:
1. Lamport Clock
2. Leslie Lamport Clock

### Lamport Clock

**The Lamport clock is used to accurately record the causal relationship between events.**

Therefore, it can have the following characteristics:
1. It can accurately identify causal relationships.
2. The order can be determined by comparing the Lamport timestamp (Cn) of two events.
3. If two events have the same Lamport timestamp (Cn), the order is determined based on the Lamport clock's sequence.

Let's look at this in a more scenario-based form:
1. Lamport clocks p1, p2, p3 exist.
2. The Lamport timestamp for all clocks starts at 0; that is, c1, c2, c3 all start at 0.
3. If an event is recorded on a specific Lamport clock n, cn increases by 1.
    1. However, if they are different Lamport clocks, c_current = c_previous + 1.

Let's look at a clearer scenario.
- Event publication history
    - p1 publishes a, b
    - p2 publishes c
    - p3 publishes d
- Events a, b, c are consecutive events.

At the initial point, c0, c1, c2 are all 0, meaning they are all equal, so event 'a' on p1, the smallest Lamport clock, occurred first.
```
* [로그] P1.a
    
[값 기록]  
  P1, C1 = 1  
  P2, C2 = 0  
  P3, C3 = 0
```

Since event 'b' was executed after 'a', p1, c1 increases again to 2.

```
* [로그] P1.a -> P1.b
    
[값 기록]  
  P1, C1 = 2
  P2, C2 = 0  
  P3, C3 = 0
```

Subsequently, both p2 and p3 events have the same Lamport timestamp of 0, so p2, with the smaller clock number, would have executed first. Therefore, c2 changes to 3 by c1 + 1 (previous value + 1).
```
* [로그] P1.a -> P1.b -> P2.c
    
[값 기록]  
  P1, C1 = 2
  P2, C2 = 3
  P3, C3 = 0
```

The final p3 d is an independently occurring event.

Therefore, there's no need to compare its continuity with other events.

Since c1 and c3 are both 0, event 'a', on the smallest Lamport clock, started first.

Events b and d
- Since c1 = 1 and c3 = 0, p1 is larger, so 'd' started first.

So the adjusted values are as follows:

```
* [로그] P1.a -> P3.d -> P1.b -> P2.c
    
[값 기록]  
  P1, C1 = 3
  P2, C2 = 4
  P3, C3 = 2
```

In short, a Lamport clock works by having each independent process maintain a counter, incrementing and adjusting their counter values whenever an event occurs to maintain order. This implies that handling concurrency for the same resource in a multi-threaded environment must also be applied.

When using Lamport clocks, tasks to be performed in an application transmit messages along with their Lamport timestamps. These messages are then recorded in each point's own queue and sorted based on their Lamport timestamps.

Thus, it can now be considered for use in the following areas:
1. Database replication algorithms
2. Solving concurrent update issues in distributed databases

### DB Replication Algorithm

Let's design a replication algorithm using the properties mentioned above.
1. Database updates are notified via messages to itself and other points.
2. When an update is received from another point, the task is temporarily stored in a queue.
3. A reception confirmation message is sent to itself and other points, but a reply is only sent if the update task is at the very front of the queue.
4. Upon receiving a reception confirmation message for a task to be updated, it indicates that the task has also been confirmed by other points.
5. The confirmed task is removed from the first position in the queue, and the task is executed.

Overall, it seems that a logic similar to proactive message logging in a two-phase commit protocol is somewhat applied.

**The important part is that messages are logged before being sent, and operations proceed once consensus is reached.**

<br>

### Solving Concurrent Update Issues

Let's solve the banking example we looked at in the previous post.
- Seoul Bank is defined as p1, c1 = 0
- Busan Bank is defined as p2, c2 = 0

That is, in the case of p1's deposit event and p2's interest event, since their Lamport timestamps are the same, p1's counter will be incremented first. This means the Seoul event is published first.

Therefore, it will execute as follows:
- Hold 10,000 KRW → Deposit 1,000 KRW → Pay 1% interest on deposit
- (10,000 KRW + 1,000 KRW) × 1.01 = 11,110 KRW

This means the amount remaining in the account should be the adjusted value, not the expected value. Furthermore, no absolute time is required in this process.

Unlike Lamport clocks, there's also the concept of vector clocks. A vector clock, unlike a Lamport clock, represents the timestamp for each node not as a single variable but as a vector. The biggest difference seems to be between scalar and vector. The purpose of ensuring event order remains the same.
