# Memory Fragmentation

## Memory Fragmentation

A state where memory space in RAM is divided into small pieces, and although there is sufficient available memory, allocation is impossible.

### Internal Fragmentation
-> Segmentation Variable size

A situation where memory space is wasted because a larger amount of memory than a Process actually needs is allocated during memory allocation.

(For example, if the OS allocates 4KB for a program, but only 1KB is actually used, 3KB of internal fragmentation occurs.)

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Ft1.daumcdn.net%2Fcfile%2Ftistory%2F99811C435B19F90F02)

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Ft1.daumcdn.net%2Fcfile%2Ftistory%2F9905AB435B19F9100D)

### External Fragmentation
-> Paging Fixed size

When memory allocation and deallocation operations are repeated, many unused small memory blocks exist in between, making it impossible to actually allocate memory even though the total memory space is sufficient.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Ft1.daumcdn.net%2Fcfile%2Ftistory%2F993EFB435B19F90F07)

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Ft1.daumcdn.net%2Fcfile%2Ftistory%2F993EFB435B19F90F07)

<br>

## Solutions for Memory Fragmentation

1. Paging - Uses virtual memory, resolves external fragmentation

Secondary storage and virtual memory divided into blocks of the same size are called pages, and RAM divided into blocks of the same size as pages are called frames.

**Paging Technique**
A technique that moves unused frames to pages (virtual memory) and moves necessary memory to frames in page units.

To map pages and frames, a Page Mapping process is required, which creates a Paging table.

2. Segmentation - Uses virtual memory, resolves internal fragmentation

Segmentation divides virtual memory into logical segments of different sizes, allocates them to memory, and converts them into physical memory addresses.

Each segment is stored in a contiguous space.

Similarly, a Segment Table is required for mapping.

However, because memory is allocated exactly as much as the process needs, internal fragmentation does not occur, but external fragmentation can arise when memory is deallocated in the middle.
