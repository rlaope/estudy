# Time Synchronization Issues in Distributed Systems

Learning Objectives
- How to synchronize time among distributed devices (programs)?
- How to ensure distributed devices maintain consistent information?


### Concurrent Updates in Distributed Databases

For example, let's say User1 has an account with a banking service. The bank operates multiple databases in Seoul and Busan, and deposits are handled by the Seoul bank, while interest payments are handled by the Busan bank.

In this scenario, what would happen if User1 deposits 1,000 won in Seoul at the time of interest payment?

- 2024-10-15 15:00:00 Seoul User1 holds 10,000 won
- 2024-10-15 15:00:00 Busan User1 holds 10,000 won
- 2024-10-15 15:01:00 Busan User1 decides to pay 1% interest
- 2024-10-15 15:01:01 Seoul User1 deposits 1,000 won

According to the scenario above, it seems like the user would have 11,100 won, but due to clock discrepancies between Seoul and Busan, the exact result becomes unpredictable.

The expected value is 10,000 + 10,000 * 0.01 + 1,000, but if the Seoul clock runs 2 seconds slower, it would be as follows:

(10,000 + 1,000) + 10,000 * 1.01

To resolve issues arising from time discrepancies between these distributed systems, each system must synchronize information such as time.

<br>

### Time Synchronization Techniques

There can be two algorithms depending on the presence or absence of a **server that provides a reference time**.
- If present: **Cristian's Algorithm**
- If absent: Berkeley Algorithm

Additionally, a separate protocol called **NTP Network Time Protocol** can be used.

<br>

### Cristian's Algorithm

Cristian's Algorithm is an algorithm where a single central server controlling time synchronizes the clocks of distributed devices.

Therefore, it makes the following assumptions:
1. Devices requiring time synchronization are called clients.
2. The central device that performs time synchronization is called the server.
3. When a client sends a request to the server, the difference between the two times is called delta req.
4. When the server sends a response to the client, the difference between the two times is called delta resp.
5. (Key) It is assumed that delta req == delta resp.
6. T4 is unknown from the server's perspective and is assumed to be the following value based on assumption 5.
`T4 = T3 + dreq|resp / 2`

However, Cristian's Algorithm has the following limitations:
1. Assumption 5, that delta req and resp are identical, is physically difficult to hold true.
2. In assumption 2, if the central server crashes, the entire system crashes, thus it acts as a SPoF.

<br>

### Berkeley Algorithm

The Berkeley Algorithm solves the synchronization problem using only the internal clocks of distributed devices, **without a separate server**.

In this process, a **master device** is selected to act as the central point, ensuring accuracy.

It involves the following steps:
1. Distributed devices have clocks with similar accuracy.
2. One of the distributed devices acts as a master device.
3. The master device sends requests to other devices and calculates the **average time** of the times received by each device.
4. It determines an adjustment value to apply to all distributed devices to match the average time.

Therefore, it has the following limitations:
1. Which device to elect as master.
2. Inability to know network time during message exchange.

Additionally, complex issues such as automatic recovery and maintaining consistency when the master fails also remain.

<br>

### NTP Network Time Protocol

The Network Time Protocol is composed of multiple strata and solves time synchronization issues using Cristian's Algorithm.

|Layer|Definition|Example|
|---|---|---|
|Stratum 0|Most accurate time, central server role|Radio stations, satellites|
|Stratum 1|Client role querying for reference time||
|Stratum 2|Client role querying for reference time||
Naturally, it can suffer from both the delta req == resp problem and the SPoF problem inherent in Cristian's Algorithm. Therefore, ultimately, **time synchronization is very difficult, and not obsessing over this aspect can be a solution for distributed computing.**
