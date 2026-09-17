### Service Discovery

In an MSA, each service has its own unique IP and Port.

It is necessary to store and manage the IP and Port information of these different services, and this mechanism is called service discovery.

What are the benefits of using a robust service discovery mechanism in a cloud-based microservices environment?

### High Availability

Service discovery should be able to support a hot clustering environment where service discovery information is shared among cluster nodes.

If one node is unavailable, another node can take over its role.

> A cluster can be defined as a group of servers and instances.

In this case, all instances collaborate with the same configuration to provide high availability, stability, and scalability.

It can provide failover to prevent cluster service interruptions when integrated with a load balancer, and session replication to store session data.

### P2P (Peer to Peer)

Service discovery shares the status of its service instances among all nodes.

### Load Balancing

Service discovery dynamically distributes requests to all managed instances.

It replaces earlier, more static and manually managed load balancers.

### Resilience

Service discovery clients cache service information locally.

Local caching emerged due to the consideration of graceful degradation of service discovery functionality.

Even if service discovery is unavailable, the application can still function.

### Fault Tolerance

If service discovery detects an unhealthy service instance, it must remove that instance from the list of available services that handle client requests.

Such faults should be detected using the service and acted upon without human intervention.
