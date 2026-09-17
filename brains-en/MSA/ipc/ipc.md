# Microservice Architecture IPC

IPC (Inter-Process Communication) is a term that refers to the act of processes exchanging data with each other, or the methods and channels for doing so.

It is essential in a microservice architecture where applications are composed of multiple services. While JSON-based REST is commonly used, it's important to carefully choose from various options in general situations, not just MSA.

### One-to-One / One-to-Many
- One-to-One: Each client request is handled by exactly one service.
- One-to-Many: Multiple services collaborate to handle each client request.

### Synchronous / Asynchronous
- Synchronous: The client assumes the service will respond in time and may block while waiting.
- Asynchronous: The client does not block, and the response does not need to be sent immediately.

### One-to-One Interactions
- Request/Response, Asynchronous Request/Response, Unidirectional Notification

### One-to-Many Interactions
- Publish/Subscribe, Publish/Asynchronous Response
