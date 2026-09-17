# buffer_head

The `buffer_head` structure in the Linux kernel acts as a bridge connecting file system blocks and memory pages.

The main fields defined in the actual Linux source code are as follows:

```c
struct buffer_head {
    unsigned long b_state;          /* Buffer state flags (Dirty, Up-to-date, etc.) */
    struct buffer_head *b_this_page;/* Points to the next buffer_head within the same page (circular linked list) */
    struct page *b_page;            /* The physical page this buffer belongs to */

    sector_t b_blocknr;             /* Logical block number on disk */
    size_t b_size;                  /* Size of the block (e.g., 1024 bytes) */
    char *b_data;                   /* Pointer to the start of data within the page */

    struct block_device *b_bdev;    /* The block device this block belongs to (e.g., hard disk) */
    bh_end_io_t *b_end_io;          /* Callback function to be called upon I/O completion */
    void *b_private;                /* Private data for b_end_io */
    
    atomic_t b_count;               /* Reference count */
};
```

To understand the relationship between a page and a buffer head, if the system page size is 4KB and the file system block size is 1KB, then one page will contain four blocks.

In this case, the kernel arranges data as follows:

1. `struct page`: Manages 4KB of physical memory; the first field of its private member points to a `buffer_head`.
2. `struct_buffer_head` list: Each `buffer_head` contains information about a 1KB region.
   1. Through the `b_this_page` field, the four `buffer_head`s are linked together in a **Circular Linked List**.
   2. `b_data` points to a specific offset (0, 1024, 2048, 3072) within that page.

`b_state`: This is a bitmask indicating the current state of the buffer.
- `BH_Dirty`: This data has been modified and needs to be written to disk.
- `BH_Uptodate`: Data has been successfully read from disk and matches memory.
- `BH_Mapped`: The actual block number on disk (`b_blocknr`) is allocated.

`b_this_page`: This is a very important field. When multiple blocks exist within a single page, they are chained together, allowing the kernel to ascertain the state of all blocks within that page by simply finding one page.

`b_blocknr`: This is not a number managed by the file system abstraction layer, but rather indicates the **sector-unit position** on the corresponding disk device.

While operating systems prefer file units, hardware lower layers use smaller units called blocks, necessitating this kind of management.

From the perspective of the page cache, it checks if data from 0-4KB of File A is in memory.

From the perspective of the buffer cache's `buffer_head`, it checks if block 800 of disk `/dev/sda1` is in memory.

Thanks to the unified page cache structure, we can efficiently manage both the contents of a file and the physical block status on the disk where that file is stored, all through this `buffer_head`.

### Note

Recently, in Linux kernel 5.x and later, there has been a movement to efficiently handle larger amounts of memory by introducing the `iomap` structure and the concept of Folios to reduce the overhead associated with `buffer_head`.

However, `buffer_head` still plays a pivotal role in the core mechanisms of popular file systems like EXT4.
