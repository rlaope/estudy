# HTTP Status Codes

### Introduction to HTTP Status Codes

**Status Codes**
A `feature` that informs the client about the processing status of the request it sent in the response.

- 1xx (Informational) : Request received and being processed
- 2xx (Successful) : Request successfully processed
- 3xx (Redirection) : Further action is required to complete the request
- 4xx (Client Error) : Client error, server cannot fulfill the request due to malformed syntax, etc.
- 5xx (Server Error) : Server error, server failed to fulfill a valid request

**What if an unknown status code appears?**
- If the server returns a status code that the client does not recognize?
- The client interprets and processes it as the higher-level status code.
- No need to change the client even if new status codes are added in the future.
- Example)
  - 299 ??? -> 2xx(Successful)
  - 451 ??? -> 4xx(Client Error)
  - 599 ??? -> 5xx(Server Error)

<br>

### 1xx (Informational)
Request received and being processed
- Rarely used, so omitted
