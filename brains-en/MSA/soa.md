# MSA, MA, SOA(Service Oriented Architecture), ESB, SOAP

### Overview

Before diving into Spring Cloud-related technologies, while studying Monolithic Architecture and Microservices Architecture, I decided to delve deeper into SOA, ESB, and SOAP within the context of Monolithic Architecture.

### Monolithic Architecture

This architecture handles all tasks within a single, massive application.

It is an architecture with strong internal dependencies and high structural coupling.

Since all tasks are processed on a single piece of hardware, powerful hardware is required,

and the system can become sluggish when subjected to heavy traffic.

### Microservices Architecture

This architecture allows small, independent services to be broken down, enabling independent development and deployment by individual teams.

It can flexibly handle situations even when subjected to heavy traffic.

It allows for horizontal scaling and can be seen as an architecture suitable for cloud services.

### SOA Service Oriented Architecture

SOA, or Service-Oriented Architecture, is an architecture that proceeds with development **service-centric** and ensures reusability by sharing the functionalities of each service.

In SOA, a service is a reusable, independent module; each service performs a clear function and depends on specific inputs and outputs. Such services can be invoked by other systems and services using appropriate interfaces.

**Advantages of SOA**

* Reusability
* Flexibility
* Interoperability
* Modularity

**Disadvantages of SOA**

Because each service in SOA is developed independently, managing dependencies between services can be challenging, and the overall complexity can increase as the number of services grows.

MSA is also an architecture that implements SOA. In other words, MSA can be seen as an evolution of the SOA concept.

MSA uses smaller units of services, making it easier to manage dependencies between services.

### ESB Enterprise Service Bus

ESB is a software architecture designed to support the integration of various applications and services. Based on Message Oriented Middleware, it provides functionalities for transmitting and transforming data between multiple applications and services.

ESB supports various protocols and is one of the core technologies for implementing SOA. It also standardizes interfaces between applications and services and acts as an intermediary, improving flexibility and interoperability between systems.

![](image/soa_0.png)

### SOAP Simple Object Acess Protocol

**It is the first protocol designed to allow applications built in different languages on different platforms to communicate.**

It emerged before REST API and is a communication protocol for exchanging XML over computer networks using HTTP, SMTP, etc. Because it operates over HTTP, communication is possible without the influence of proxies and firewalls. However, it has the disadvantages of being more complex in structure, harder to handle, and heavier than REST API.

#### Structure of SOAP

A SOAP message is largely divided into two parts: **Header and Body**.

The header contains information about message transmission and the recipient,

and the body contains the actual data to be transmitted. These SOAP messages are defined using XML Schema (XSD).

#### However... REST

Currently, with the emergence of REST (Representational State Transfer), it is rising as an alternative to SOAP and is used in many web services due to its simpler structure and faster processing speed.

### Conclusion

Thus, we have learned about MA, MSA, SOA, ESB, and SOAP.

I plan to further study MSA technologies and wish to acquire cloud technologies in the future.
