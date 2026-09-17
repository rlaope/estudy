# 3xx - Redirection

### 3xx (Redirection)
Additional action by the user agent is required to complete the request.
- 300 Multiple choices
- 301 Moved Permanently
- 302 Found
- 303 See Other
- 304 Not Modified
- 307 Temporary Redirect
- 308 Permanent Redirect

### Understanding Redirection
- If a web browser receives a 3xx response with a Location header, it automatically moves (redirects) to the specified Location.

Automatic Redirection Flow
1. Request Client -> Server
```
GET /event HTTP/1.1
Host: localhost:8080
```
2. Response -> Server -> Client

```
HTTP/1.1 301 Moved Permanently
Location: /new-event
```

3. Automatic Redirection Client -> Client
4. Request Client -> Server
```
GET /new-event HTTP/1.1
Host: localhost:8080
```
5. Response -> Client
```
HTTP/1.1 200 OK
```

<br>

### Types
- Permanent Redirection - The URI of a specific resource has moved permanently.
  - e.g.) /members -> /users
  - /event -> /new-event
- Temporary Redirection - A temporary change.
  - Moving to the order history screen after completing an order.
  - PRG : Post/Redirect/Get
- Special Redirection
  - Using cache instead of result.

### Permanent Redirection

#### 301 , 308
- The URI of the resource has moved permanently.
- The original URL is no longer used; search engines also recognize the change.
- 301 Moved Permanently
  - When redirecting, the request method MAY change to GET, and the body MAY be removed.
- 308 Permanent Redirect
  - Functionality is the same as 301.
  - When redirecting, the request method and body are preserved (if the initial request was POST, the redirection maintains POST).

### Permanent Redirection - 301

1. Request Client -> Server, using POST, message present

```
POST /event HTTP/1.1
Host: localhost:8080

name=hello&ange=20
```
2. Response Server -> Client

```
HTTP/1.1 301 Moved Permanently
Location: /new-event
```

3. Automatic Redirection Client -> Client
4. Request Client -> Server, changed to GET, message removed

```
GET /new-event HTTP/1.1
Host: localhost:8080
```

5. Response Server -> Client
  
```
HTTP/1.1 200 OK
```

<br>

### Permanent Redirection - 308

1. Request Client -> Server, using POST, message present
```
POST /event HTTP/1.1
Host: localhost:8080

name=hello&age=20
```
2. Response Server -> Client

```
HTTP/1.1 308 Permanent Redirect
Location: /new-eve
```
3. Automatic Redirection Client -> Client
4. Request Client -> Server

```
POST /new-event HTTP/1.1
Host: localhost:8080

name=hello&age=20
```
5. Response Server -> Client

```
HTTP/1.1 200 OK
```

<br>

### Temporary Redirection 302, 307, 303

- The URI of the resource is temporarily changed.
- Therefore, search engines should not change the URL.
- 302 Found
  - When redirecting, the request method MAY change to GET, and the body MAY be removed.
- 307 Temporary Redirect
  - Functionality is the same as 302.
  - When redirecting, the request method and body are preserved (the request method MUST NOT be changed).
- 303 See Other
  - Functionality is the same as 302.
  - When redirecting, the request method changes to GET.

### PRG Post/Redirect/Get
**Temporary Redirection - Example**
- What happens if you refresh the web browser after ordering with POST?
- Refreshing is a re-request.
- This can lead to duplicate orders.

### Before using PRG
1. Request

```
POST /order HTTP/1.1
Host: localhost:8080

itemId=mouse&count=1
```
2. Save order data: 1 mouse
3. Response

```
HTTP/1.1 200 OK

<html>주문완료</html>
```
4. Refresh on the result screen
5. Request

```
POST /order HTTP/1.1
Host: localhost:8080

itemId=mouse&count=1
```

6. Save order data: 1 mouse
7. Response

```
HTTP/1.1 200 OK

<html>주문완료</html>
```

### Temporary Redirection - Example
- Prevents duplicate orders caused by refreshing after ordering with POST.
- Redirects to the order result screen with a GET method after ordering with POST.
- Even if refreshed, the result screen is retrieved with GET.
- Instead of duplicate orders, only the result screen is re-requested with GET.

1. Request
```
POST /order HTTP/1.1
Host: localhost:8080

itemId=mouse&count=1
```

2. Save order data: 1 mouse
3. Response

```
HTTP/1.1 302 Found
Location: /order-result/19
```

4. Refresh on the result screen
5. Request

```
GET /order-result/19 HTTP/1.1
Host: localhost:8080
```

6. Retrieve order data: order number 19
7. Response

```
HTTP/1.1 200 OK	

<html>주문완료</html>
```
8. Refresh on the result screen
   - GET /order-result/19
   - Only the result screen is re-requested (moves to step 5).

- Redirection after PRG
  - The URL has already been redirected from POST -> GET.
  - Even if refreshed, only the result screen is retrieved with GET.

<br>

### So, what should I use?

**302, 307, 303**

- Quick summary
  - 302 Found -> Can change to GET
  - 307 Temporary Redirect -> Method must not change
  - 303 See Other -> Method changes to GET
- History
  - The initial intent of the 302 specification was to preserve the HTTP method.
  - However, most web browsers changed it to GET (some behave differently).
  - Thus, clear 307 and 303 emerged to replace the ambiguous 302 (308 also appeared as an alternative to 301).
- Reality
  - While 307 and 303 are recommended, in reality, many application libraries already use 302 as the default.
  - If it's acceptable for the method to change to GET during automatic redirection, using 302 is generally not a big problem.

<br>

### Other Redirections
- 300 Multiple Choice: Not used.
- 304 Not Modified
  - Used for caching purposes.
  - Informs the client that the resource has not been modified. Therefore, the client reuses the cached version stored on its local PC (redirects to the cache).
  - A 304 response MUST NOT contain a message body (because the local cache should be used).
  - Used with conditional GET and HEAD requests.
