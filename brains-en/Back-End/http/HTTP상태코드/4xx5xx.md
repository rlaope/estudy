# 4xx - Client Errors, 5xx - Server Errors

### Client Errors
- The server cannot fulfill the request due to malformed syntax in the client's request.
- The error originates from the `client`.
- Important: Since the client is already sending an incorrect request or data, retrying the exact same request will fail.

### 400 Bad Request
**The server cannot process the request because the client sent a malformed request.**

- Errors in request syntax, message, etc.
- The client should review or resend the request content.
- Example: When request parameters are incorrect or do not match the API specification.

### 401 Unauthorized
**The client needs authentication for the requested resource.**

- Not authenticated.
- When a 401 error occurs, the response should include the `WWW-Authenticate` header explaining the authentication method.
- Note
  - Authentication: Verifying who you are (login).
  - Authorization: Granting permissions (e.g., access rights to specific resources like ADMIN privileges; authorization requires authentication).
  - Although the error message is 'Unauthorized', it means 'unauthenticated' (the name is a bit misleading).

### 403 Forbidden
**The server understood the request but refused to authorize it.**

- Typically occurs when authentication credentials exist, but access permissions are insufficient.
- Example: A non-admin user is logged in but tries to access an admin-level resource.

### 404 Not Found
**The requested resource could not be found.**
- The requested resource does not exist on the server.
- Or, when the client attempts to access a resource for which they lack sufficient permissions, and the server wishes to hide the existence of that resource.

<br>

### 5xx - Server Errors
- An error occurred due to a server issue.
- Since the problem is with the server, retrying might succeed (e.g., if the server recovers).

### 500 Internal Server Error
**An error occurred due to a server issue; if it's ambiguous, it's a 500 error.**
- An error occurred due to an internal server problem.
- If it's ambiguous, it's a 500 error.

### 503 Service Unavailable
**Service Unavailable.**

- The server is temporarily unable to handle the request due to temporary overload or scheduled maintenance.
- It may also send a `Retry-After` header field indicating when the service will be restored.

Ideally, 5xx errors should not occur. They indicate genuine server problems (e.g., `NullPointerException`, database downtime, etc.).
