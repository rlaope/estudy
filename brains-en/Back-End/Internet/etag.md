# Cache Validation Headers ETag, If-None-Match

ETag and If-None-Match validation headers offer a simpler approach than Last-Modified and If-Modified-Since.
  
ETag can be used when the server wants to fully control caching.

![](https://velog.velcdn.com/images/mmmdo21/post/61357d79-0876-4cda-8329-d99a17bb7b03/image.png)

### How it Works
The server responds by writing the ETag in the header.
  
The client's cache stores that ETag value.

![](https://velog.velcdn.com/images/mmmdo21/post/d7003dc2-5752-4520-8f4a-288244010491/image.png)

If the cache has expired and a new request needs to be made, the client sends an If-None-Match header with the ETag value for validation in the request header (conditional request).

![](https://velog.velcdn.com/images/mmmdo21/post/bb79c5d9-a157-41ad-ad62-d04211170032/image.png)

If the data on the server has not changed, the ETag remains the same, making If-None-Match false.
  
In this case, the server responds with 304 Not Modified, and there is no HTTP body.
  
The browser cache reuses the response result and updates the header data.

![](https://velog.velcdn.com/images/mmmdo21/post/2ff52843-4f9f-4f8e-aedb1d5f1501/image.png)

### Summary

![](https://velog.velcdn.com/images/mmmdo21/post/1410fd25-36c6-457d-b91c-fddd56ddc415/image.png)
