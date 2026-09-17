# HTTP Methods - GET, POST

<br>

### Types of HTTP Methods

Main Methods
- GET : Retrieve a resource
- POST : Process request data, primarily used for creation
- PUT : Replace a resource; if the resource doesn't exist, create it
- PATCH : Partially modify a resource
- DELETE : Delete a resource

Other Methods
- HEAD : Same as GET, but excludes the message body, returning only the status line and headers
- OPTIONS : Describes the communication options (methods) available for the target resource (primarily used in CORS)
- CONNECT : Establishes a tunnel to the server identified by the target resource
- TRACE : Performs a message loop-back test along the path to the target resource

<br>

### GET
- Retrieve a resource
- Data to be sent to the server is passed via the query string
- Although it's possible to send data using the message body, it's not recommended as many places do not support it

### POST
- Process request data
- Send request data to the server via the message body
- The server processes the request data
- Performs all functions that process data received through the message body.
- Primarily used for creating new resources or processing tasks with the transmitted data

```
POST /members HTTP/1.1
Content-Type: application/json

{
  "username" : "hello",
  "age" : 20
}
```

**What does "process request data" mean? Examples**

- Specification: The POST method requests that the target resource process the representation enclosed in the request according to the resource's own specific semantics. (Google Translate)
- For example, creating a new resource not identified by P.

1. Create a new resource (registration)
   - Create a new resource that the server has not yet identified.

2. Process request data
  - Cases where a process needs to be handled, beyond simply creating or modifying data.
  - E.g., in an order, from "payment complete" to "delivery started" to "delivery complete," where the state of a process changes beyond a simple value modification.
  - A new resource may not necessarily be created as a result of a POST request.
  - E.g., POST /orders/{orderId}/start-delivery (control URI)

3. Cases that are ambiguous to handle with other methods
   - E.g., when you need to pass query data as JSON, but it's difficult to use the GET method.
   - If ambiguous, POST is used for the following functions:
  - Providing a block of data, such as the fields entered into an HTML form, to a data-handling process (e.g., user registration, orders using information entered in an HTML FORM).
  - Posting messages to a bulletin board, newsgroup, mailing list, blog, or similar group of articles (e.g., writing a forum post, adding a comment).
  - Creating a new resource that the server has not yet identified (e.g., creating a new order).
  - Appending data to an existing resource (e.g., adding content to the end of a document).

- Summary: When a POST request comes to this resource URI, how the request data should be processed must be defined separately for each resource -> nothing is predefined.

<br>
