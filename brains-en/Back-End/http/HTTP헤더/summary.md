# HTTP Header Overview

### HTTP Headers
- header-field = field-name ":" OWS field-value OWS(OWS: whitespace allowed)
- field-name is case-insensitive

```
GET /search?q=hello&hl=ko HTTP/1.1
Host: www.google.com
```
```
HTTP/1.1 200 OK
Content-Type: text/html;charset=UTF-8
Content-Length: 3423

<html>
 <body>...</body>
</html>
```

### Purpose
- All additional information required for HTTP transmission
- e.g., message body content, message body size, compression, authentication, requesting client, server information, cache management information...
- Too many standard headers
- Arbitrary headers can be added if needed
  - helloworld: hihi

### Classification - RFC2616 (Past)

- Header Classification
  - General Header: Information applied to the entire message, e.g., Connection: close
  - Request Header: Request information, e.g., User-Agent : Mozilla/5.0 (Macintosh;..)
  - Response Header: Response information, e.g., Server: Apache
  - Entity Header: Entity body information, e.g., Content-Type: text/html, Content-Length: 3423


<br>

### HTTP BODY
**message body - RFC2616 (Past)**
- The message body is used to transfer the entity body.
- The entity body is the actual data to be transferred in a request or response.
- Entity headers provide information to interpret the data in the entity body.
  - Data type (html, json), data, length, compression information, etc.

### HTTP Standard
1999 FRC -> Deprecated
2014 RFC7230 ~ 7235 published

### RFC723x Changes
- Entity -> Representation
- Representation = representation Metadata + Representation Data
- Representation = Representation Metadata + Representation Data

### message body - RFC7230
- Representation data is transferred via the message body.
- Page body = Payload
- Representation is the actual data to be transferred in a request or response.
- Representation headers provide information to interpret the representation data.
  - Data type (html, json), data length, compression information, etc.
- Note: Although representation headers should distinguish between representation metadata and payload messages, here they are referred to as representation headers.

<br>

## Representation
- Content-Type : Format of representation data
- Content-Encoding : Compression method for representation data
- Content-Language : Natural language of representation data
- Content-Length : Length of representation data
- Representation headers are used for both requests and responses.

### Content-Type
**Description of representation data format**
- Media type, character encoding
- e.g.) text/html; charset=utf-8 , application/json , image/png


### Content-Encoding
**Representation data encoding**
- Used to compress representation data
- The sender compresses the data and adds the encoding header.
- The receiver decompresses the data using information from the encoding header.
- e.g.)
  - gzip (compression)
  - deflate
  - identity (original)

### Content-Language
**Natural language of representation data**
- Expresses the natural language of the representation data
- e.g.)
  - ko
  - en
  - en-US

### Content-Length
**Length of representation data**
- In bytes
- If Transfer-Encoding is used, Content-Length should not be used.
