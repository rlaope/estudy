# CloudFront

It is a CDN service provided by AWS.
It uses multiple edge locations, allowing users to access the closest one to minimize latency.

> CDN (Content Delivery Network): A geographically distributed network of servers and their data centers that helps deliver content to users with minimal latency.

### Advantages
- It minimizes latency by utilizing edge locations, allowing users to access the closest one.
- By leveraging caching, if the information is available at an edge location, it doesn't need to access S3 again. This reduces S3 costs.

> What is an Edge Location: It is a point of presence for AWS's CDN services to deliver various services at the fastest speed (caching).
