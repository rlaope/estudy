# gRPC, RPC, How It Works, HTTP API Comparison

## RPC: Remote Procedure Calls
A protocol that allows a program on one computer to execute a procedure on another computer.
  
Developers don't need to explicitly code the details of remote interactions -> the framework handles it automatically.
  
In client code, it appears as if server code functions are being called directly.
  
Client code language != server code language: can be written in different languages.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbqlDmG%2Fbtq7uceRu8g%2FttYh7wR46nA9iHST541Wmk%2Fimg.png)

## gRPC = Communication Framework
Microservices are built with various programming languages (e.g., Back-End: Go, Front-End: js).
  
Communication between these servers is necessary.
  
The number of messages exchanged between microservices is enormous >>> fast communication is preferable.
  
Developers should focus only on implementing core logic and leave communication to the framework.
  
gRPC = High-performance Open-source Feature-rich Framework

- originally developed by Google
- Now part of the Cloud Native Computing Foundation (CNCF - like Kubernetes)
- gRPC messages are primarily encoded in Protobuf (binary format) > efficient for transmission/reception but not human-readable

gRPC generates client code and server interface code via Protocol Buffers. The generated language can be changed based on options.
  
-> A single .proto file can be used to generate server/client code in multiple languages such as Go, Python, Java, etc.

<br>

## How gRPC Works

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FFciKH%2Fbtq7ut1Wtof%2Fz64S0EkpD9yk4WWg048b10%2Fimg.png)

1. The client generates a stub (providing the same methods as the server).
2. The stub calls the gRPC framework (calling via the internal network).
3. Client and server use stubs for interaction -> only requiring access to each other's core service logic.

## gRPC vs HTTP API
Feature | gRPC | HTTP API
--|--|--
Contract | Required (.proto) | Optional (OpenAPI)  
Protocol | HTTP/2 (fast) | HTTP
Payload | Protobuf (compact, binary message format) | JSON (larger, human-readable)  
Norms | Strict specification | Loose. Any HTTP is valid.
Streaming | Client, Server, Bidirectional | Client, Server
Browser Support | No (gRPC-Web required) | Yes
Security | Transport (TLS) | Transport (TLS)  
Client Code Generation | Yes | OpenAPI + Third-party tools

.proto file: Defines gRPC service/message contracts.

## Limitations
Cannot directly call gRPC services from a browser.

- Use gRPC-Web (bidirectional streaming not possible, server streaming limited)
- Use gRPC services with annotations in .proto files as HTTP metadata in a RESTful JSON Web API

(Application supports both JSON Web API and gRPC)
