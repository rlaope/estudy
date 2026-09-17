# Spring Batch Technical Trade-offs

Spring Batch is a framework supported by Spring for developing batch applications for large-scale data processing.

Today, we will explore the technical trade-offs of the Spring Batch framework, when to apply it,
and its differences from real-time API methods.

First, the trade-offs of Spring Batch are likely as follows:

1. Complexity
2. Simplicity vs. Flexibility

Spring Batch may require various components and configurations to support advanced features, which can lead to a steep learning curve.

Furthermore, for some simple batch tasks, the flexibility of Spring Batch might not be necessary, potentially introducing unnecessary complexity.

Typically, this aspect represents a trade-off in terms of learning time and flexibility rather than performance. Let's explore the performance differences compared to processing via APIs or queries.

<br>

## API vs Batch

API processing, batch processing, and data cleanup via queries are used for different purposes and in different situations.

### Real-time Processing vs. Batch Processing

Data cleanup using APIs or queries can be done in real-time or immediately. It is suitable for real-time data updates and queries. And it runs at irregular intervals.

On the other hand, batch processing is used to periodically process and clean up large-scale data and runs at regular intervals.

### Processing Time, Resource Usage, and System Load

Batch processing has the advantage of efficiently managing processing time and resource usage compared to real-time processing (more details below).

Furthermore, processing large-scale data in real-time can lead to system overload. In contrast, batch processing is typically executed during off-peak hours, which helps distribute the system load.

### Data Consistency

API and query methods, being real-time processing, can reflect real-time updates, but this requires additional management to maintain consistency.

On the other hand, batch processing is executed periodically, making it easier to maintain consistency. (Of course, batches can also run multiple times concurrently; I will discuss solutions for this in another post.)

To summarize, the choice is between real-time processing and processing at regular intervals.

When processing large volumes of data, system load and resource usage must be considered.

- API -> Real-time processing / Relative speed / + Easier QA
- Batch -> Post-processing / Absolute speed / + More complex QA

However, Spring Batch does not always guarantee data consistency. This is explained in detail in [[Spring Batch 데이터 일관성 유지 방법]].

### When should Spring Batch be used?

Batch processing is needed when **tasks run at regular intervals and involve processing such large volumes of data that real-time processing is difficult.**

Situations requiring large-scale data processing == situations requiring batch processing.

In Spring Batch, queries are always performed based on `Paging` or `Cursor`.
