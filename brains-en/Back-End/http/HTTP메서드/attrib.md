# Properties of HTTP Methods

#### Properties of HTTP Methods
- Safe Methods
- Idempotent Methods
- Cacheable Methods

**Safe**
- Calling it does not change the resource.
- Q: What if continuous calls accumulate logs and cause a failure?
- A: Safety only considers the resource itself. It does not account for such side effects.

**Idempotent**
- f(f(x)) = f(x)
- Whether called once, twice, or 100 times, the result is the same.
- Idempotent Methods
- GET: Whether retrieved once or multiple times, the same result is returned.
- PUT: Replaces the result. Therefore, even if the same request is made multiple times, the final result is the same.
- DELETE: Deletes the result. Even if the same request is made multiple times, the deleted result is the same.
- POST: Is not idempotent. Calling it twice can result in duplicate payments.

- Usage
  - Automatic recovery mechanism
  - When the server fails to provide a normal response due to a TIMEOUT, etc., it serves as a basis for determining whether the client can retry the same request.
- Q: What if another party modifies the resource during a retry?
  - User1: GET > username : A age : 20
  - User2: PUT > username : A age : 30
  - User1: GET > username :A age : 30 -> Retrieves data changed by User2's action.
- A: Idempotency does not account for resources being changed by external factors in the interim.

**Cacheable**
- Can the response result resource be cached and used?
- GET, HEAD, POST, PATCH are cacheable.
- In practice, only GET and HEAD are typically used for caching.
- For POST and PATCH, the request body content must also be considered as part of the cache key, which is not easy to implement.
