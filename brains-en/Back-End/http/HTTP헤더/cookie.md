# Authentication, Cookies

### Authentication
- Authorization: Client authentication information delivered to the server
- WWW-Authenticate: Defines authentication methods required for resource access

### Authorization (Authorization, Permission Grant)
Client authentication information delivered to the server
- Authorization: Basic xxxxxxxxxxx

### WWW-Authenticate (Authentication)
Defines authentication methods required for resource access
- Defines authentication methods required for resource access
- Used with 401 Unauthorized response
- WWW-Authenticate: Newauth realm= "apps", type=1, title="Login to \"apps\
"", Basic realm="simple"

<br>

## Cookies
- Set-Cookie: Server sends cookie to client (response)
- Cookie: Client stores cookies received from the server and sends them to the server with HTTP requests

### Stateless
- HTTP is a stateless protocol.
- The connection is terminated once the client and server exchange requests and responses.
- If the client requests again, the server does not remember the previous request.
- The client and server do not maintain state with each other.

Without cookies, the server doesn't know who the client is.
While user information can be included in every request, it's cumbersome and increases data volume.

<br>

### Login
1
```
POST /login HTTP/1.1
user=홍길동
```

2
```
HTTP/1.1 200 OK
Set-Cookie: user=홍길동

홍길동님이 로그인했습니다.
```

3. Stored in cookie store
4. Access welcome page after login

```
GET /welcome HTTP/1.1
Cookie: user=홍길동
```
5
```
HTTP/1.1 200 OK

안녕하세요. 홍길동님
```

### Cookies
- Example) set-cookie: sessionId=abcde1234; expires=Sat, 26-Dec-2020 00:00:00 GMT; path=/; domain= google.com; Secure
- Use cases
  - User login session management
  - Tracking advertising information
- Cookie information is always sent to the server
  - Causes additional network traffic
  - Use only minimal information (session ID, authentication token)
  - If you want to store data inside the web browser without sending it to the server, refer to web storage
- Caution
  - Sensitive data (e.g., resident registration numbers, credit card numbers) should not be stored

### Cookies - Lifecycle
**Expires,max-age**
- Set-Cookie: expires=Sat, 26-Dec-2020 04:39:21 GMT
  - Cookie deleted when expired
- Set-Cookie: max-age=3600 (3600 seconds)
  - Specifying 0 or a negative number deletes the cookie
- Session cookie: If the expiration date is omitted, it persists only until the browser closes
- Persistent cookie: If an expiration date is entered, it persists until that date

### Cookies - Domain
**Domain**
- Example) domain=example.org
- Explicit: Includes the specified document's domain + subdomains
  - Create cookie by specifying domain=example.org
    - example.org, of course,
    - dev.example.org can also access the cookie
- Omission: Only the current document's domain applies
- Create a cookie on example.org and omit domain specification
  - Cookie accessible only on example.org
  - dev.example.org cannot access the cookie

### Cookies - Path
**Path**
- Example) path=/home
- Cookie access only for pages within this path and its subpaths
- Typically set path=/ (root)
- Example)
  - path=/home specified
  - /home -> possible
  - /home/level1 -> possible
  - /home/level1/level2 -> possible
  - /hello -> impossible

### Cookies - Security
**Secure, HttpOnly, SameSite**
- Secure
  - Cookies are sent regardless of http or https
  - If Secure is applied, it's sent only over https
- HttpOnly
  - Prevents XSS attacks
  - Not accessible from JavaScript (document.cookie)
  - Used only for HTTP transmission
- SameSite
  - Prevents XSRF attacks
  - Cookie sent only if the request domain and the domain set in the cookie are the same
