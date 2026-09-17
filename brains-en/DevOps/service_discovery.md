# Service Discovery Patterns in MSA

## MSA Service Discovery Patterns
Distributed environments like MSA are composed of remote calls between services.

Remote service calls typically use IP addresses and ports.

In cloud environments, services are frequently created dynamically due to auto-scaling or container-based deployments, leading to frequent dynamic changes in service IPs.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Ft1.daumcdn.net%2Fcfile%2Ftistory%2F99AD92335AD610DE16)

Therefore, when a service client wants to call a service, it needs a mechanism to find the service's location (i.e., IP, Port), which is precisely what `Service discovery` refers to.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Ft1.daumcdn.net%2Fcfile%2Ftistory%2F99E912455AD610DE09)

Let's look at the next diagram. When instances of Service A are created, their addresses are registered with a Service registry (service registration server).

A client wishing to call Service A queries the Service registry for Service A's address, receives the registered address, and then calls the service at that address.

## Client Discovery vs Server-Side Discovery
There are two main approaches to implementing Service discovery: client-side discovery and server-side discovery.

The method described earlier, where the service client finds the service's location from the service registry and then calls it, is called client-side discovery.

Another approach involves placing a type of proxy server (load balancer) in front of the service being called. In this method, the service client calls the load balancer, which then retrieves the registered service location from the service registry and routes the request based on that information.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Ft1.daumcdn.net%2Fcfile%2Ftistory%2F99AF813E5AD610DE03)

The most common example is the load balancers used in the cloud. AWS ELB and Google Cloud Load Balancer are typical examples of server-side discovery.

## Service Registry
So, how should a Service registry, which registers services, be implemented?

The simplest way to implement this is by registering multiple IPs to a single hostname in DNS records.

However, DNS is not ideal because it takes time for updates, such as when records are deleted. Therefore, using dedicated solutions is often preferred. Services like ZooKeeper or etcd can be utilized, or specialized Service discovery solutions such as Netflix's Eureka and HashiCorp's Consul are available.

## Enhanced Features
While Service discovery primarily involves registering services and returning a list of registered services, it can offer more advanced features through intelligent functionalities.

For example, it can perform health checks on registered services in the Service registry to determine which services are currently available, and then return only the list of available services.

It can also include advanced features like adjusting load distribution ratios between services, returning Master/Slave server information from the server list, or even returning authentication key information needed to connect to servers, thus allowing for expansion into various functionalities.
