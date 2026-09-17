# Paging, Fragmentation, TLB

Let's review what we learned about Memory - Address, Contiguous allocation, and MMU.

Contiguous allocation has the advantage of making the MMU very simple.

This is because it only needs to check for addition and the upper limit. However, as mentioned in the previous post, it is not used in current systems.

Today, we'll find out why.

**Disadvantages of contiguous allocation**

It's common for multiple computer processes to run simultaneously.

And those processes will, of course, all have different sizes and execution times.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FL1WmC%2FbtrtqOLUSGk%2FUJG7R7aIcVmDAcZmfLNweK%2Fimg.png)

First, process1 is executed. Subsequent processes will be placed contiguously.

This is because contiguous allocation means that if it's logically contiguous, it's also physically contiguous.

In other words, actions like splitting process1 to put it into empty memory or reordering it by splitting are not possible.

It must be placed contiguously.

The image above shows processes 1 to 3 placed contiguously in empty memory. The memory is full after process3 entered last.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FdjvT3a%2FbtrtytlIncM%2FfL2qbcTlApl23DiBsrAGVK%2Fimg.png)

Now, process 3 terminates, and its memory space becomes empty. This space is called a HOLE.

Processes 4 and 5 entered the HOLE. It would be ideal if they could fit perfectly into the empty memory space, but that's not the reality.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fu0E1s%2Fbtrtwqje11U%2FhNKzvHM6MSK8FWHqJ50h81%2Fimg.png)

Even after process5 is placed, there's a small amount of empty memory, but not enough for process6 to execute. Space would become available if process1 finished, but even if the other processes finish, there won't be enough space unless 2, 4, and 5 all finish at once. This is called fragmentation.

Fragmentation is divided into internal fragmentation and external fragmentation.

The example above is external fragmentation. Fragmentation not only means "fragmentation" but also "piece," so it's named because memory remains but cannot be used, being broken into pieces.

To summarize, **external fragmentation is a problem where, during repeated memory allocation and deallocation, many unused empty memory fragments are created, resulting in sufficient total memory space but no memory space that can actually be allocated for process execution.**

To solve this problem, compaction exists.

<br>

### Compaction

The method to solve the external fragmentation problem is simple: reorganize the in-use memory spaces to create room for the memory space required by the process that needs to run.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FS7C1Y%2FbtrtyeoLjtc%2FaaNEfTTjgF0o90qNskydyk%2Fimg.png)

Above, processes 2 and 5 were moved to create the necessary memory space for process6.

However, this method involves temporarily copying processes 2 and 4 and then bringing them back.

This means another storage device is needed. While we use SSDs now, in the days before SSDs, HDDs would have been used, which would be very slow.

In summary, compaction is the process of making empty memory spaces contiguous.

While compaction can reduce external fragmentation, it introduces an I/O problem, which is the process of copying and retrieving memory. This process incurs overhead, leading to the development of a better method: paging.

Even before paging, much effort was put into making the above methods efficient. The question was how to fit processes efficiently, leading to the birth of three concepts: first-fit, best-fit, and worst-fit.

- first-fit: To find a hole to allocate memory, search sequentially from the beginning; when a hole is found, allocate memory to the first hole.
- best-fit: Try all available holes and place the process in the hole that best matches its size.
- worst-fit: Place the process in the largest remaining hole. This way, the remaining hole will be large enough for other processes to use.

The methods described above allocate memory based on process size. This is called **variable partitioning**.

Because not all processes are the same size, regardless of how they are placed, the problem of external fragmentation cannot be prevented.

The solution that emerged is paging, and **paging allocates memory by dividing it into fixed-size blocks.**

This method is called fixed partitioning. As mentioned above, external fragmentation occurs due to variable partitioning, while internal fragmentation occurs due to fixed partitioning. Let's explore this further.

<br>

### Paging

1. pages: blocks obtained by dividing logical memory into fixed sizes.
2. frames: blocks obtained by dividing physical memory into fixed sizes.

First, the logical address space of a process is divided into several parts called pages, and the physical address space is divided into several parts of the same size called frames. Pages and frames must all be divided into the same size.

The paging technique maps these divided pages and frames.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FHS9vo%2FbtrtwKhADHK%2FETbN4QumvE5JIcqc7vq5hk%2Fimg.png)

In the image above, the left side is the logical memory address, and the right side is the physical memory address. As you can see, both are divided into the same size.

Here, a specific logical memory address value can be expressed as the first offset of page 1, and a specific physical address value can also be expressed as the first offset of frame 1.

> What is an offset?
> It can be thought of as an index from the beginning of an object within that object. In memory, it is said to be the value added to the first address, which serves as a reference, to create the second address. For example, if object A contains the string "abcdef", 'c' can be considered to have an offset of 2 from the start of A.

<br>

### Internal Fragmentation

Now let's look at internal fragmentation, a problem with fixed partitioning.

Since we've divided into uniform sizes and don't need to place them contiguously, external fragmentation no longer occurs.

So, can we now allocate memory without any empty holes?

When dividing into uniform sizes, it's possible that the memory size doesn't divide perfectly by the page size. If a memory block is slightly larger than the block size, there will be leftover space. This situation is called Internal Fragmentation.

**+ How are pages mapped to frames?**

First, let's learn about the page table. A table that records information about the frame each page will be mapped to is called a **page table**. That is, a page table has as many entries as there are pages.

For example, if a system has a physical address space of 1024KB (1024 * 1024 bytes) and each page has a size of 1024 bytes, then the page and frame sizes must be the same, meaning there will be a total of 1024 pages in the current physical space.

Then, the offset addresses that can be held within a single page are 0 to 1023, and frames also have offsets from 0 to 1023 since they are the same size. The offset information is passed directly.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FcfUa30%2FbtrtsVLdCh6%2FISkCxokuB9ENSYlGpuZEd0%2Fimg.png)

So, what data is contained in the page table?

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fxqzv0%2FbtrtvsaGFKc%2FeRG8oux5OUyArn0OeKOt2K%2Fimg.png)

The following additional information is included:

- reference bit (access bit): Contains information indicating whether the page has been referenced.
- valid bit (present bit): Contains information indicating whether the current page is in physical memory or in the paging file.
- dirty bit (modified bit): Contains information indicating whether the current page has been modified.

Based on the content so far, a page table exists for each process, and each page table has as many entries as the number of pages a process has. Each entry contains additional data, including information about the mapped frame.

Now, let's look at the problems with this page table.

**Regarding the large page table problem (page table size issue)**

First, we said that a page table exists for each process. Considering the maximum size of a process,

For example, if there is 2^32 bits of logical memory, each page size is 2^12 bits, and the size of one entry in the page table is 4 bytes, then a process's page table would need to hold approximately 2^20 page entries.

If each page entry is 4 bytes, then 2^20 * 4 bytes = approximately 4MB. If we assume 10 processes are running on the computer, the page table size alone would already be 40MB.

How can these problems be solved?

<br>

### Multi-level paging

Multi-level paging is a technique where the page table itself is paged again. For example, a typical paging technique would write a page table like "data is on the 1st line of page 38 out of 1000 pages."

Applying multi-level paging here would tell you "data is on the 1st line of the second page within the 3rd chapter." This example applies two-level paging, as one layer, "chapter," has been added.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fts65f%2Fbtrtwla29e5%2FkBHpx9nGN4L7IYEYt1szw0%2Fimg.png)

Multi-level paging reduces memory issues because it doesn't require carrying all page tables. However, since access must follow the order of chapter -> page -> offset, if there are too many layers, memory access overhead can occur.

<br>

### Hashed Page Table

If the address space is larger than 32 bits, a hashed page table, where the hash value becomes the virtual page number, is often used. Each entry in a hashed table has a linked list, and each hash element has three fields (virtual page number, mapped page frame number, and a pointer to the next element in the linked list).

The table is examined using the hashed result of 'p', the page number of the logical address space, and then the corresponding physical memory is accessed. Since one list can be used for the same hash value, the problem can be solved.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbsQszU%2FbtrtsECdmGe%2FN21rLLQtTRxevabRq9LXf1%2Fimg.jpg)

<br>

### Inverted Page Table

This is a table structure created from the perspective of physical memory addresses. Previous tables were all based on logical address space, mapping divided pages sequentially into the page table.

Therefore, while previous page tables contained information about which frame a page was in, they now also include the process ID (pid) value.

An inverted page table has the advantage that only one overall table is needed, rather than one page table per process. However, its larger size means search times are longer. Additionally, if pid information is missing, there's the inconvenience of having to go to the original page table to update and modify it.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2F9SPew%2FbtrtwKaTQrj%2Fod9O7zp4KqcBod1AnZHRxk%2Fimg.png)

<br>

### Memory Access Overhead Problem

Where should the page table be stored to allow for the fastest referencing and address translation?

It's the MMU. However, putting 4MB of memory into the MMU is impractical.

Ultimately, it must be stored in RAM, not the CPU, which means accessing memory twice for a single memory access.

One must access the page table to read the mapped frame information, and then access the actual memory to read the data.

This process incurs overhead... How can this be solved?

By using cache memory. Even if the page table is large, not all of it is actually used, so this point is exploited.

Programs have a characteristic called locality, meaning frequently used items are used often, and rarely used items are almost never used.

This characteristic is leveraged by caching the principle that only a few of the actually referenced pages are used.

<br>

### TLB(Translation Look-aside Buffer)

The method that emerged from this is the TLB, an Associative Memory. While general memory accesses an address and returns data, Associative Memory uses an index to store mapped data, allowing it to quickly return the data on subsequent identical accesses. This can be seen as the same concept as a cache.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbqLuOE%2FbtrtzafEYWI%2Fig1nyCAdrXxtujVo3MGt2k%2Fimg.jpg)

However, as shown above, even when using the TLB to quickly find frame mapping information, if the frame number is not yet in the TLB, it goes back to the page table to get the frame number and then stores it in the TLB.

This is because the TLB has a fixed size and cannot store data for all frame numbers.
