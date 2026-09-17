# free

![](https://miro.medium.com/v2/resize:fit:1400/format:webp/1*kDnHbsX7kPSW_4_TxWQ00w.png)

This command allows you to check the memory usage status of the system.

Key metrics include `free` and `available`. `free` refers to memory that no one is using, while `available` is the amount of memory that can be allocated to an application (process).

`available` can be seen as the sum of `free` and `buff/cache`. Among `buff/cache`, `buff` represents the cache amount of the block itself held by block devices, and `cache` refers to the page cache.

- **Page Cache**: Simply put, when a process reads or uses a file stored on disk, it caches the file's contents in memory. The memory used for this is called page cache or cache memory.

By keeping the file contents of block devices in memory, unnecessary disk I/O can be reduced, thereby improving system performance. However, if an application lacks sufficient memory, page cache memory is released and returned. (`buff` represents the cache for the block itself held by block devices.)

**Swap is a feature that uses a portion of a block device as if it were memory.**
- Swap is a feature that uses a specific part of the disk as virtual memory when memory usage is insufficient. If server memory is low, a specific segment of memory is swapped out to disk, allowing other waiting processes to use memory. When the swapped-out memory is called again, it is brought back into the memory area; this is called swap in.
- Even if a swap in occurs, if there's no change in actual memory, it's better to leave the original disk content as is. Therefore, the memory that was swapped in remains on the actual disk, and this is called swap cached.
- However, continuous use of swap leads to performance degradation. This is because memory references by processes constantly trigger I/O to the block device, resulting in slower speeds and performance degradation.
- Depending on application characteristics, using swap can be either beneficial or detrimental. For applications like API servers, restarting quickly is often better for performance than using swap. For database servers, where downtime poses a significant risk, it's better to ensure availability even if it means using swap.

> However, in container deployment environments, systems like k8s typically have swap disabled by default. This can be attributed to the fact that restarting containers is very fast, making a restart more performant than using swap when memory issues arise.

In summary, `free` shows the system's memory usage status. If `buff/cache` memory is high, it indicates a situation with high I/O. If swap is in use, it suggests a memory shortage, requiring a memory upgrade.

If a server's memory is insufficient, symptoms typically appear in the following order: `buffer/cache memory released for use` -> `swap usage` -> `OOME` occurrence.
