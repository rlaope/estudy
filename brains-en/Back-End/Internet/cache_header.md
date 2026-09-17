# Cache Validation Headers, Last-Modified, If-Modified-Since

![](https://velog.velcdn.com/images/mmmdo21/post/2a726ec3-8999-4738-9ab7-ac63daba5e09/image.png)

When a cache's validity period expires, it sends another request to update the cache with new data.
  
What if the cache's validity period has passed, but the server data hasn't changed, and it's still acceptable to use the existing data? Is there a way to validate and use it?

## Last-Modified and If-Modified-Since

### First Request
![](https://velog.velcdn.com/images/mmmdo21/post/23418486-288d-4567-85ce-aa55d4b46e4d/image.png)

You can determine the cache's modification time using the Last-Modified validation header.
  
Last-Modified includes information about the last time the data was modified in the header.
  
As a result, when the response is stored in the cache, the data's last modification date is also saved.

### Second Request

Even if the cache's validity period expires, you can make a conditional request using the If-Modified-Since header.

![](https://velog.velcdn.com/images/mmmdo21/post/bf54179e-24c7-4735-b643-2ac46fbfbb6d/image.png)

![](https://velog.velcdn.com/images/mmmdo21/post/6e8f6a16-dd0e-4665-836f-c32c1bbbc42a/image.png)

If the data has not been modified when compared to the server's last modification date for that resource, the server informs this in the response message.
  
In this case, the HTTP Body is absent from the response data, and the status code is 304 Not Modified, indicating that nothing has changed.
  
Since the body is omitted from the transmitted data, only 0.1MB containing just the headers is sent.
  
After receiving this response, the client updates its cache, and it becomes valid again for a certain period (60 seconds).

### Summary
- What if the cache's validity period expires, but the server data hasn't been updated?
  - 304 Not Modified + only header metadata in response (no body)
  - The client updates the cache's metadata with the response header information sent by the server.
  - The client reuses the data stored in the cache.
  - As a result, network download occurs, but only a small amount of header information is downloaded (a very practical solution).

> Metadata: Data that describes data

### Drawbacks
Cannot adjust cache in units less than 1 second
  
Uses date-based logic
  
If data is modified, resulting in a different date, but the actual data content remains the same (e.g., modifying the same data to yield the same result).
  
When the server wants to manage separate cache logic
  
e.g., When you want to maintain the cache for changes that don't significantly affect the content, such as spaces or comments.
