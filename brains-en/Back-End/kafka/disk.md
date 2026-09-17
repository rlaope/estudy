# Kafka Disk I/O

Kafka stores data on the broker's local disk. Despite this, Kafka is fast. What's the reason?

### Sequential I/O

Disk I/O can be slow or fast depending on how it's used.

As shown in the figure below, **the speed of sequentially accessing data on disk** is 150,000 times faster than disk random access and even faster than memory random access.

![](https://miro.medium.com/v2/resize:fit:1400/format:webp/1*XxYnFy3pc918ljOWJ8IOlg.png)

Disk I/O Speed Comparison

Modern OSes provide technologies like `read-ahead` and `write-behind` to support faster sequential read/write operations. Kafka **stores data in a message queue fashion, which benefits from sequential I/O, providing fast performance.**

<br>

### Page Cache

To reduce disk seeks and increase throughput, modern operating systems have become more aggressive in using main memory for **Page Cache (disk cache)**. All disk reads/writes go through the page cache, and since it's managed by the OS rather than by users or applications, it can store about twice the cache without redundant storage in user and kernel space, and applications can quickly benefit from the cache even after restarting.

![](https://miro.medium.com/v2/resize:fit:1400/format:webp/1*rjlyW4hhBaJGqVJfKu7dSw.png)

Page Cache

Kafka runs on the JVM, and storing objects in Heap memory is very expensive, with the disadvantage that GC slows down as heap memory grows. In conclusion, using the **File System and page cache**, which benefit from sequential read/write, can achieve better performance than using memory caches or other structures.

<br>

### Zero Copy

![](https://miro.medium.com/v2/resize:fit:1400/format:webp/1*wb6XhkM0vKf9YxyEHLlvsw.png)

Typical Data Transfer Process

Generally, **the procedure for transferring data over a network** is as follows.

1. The OS reads data from disk and stores it in the page cache in the kernel space.
2. The application reads data from the page cache into user space.
3. The application writes data to the socket buffer in the kernel space.
4. The OS copies data from the socket buffer to the NIC buffer and transmits it over the network.

The above process involves unnecessary system calls. The `sendfile` function provided by the OS allows direct copying from the page cache in the kernel space to the NIC buffer, enabling **efficient data transfer.**

![](https://miro.medium.com/v2/resize:fit:1400/format:webp/1*XNOO-4gj7xOwSzbSjeQ4jw.png)

Zero Copy

Kafka uses this **Zero Copy** technology to effectively transfer data by reducing unnecessary copies and system calls when messages are produced/consumed. While there isn't a significant performance impact for a single message, the difference in performance becomes noticeable when millions of messages are transferred through Kafka.

In addition to these, there are other features like batch transfers and compression that are worth exploring.
