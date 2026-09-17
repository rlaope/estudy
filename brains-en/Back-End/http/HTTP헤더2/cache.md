# Cache Basic Operation

### When Cache is Not Used
- Even if data hasn't changed, it must be continuously downloaded over the network.
- Internet networks are very slow and expensive.
- Browser loading speed is slow.
- Slow user experience.

### Cache Applied

**First Request**
```
HTTP/1.1 200 OK
Content-Type: image/jpeg
cache-control: max-age=60
Content-Length: 34012

dafdsfasfsad
```

**Cache Applied**
- Thanks to caching, there's no need to use the network during the cacheable period.
- Reduces expensive network usage.
- Browser loading speed is very fast.
- Fast user experience.

**Third Request - Cache Expiration**
When a request is made, the browser cache validates the expiration time.
If the time has passed, it downloads the data again, like the first request, and stores it in the cache.
- If the cache expiration time is exceeded, the data is re-queried from the server, and the cache is updated.
- At this point, a network download occurs again.
