# Validation Headers and Conditional Requests

### Summary
- Even if the cache validity period expires, if the server data has not been updated, respond with 304 Not Modified + header metadata only (no body).
- The client updates the cache's metadata with the response header information sent by the server.
- The client reuses the data stored in the cache.
- As a result, network download occurs, but only header information, which has a small capacity, is downloaded.
- A very practical solution.

<br>

- **Validation Headers**
  - Data used to verify if cache data and server data are the same.
  - Last-Modified, ETag
- **Conditional Request Headers**
  - Branching based on conditions using validation headers.
  - If-Modified-Since: Uses Last-Modified.
  - If-None-Match: Uses ETag.
  - If the condition is met, 200 OK.
  - If not met, 304 Not Modified.

#### Examples
- If-Modified-Since: Has the data been modified since then?
  - Example of unchanged data
    - Cache: 2020 November 10 10:00:00 vs Server: 2020 November 10 10:00:00
    - 304 Not Modified, only header data sent (Body not included).
    - Transfer size 0.1M (Header 0.1M, Body 1.0M).
  - Example of changed data
    - Cache: 2020 November 10 10:00:00 vs Server: 2020 November 10 11:00:00
    - 200 OK, all data sent (Body included).
    - Transfer size 1.1M (Header 0.1M, Body 1.0M).
- Cache adjustment in units less than 1 second (0.x) is not possible.
- Uses date-based logic.
- Cases where the date differs due to data modification, but the data result is identical because the same data was modified.
- If you want to manage separate cache logic on the server.
  - E.g., if you want to maintain the cache for changes that don't have a significant impact, like spaces or comments.

### ETag, If-None-Match

- ETag(Entity Tag)
- Assigns an arbitrary unique version name to cache data.
  - E.g.) ETag: "v1.0" , ETag: "asdfasdf"
- When data changes, this name is changed (Hash is regenerated).
  - E.g.) "aaaa" -> ETag: "bbbb"
- Simply send only the ETag; if it's the same, keep it; if different, retrieve it again!

<br>

#### First Request
```
HTTP/1.1 200 OK
Content-Type: image/jpeg
cache-control: max-age=60
ETag: "aaaaa"
Content-Length: 34012

adfas
```

#### Second Request - Cache Expiration
```
GET /star.jpg
If-None-Match: "aaaaa"
```
```
HTTP/1.1 304 Not Modified
Content-Type: image/jpeg
cache-control: max-age=60
ETag: "aaaaa"
Content-Length: 34012
```
No HTTP body.
Because the ETag is the same, the data has not been modified.

### ETag, If-None-Match Summary
- Simply send only the ETag to the server; if it's the same, keep it; if different, retrieve it again!
- Cache control logic is fully managed by the server.
- The client simply provides this value to the server (the client doesn't know the caching mechanism).
- E.g.,
  - The server maintains the same ETag even if the file changes during the 3-day beta open period.
  - All ETags are updated according to the application deployment cycle.
