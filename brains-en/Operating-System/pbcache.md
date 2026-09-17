# Page Cache, Buffer Cache

The OS's Page Cache and Buffer Cache are core mechanisms that use a portion of main memory (RAM) as storage to improve disk I/O performance.

Although these two concepts have been unified in modern Linux kernels, their origins and logical distinctions are still worth studying.

To distinguish them first, the Page Cache operates on a page unit (typically 4KB), while the Buffer Cache operates on a block unit (512B~4KB).

The Page Cache targets file data, is abstracted at a high level (file system layer), and is used to improve file read/write speeds.

The Buffer Cache targets physical blocks on the disk, is abstracted at a low level (block device layer), and is used for managing disk metadata and block alignment.

- **Page Cache**: Used when a user accesses a file via `read()` or `write()` system calls. It handles requests like "read from the 100th byte of the file."
- **Buffer Cache**: Used when directly accessing file system metadata (e.g., inode, Superblock) or disk devices. It handles requests like "read the 500th sector of the disk."

### Unified Page Cache (Unified Buffer Cache)

The biggest problem with the old architecture was **Double Caching**. When file data was read, it was stored in the Page Cache, and then the exact same data was stored again in the Buffer Cache at the block layer below, leading to wasted memory.

Modern Linux kernels (since Linux 2.4) adopted a unified Page Cache architecture to solve this.

- Structure: The Buffer Cache is no longer an independent memory area, but rather a **logical concept included within the Page Cache**.
- Operation: A single 4KB page in the Page Cache holds `buffer_head` structures pointing to four 1KB disk blocks, thereby performing the roles of both caches simultaneously.

```c
struct buffer_head {
    unsigned long b_state;          /* Buffer state flags (Dirty, Up-to-date, etc.) */
    struct buffer_head *b_this_page;/* Points to the next buffer_head within the same page (circular linked list) */
    struct page *b_page;            /* The physical page this buffer belongs to */

    sector_t b_blocknr;             /* Logical block number on disk */
    size_t b_size;                  /* Size of the block (e.g., 1024 bytes) */
    char *b_data;                   /* Pointer to where data starts within the page */

    struct block_device *b_bdev;    /* The block device this block belongs to (e.g., hard disk) */
    bh_end_io_t *b_end_io;          /* Callback function to be called upon I/O completion */
    void *b_private;                /* Private data for b_end_io */
    
    atomic_t b_count;               /* Reference count */
};
```

### Read/Write Flow at the Kernel Level

**Read Flow (Page Cache Look-up)**
1. A process calls `read()`.
2. The kernel checks if the corresponding offset of the file is in the Page Cache (referencing the `address_space` structure).
3. Cache Hit: Data is immediately copied from memory and delivered to the user. (No disk access)
4. Cache Miss: Data is read from disk, populated into the Page Cache, and then delivered. (At this point, Read-ahead is performed to pre-fetch surrounding blocks.)

**Write Flow (Write-back Policy)**
1. A process calls `write()`.
2. Data is written to the Page Cache, and the corresponding page is marked as a Dirty Page.
3. When certain conditions are met (e.g., time elapsed, low memory, explicit sync), the kernel's Background Worker (flusher thread) performs the actual write to disk.

### Observation

`free -m` allows you to check the amount of memory used by these two caches under the `buff/cache` entry.

`/proc/meminfo`: Detailed figures can also be checked through the `Cached`, `Buffers`, and `Dirty` entries.

`vmstat 1`: `bi` and `bo` represent block in and block out, respectively, allowing real-time monitoring of data movement between the cache and disk.
