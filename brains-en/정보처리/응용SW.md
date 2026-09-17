# Application Software (Computer System General Study Notes)

### Network Protocol
- A set of pre-defined transmission rules that allow messages to be exchanged between computers or remote communication devices.

### OSI 7-Layer Model

`Necessity of OSI 7-Layer Model`: During data communication, if there are differences in the structure and protocols between the sender and receiver, the transmitted data may not be correctly recognized. To address this, problems can be resolved by providing standardized services and protocols at each layer.

`Roles and Functions of the OSI 7-Layer Transport Layer`
- Port assignment for sender and receiver
- Message segmentation and reassembly
- Congestion control and flow control between processes

### What is TCP Communication?
- A protocol belonging to the transport layer
- A connection-oriented method that can transmit data 1:1 without errors, based on reliability.

### What is a Packet Switching System?
- A data communication system that divides data into packets, creates headers with sender and receiver addresses, and then uses a switch to refer to the header information to deliver the packets to the destination.

### Link-State Routing
`Link-State Routing Algorithm`

- A method that receives connection state information from all connected routers and creates a routing table with the shortest path to each router.
- Exchanges information between routers using the OSPF protocol.

### Concept of Middleware
- Software located between the operating system and application software to support various functions, ensuring complex processing runs smoothly for stable system execution.

`One of the main functions`
- IT Resource Management: Software that provides functions to continuously monitor and manage performance and availability based on management policies for IT resources.

`Why middleware is needed from the perspective of OS and application software`
- Operating System: Can lead to improved task processing efficiency and maintainability.
- Application Software: Can improve development productivity by minimizing redundant development, and enhance overall system stability by utilizing verified middleware modules.

### Concept of e-Government Standard Framework
- A development framework utilized in public information projects by government ministries, local governments, and public institutions when building `JAVA`-based web/mobile systems.

`Functions and Roles of the Development Environment among the Components of the e-Government Standard Framework`
- Data processing
- Test automation
- Code inspection
- Support for open-source software required for development

### Scouter
- An application performance management tool released as open-source software, which is middleware that helps application software engineers stably operate and manage systems in distributed environments.

`Scouter Agent`
- Installed on target systems for monitoring, such as WAS and Database, to measure performance and transmit the results to the server.

### DBMS Functions
- Concurrency Control: Performs control to ensure data integrity from concurrent processing of multiple transactions.
- Recovery Management: Responds to data loss and corruption due to system errors and failures.
- Performance Management: Optimizes execution plans to ensure data processing speed.
- Security Management: Controls access for unauthorized users and encrypts sensitive information.
