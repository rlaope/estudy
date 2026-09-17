# Cache and Conditional Request Headers

- Cache-Control: Cache Control
- Pragma: Cache Control (Backward Compatibility)
- Expires: Cache Expiration Period (Backward Compatibility)

### Cache-Control
**Cache Directives**
- Cache-Control: max-age
  - Cache validity period, in seconds
- Cache-Control: no-cache
  - Data can be cached, but always revalidate with the origin server before use
- Cache-Control: no-store
  - Contains sensitive data, so it must not be stored (use in memory and delete as quickly as possible)

### Pragma
**Cache Control (Backward Compatibility)**
- Pragma: no-cache
- HTTP/1.0 backward compatibility

### Expires
**Specify Cache Expiration Date (Backward Compatibility)**
- expries: Mon, 01 Jan 1990 00:00:00 GMT
- Specifies the cache expiration date as an exact date
- Used since HTTP 1.0
- Now, the more flexible Cache-Control: max-age is recommended
- If used with Cache-Control: max-age, Expires is ignored
