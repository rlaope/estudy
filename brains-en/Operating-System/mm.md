# MMU, TLB, Page Table

When multiple processes are allocated memory, if a conflict occurs, problems can arise where one process's code overwrites or encroaches upon another's.

To solve this problem, the operating system allocates memory such that each process has its own independent memory space.

The overall picture is roughly like this:

```
cpu -> 가상주소 -> MMU -> TLB 확인 -> Page Table -> 물리주소 변환 -> 메모리 접근
```

Let's take a look.

### MMU(Memory Management Unit)

It is hardware that translates virtual addresses into physical addresses.

It exists either integrated within the CPU hardware or as an external chip layer, and it's convenient to think of it as an address translator.

The operating system makes it appear to each process as if it is using the entire memory as a single contiguous space.

In reality, it is physically separated. However, it is made to appear that way through the concept of virtual memory.

You can think of its role as reading these virtual memory addresses and converting them into physical addresses.

### Page Table

It is a table of virtual address-to-physical address mapping information managed by the operating system, and page sizes are typically in 4KB units.

| Virtual Page Number | Physical Frame Number | Protection Bit | Valid Bit           |
| ------------------- | --------------------- | -------------- | ------------------- |
| 0x00001             | 0x00A3                | RW             | 1                   |
| 0x00002             | 0x00A4                | RO             | 1                   |
| 0x00003             | —                     | —              | 0 (Page Not Loaded) |

As shown above, each process has its own page table, and if the valid bit is 0, it triggers a page fault (access denied) for protection.

Access permissions are also controlled by protection bits (read/write/execute).

Looking at the address translation process (internal MMU operation), a virtual address is typically composed as follows:

```
( Virtual Page Number \ Offset within Page)
```

1. The CPU generates a virtual address, e.g., `0x7FFE1234`.
2. The MMU extracts the page number: `0x7FFE`.
3. It searches the TLB (explained later).
4. If not found, it checks the page table and then identifies the physical frame number.
5. It caches it in the TLB (the TLB essentially acts as an address cache).
6. The physical address is assembled.
7. It accesses the actual memory.

The physical address is (physical frame number << page offset) + offset.

**Protection Mechanism**

The operating system and hardware establish the following rules:

1. Kernel Mode / User Mode Separation
   1. User programs are prohibited from accessing kernel areas.
   2. Requests are only possible via system calls.
2. Page Protection Bit
   1. Each page has read (R), write (W), and execute (X) permissions.
   2. Code areas are set to "read+execute", and data areas to "read+write".
3. Page Fault
   1. Upon incorrect address access, the CPU generates an interrupt, and
   2. The OS either terminates the process or loads the required page.

### TLB(Translation Lookaside Buffer)

It is a high-speed cache memory inside the MMU that stores recently translated virtual page -> physical frame information.

When the CPU accesses memory, it almost always queries the TLB first. Generally, the TLB hit rate is said to be over 95%.

<br>

### Hierarchical Page Tables

Also known as multi-level page tables, creating a 1:1 table for every virtual address would be too large.

Therefore, a hierarchical structure is used. For example, if a 32-bit address is divided into 4KB pages,

The total number of pages would be 2^32 / 2^12 = 1,048,576. If each page entry is 4 bytes, a 4MB table is required.

> (2^32 = total address space size; for a 32-bit address, 2^32 combinations are possible = 4GB)
> Page size is in 4KB units (2^12). This can be confusing if you don't know it.

Therefore, it is divided into 2-level or 4-level structures. Modern x86-64 uses 4-level page tables.

```
가상주소 = [P4 | P3 | P2 | P1 | Offset]
```
-> It searches down through the levels to find the physical address.
