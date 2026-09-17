# General Information
- From: User agent's email information
- Referer: Previous web page address
- User-Agent: User agent application information
- Server: Software information of the origin server processing the request
- Date: Date the message was generated

### From
**User agent's email information**
- Generally not widely used
- Primarily used by search engines, etc.
- Used in requests

### Referer
**Previous web page address**

- The address of the previous web page for the currently requested page
- When navigating from A to B, the request for B includes Referer: A
- Referer can be used to analyze traffic sources
- Used in requests
- Note: "referer" is a misspelling of "referrer"

### User-Agent
**User agent application information**

- user-agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36(KHTML, like Gecko) Chrome/86.0.4240.183 Safari/537.36
- Client's application information (web browser information, etc.)
- Statistical information
- Can identify which types of browsers are experiencing issues
- Used in requests

### Server
**Software information of the ORIGIN server processing the request**
- Server: Apache/2.2.22(Debian)
- server: nginx
- Used in responses

### Date
**Date and time the message was generated**
- Date: Tue, 15 Nov 1994 08:12:31 GMT
- Used in responses

<br>

## Special Information
- Host: Requested host information (domain)
- Location: Page redirection
- Allow: Permitted HTTP methods
- Retry-After: Time the user agent should wait before making the next request

### Host
**Requested host information (domain)**
- Used in requests
- Required
- When a single server needs to handle multiple domains
- When multiple domains are applied to a single IP address, a server can handle multiple domains at once via virtual hosts, and multiple actual applications can be running.

```
GET /hello HTTP/1.1
Host: aaa.com
```

### Location
**Page redirection**
- If a web browser receives a 3xx response with a Location header, it automatically navigates (redirects) to the Location specified.
- Explained in response code 3xx
- 201 (Created): The Location value is the URI of the resource created by the request
- 3xx (Redirection): The Location value points to the target resource for automatically redirecting the request

### Allow
**Permitted HTTP methods**
- Must be included in the response for 405 (Method Not Allowed)
- Allow: GET HEAD PUT

### Retry-After
**Time the user agent should wait before making the next request**
- 503 (Service Unavailable): Can indicate until when the service will be unavailable
- Retry-After: Fri, 31 Dec 1999 23:59:59 GMT (date format)
- Retry-After: 120 (seconds format)
