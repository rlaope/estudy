# HTTP Messages

![message](../image/httpmessage.png)

### Request Message
- start-line : `request-line` / status-line
- `request-line` = method SP(space) request-target(Path=request target) SP HTTP-version CRLF(enter)

#### HTTP Methods (GET for retrieval)
Types : GET , POST , PUT, DELETE..
Specifies the action the server should perform
- GET : Retrieve a resource
- POST : Process request body

#### Request Target (/search?p=hello&hl=ko)
absolute-path\[?query] absolute path\[query]
Absolute path = a path starting with "/"
Note: There are other types of path specification methods, such as *, http://...?x=y.

#### HTTP version

<br>

### Response Message

- start-line = request-line / status-line

- status-line = HTTP-version SP status-code SP reason-phrase CRLF

- HTTP Version
- HTTP Status Code: Indicates request success or failure
200: Success
400: Client request error
500: Server internal error

Reason Phrase: A short, human-readable description of the status code

<br>

### HTTP Headers
header-field = field-name ":" OWS field-value OWS (OWS:whitespace allowed)
field-name is case-insensitive

#### HTTP Request Message
GET/search?=q=hello&hl=ko HTTP/1.1
**Host : www.google.com**

#### HTTP Response Message
HTTP/1.1 200 OK
**Content-Type : text/html;charset=UTF-8**
**Content-Length : 3423**

**Purpose**
- All additional information required for HTTP transmission
- e.g. message body content, message body size, compression, authentication, request client (browser)
- Too many standard headers
- Can add arbitrary headers if needed
- e.g. helloworld: hihi

#### HTTP Message Body

**Purpose**
- Actual data to be transmitted
- Can transmit all data that can be represented as bytes, such as HTML documents, images, videos, JSON, etc.

**Simple and extensible**
- HTTP is simple, its spec is worth reading..
- HTTP messages are also very simple.
- Highly successful standard technologies are simple yet extensible.

<BR>

### HTTP Summary
- Everything is transmitted in HTTP messages
- HTTP History: Learn based on HTTP/1.1
- Client-server architecture
- Stateless protocol
- HTTP Messages
- Simple, extensible
- This is the era of HTTP
