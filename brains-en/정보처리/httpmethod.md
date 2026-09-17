# HTTP Methods
HTTP methods are `a means for clients to inform web servers of the purpose or type of a user's request`. Initially, HTTP only had the GET method, but various other methods have emerged since then.

<br>

### Types and Characteristics of HTTP Methods
There are a total of 9 types of HTTP methods. Among these, 5 methods are primarily used. Let's explore each of their names and characteristics.

5 Main Methods
- GET : Retrieve a resource
- POST : Process request data
- PUT : Replace a resource; create it if it doesn't exist
- PATCH : Partially modify a resource
- DELETE : Delete a resource

4 Other Methods
- HEAD : Identical to GET, but returns only the status line and headers, excluding the message body
- OPTIONS : Describes the communication options available for the target resource
- CONNECT : Establishes a tunnel to the server identified by the target resource
- TRACE : Performs a message loop-back test along the path to the target resource

**Let's delve deeper into the 5 main methods.**
1. GET is typically used to `retrieve` resources, and data to be sent to the server is transmitted via the query string. While it's possible to send data using the message body, it's not recommended as many implementations do not support it.
2. POST processes data requests and sends data to the server via the message body. It is primarily used for registering new resources or processing data.
3. PUT replaces a resource if it exists, and creates it if it doesn't. In simple terms, it overwrites data.
4. PATCH is used to modify a resource, similar to PUT, but PATCH can only change a portion of the resource.
5. DELETE is used to remove a resource.

<br>

### HTTP Method Properties
HTTP method properties include safety, idempotence, and cacheability.

1. Safety : This means that repeatedly calling the method does not alter the resource. Among the main methods, GET can be considered safe.
2. Idempotence : This means that repeatedly calling the method yields the same result. GET, PUT, and DELETE can be considered idempotent, but POST and PATCH cannot.
3. Cacheability : Cacheable literally means that data can be retrieved efficiently through caching. While GET, HEAD, POST, and PATCH are cacheable, caching is primarily used for GET and HEAD in practice.

![Method Table](./image/http-method.png)

<br>

### HTTP Status Codes
HTTP status codes are a feature that informs the client about the processing status of their request in the response.
They typically range from 1xx to 5xx and can be broadly categorized as follows:
- 1xx (Information): Request received, continuing process
- 2xx (Successful): Request successfully processed
- 3xx (Redirection) : Further action needs to be taken to complete the request
- 4xx (Client Error) : Client error; the server cannot fulfill the request due to incorrect syntax or similar issues
- 5xx (Server Error) : Server error; the server failed to fulfill a valid request

<br>

### Types and Meanings of HTTP Status Codes
First, 1xx codes indicate that the request has been received and is being processed, but they are rarely used, so we'll omit them.

2xx codes signify success.
- 200 OK : Request successful
- 201 CREATE : Request successful, new resource created
- 202 Accepted : Request accepted for processing, but the processing has not been completed
- 204 No Content : The server successfully fulfilled the request, but there is no content to send in the response payload body

3xx codes indicate redirection, which means automatically moving to the location specified in the `Location` header.
- 301 Moved Permanently : The redirect request method changes to GET, and the body may be removed
- 302 FOUND : During redirection, the request method changes to GET, and the body may be removed
- 303 See Other : During redirection, the request method changes to GET
- 304 Not Modified : Used for caching purposes
- 307 Temporary Redirect : During redirection, the request method and body are preserved (the request method must not be changed)
- 308 Permanent Redirect : During redirection, the request method and body are preserved (if an initial POST is sent, the redirect also preserves it)

4xx codes indicate that an error occurred on the client side.
- 400 Bad Request : The client sent an invalid request, and the server cannot process it
- 401 Unauthorized : The client needs authentication for the requested resource
- 403 Forbidden : The server understood the request but refused to authorize it
- 404 Not Found : The requested resource could not be found

5xx codes indicate that an error occurred on the server side.
- 500 Internal Server Error : An error occurred due to a server issue; if ambiguous, use 500
- 503 Service Unavailable : Service is unavailable
