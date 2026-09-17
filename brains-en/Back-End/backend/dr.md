# Implementing DR with IDC Redundancy + Kafka

**Terminology Before We Start**
- IDC (Internet Data Center) is an abbreviation for Internet Data Center, a facility established when it's necessary to gather and concentrate servers that are central to Internet connectivity.
- DR (Disaster Recovery) refers to quickly restoring a system when an IDC fails during a disaster.

Through redundancy, we can prepare for failures that occur in an IDC during a disaster (or due to system administrator error, etc.).

If the sole IDC fails, no services will be available until it's restored, making prolonged downtime extremely critical.

Therefore, redundancy involves operating two server environments, such as IDCs, to solve this problem. The operating modes include active-standby and active-active.

An active-standby configuration typically operates by bringing up the standby server to recover from a failure when one occurs.

Consequently, if the active IDC does not fail, the standby will not switch to active.

However, in an active-standby setup, if failures in the active system are infrequent (and typically, ordinary failures would be handled by mechanisms within the active system), there might be cases where the standby doesn't properly become active even when a failure occurs.

If an application requires high real-time performance and minimal downtime, an active-active configuration could also be considered.

### Example

Let's consider an example of making a data streaming platform like Kafka redundant. What would happen if real-time information, such as stock market systems or stock data, experienced a failure and increased downtime? It's a terrifying thought.

Therefore, by using an active-active configuration where idc1 and idc2 mirror each other's data and synchronize Offsets, you can even gain the benefit of load balancing for producing events. Here, it's best for only one instance to handle the data consumption mechanism. Since the produced topics are the same, if two instances were to consume, the computation for processing the same data could nearly double.

Furthermore, if idc1 fails, idc2 is still operational, providing a significant advantage in terms of availability.

However, ensuring data consistency in an active-active configuration is very difficult and costly.
Therefore, it requires establishing and managing appropriate streaming data management automation systems, monitoring, and alert systems.
