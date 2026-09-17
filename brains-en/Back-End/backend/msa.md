# What is MSA Microservice Architecture?

### What is MSA?
**MicroService Architecture**

- MSA is an abbreviation for MicroService Architecture, and it is one of the software development techniques.
- MSA is a framework composed of small, independently deployable services, each performing a specific function.
- It's a method of implementing applications by combining multiple lightweight and independent services. Since each service operates with its own database, development, building, and deployment can be performed efficiently.

![msa](./image/msa.png)

### Background of MSA's Emergence
![mas등장](image/msa등장.png)

In the early stages of application development, the 'Monolithic' approach was used, embedding the entire source code into a single deployment unit (war or ear). However, even minor changes to existing applications often led to frequent downtime, such as updating according to an internal QA (Quality Assurance) cycle, or having to shut down the entire system and resolve errors when an error occurred due to a partial service update. To solve these problems, the MicroService Architecture emerged, which divides the core services of an application, allowing each service to be built and deployed independently.

<br>

### What is Monolithic?
**It refers to a form where all components of software are integrated into a single project, developed module by module, and then packaged and deployed as one complete artifact.**

#### Advantages
- Simple architectural structure and ease of development in the early stages

#### Disadvantages
- As the service grows, understanding the overall system structure and maintenance becomes difficult.
- Partial failures can escalate into failures of the entire service.
- Deployment time is long.
- Dependent on a single framework and language.
- Difficult for partial Scale-out (a method of distributing work across multiple servers).

<br>

### Advantages of MSA
- Distributed development shortens the development cycle, enabling fast and flexible deployment (reduced time to market).
- Services are independent, so they do not affect other services (excellent recovery capability).
- Freedom to adopt and extend technologies per service (high scalability).
- Compared to the monolithic approach, applications are modularized and smaller in scale, reducing concerns (easy deployment).
- Uses polyglot APIs (enhanced openness).
- Since a single application is divided into multiple parts, updating and improving each service is easy (convenient access).

### Disadvantages of MSA
- Each service communicates via APIs, leading to overhead due to network communication.
- Logs are generated per service, so there is no central log monitoring.
- With numerous services in a single project, monitoring overhead for all services increases.
- Since one service calls another, tracing paths and failures is difficult when an error occurs.
- Services are distributed, making them relatively more complex compared to monolithic architectures.
