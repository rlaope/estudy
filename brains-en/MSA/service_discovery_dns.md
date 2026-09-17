# Reasons Not to Use DNS and Load Balancer for Service Discovery

When an application calls resources distributed across multiple servers, it must know the **physical location** of these resources.

Therefore, service location resolution was typically handled by a combination of DNS and Load Balancer.

![](https://thebook.io/img/080283/212.jpg)

In traditional scenarios, a load balancer's routing table entry, which receives requests from service consumers, contains a list of one or more servers hosting the service.

The load balancer selects one server from this list and forwards the request.

Service instances in this traditional approach were **deployed on one or more application servers.**

The number of applications was typically fixed (e.g., the number of applications hosting the service did not increase or decrease) and persistent. That is, if a server running an application failed, it would be restored with the same IP address and configuration it was using.

For high availability, a standby secondary load balancer would send ping signals to check the liveness of the primary load balancer.

If the primary load balancer was down, the secondary load balancer would become active, take over the primary load balancer's IP, and process requests.

This model works well for applications running within a company's walled-off data center and for a relatively small number of services running on fixed servers, but it does not work well for cloud-based microservice applications.

<br>

### Single Point of Failure

If a load balancer goes down, all applications that depend on it also go down.

Even if a load balancer is made highly available, it is likely to become a centralized gateway within the application infrastructure.

<br>

### Limited Horizontal Scalability

Commercial load balancers are constrained by two factors: their redundancy model and licensing costs.

Most commercial load balancers use a hot-swap model for redundancy (replacing components without affecting the operating system), so only one server handles the load.

That is, the secondary load balancer exists solely for failover in case the primary load balancer goes down.

Inherently hardware-constrained, commercial load balancers are designed for fixed capacities rather than more variable models and have restrictive licensing models.

<br>

### Most Load Balancers Are Statically Managed

Most of these load balancers are not designed to dynamically register and deregister services quickly.

They use a centralized database to store routing rules, and typically require using a vendor's proprietary API to store new routes.

<br>

### Load Balancers Act as Proxies

Load balancers act as proxies for application servers. This translation layer adds complexity to the service infrastructure because service mapping rules must be manually defined and deployed.

Furthermore, in traditional load balancer scenarios, new service instances are not registered with the load balancer when they start.

<br>

For these four reasons, service discovery is not typically implemented using DNS and Load Balancer.

It's important not to misunderstand: DNS and Load Balancers are by no means bad.

Firstly, this approach works well in centralized network infrastructures.

And load balancers still play a crucial role in handling SSL termination and managing service port security.

Behind a load balancer, access to incoming and outgoing ports for all servers can be restricted.

This concept of minimal network access often becomes an important factor when trying to meet requirements for industry standard certifications, such as PCI compliance.
