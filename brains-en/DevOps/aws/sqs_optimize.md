# Optimization Improvements for Amazon SQS Speed and Scaling

Amazon SQS, like many AWS services, is implemented using a collection of internal microservices.

- **Customer Frontend**: The customer-facing frontend accepts, authenticates, and authorizes direct API calls such as `CreateQueue` and `SendMessage`. It then routes each request to the storage backend.
- **Storage Backend**: This internal microservice is responsible for persisting messages sent to standard (non-FIFO) queues. Using a cell-based model, each cluster contains multiple hosts, and each customer queue is assigned to one or more clusters, with each cluster managing numerous queues.

![](https://d2908q01vomqb2.cloudfront.net/da4b9237bacccdf19c0760cab7aec4a8359010b0/2024/03/25/sqs_fleets_3.png)

### **Connections - Old and New**

In the original implementation, a connection was used for each request between these two services. Since each frontend had to connect to multiple hosts, a connection pool was necessary, and there was an inherent risk of eventually reaching a limit on the number of open connections. While simply adding more hardware to scale out could address such issues, it's not always the best approach.

Ultimately, it was decided to develop and use a new proprietary binary framing protocol between the customer frontend and the storage backend. This protocol can multiplex multiple requests and responses over a single connection using a 128-bit ID and checksums to prevent crosstalk. Server-side encryption also provides an additional layer of protection to prevent unauthorized access to queue data.

### **Verifying the Impact**

The new protocol entered production earlier this year and, as of this writing, has processed 744.9 trillion requests.

The scalability cliff (limit of scale-out) has been eliminated, and we are exploring other ways to leverage this protocol.

In terms of performance, the new protocol reduced data plane latency by an average of 11 percent and by 17.4 percent at the p90 level. This transformation not only enhances SQS's own performance but also benefits services built on top of SQS.

For example, the internal time consumed before messages sent via SNS are delivered has decreased by 10%. Finally, due to the protocol change, the existing SQS host fleet (a mix of x86-based and Graviton-based instances) can now handle 17.8 percent more requests than before.

### Conclusion

In conclusion, we were able to enhance the overall performance of SQS through our proprietary binary framing protocol.
