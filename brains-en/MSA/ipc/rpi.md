# Synchronous RPI Pattern Application Communication

RPI is an IPC where a client sends a request to a service, and the service processes it and returns a response. While there can be various types, REST and gRPC are representative examples.

## REST
It represents resources existing on the web with URLs and actions on those resources with HTTP protocol methods. The data format is mostly JSON.

### Advantages
- Simple and familiar
- Easy to test
- No need for an intermediate broker, simplifying the system architecture

### Disadvantages
- Supports only request/response style communication.
- Lower availability.
- Difficult to fetch multiple resources with a single request. (GraphQL is emerging to efficiently query data.)
- Often difficult to map multiple update operations to HTTP verbs (a specific data item can have multiple actions, and not all can be mapped to methods).

<br>

## gRPC
It is a binary message-based protocol, and by defining protocol buffers as IDL, a protocol buffer compiler can generate client-side stubs and server-side skeletons.

### Advantages
- Easy to design APIs that include various update operations.
- Compact and efficient IPC when exchanging large messages
- Thanks to bidirectional streaming, it supports both RPI and messaging communication styles.
