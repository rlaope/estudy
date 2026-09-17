# Virtual Memory System

## Memory
- Memory is a device that stores programs and the data and code required for program execution.
- Memory is broadly classified into internal storage, which is main memory, and external storage, which is secondary memory.
  - DRAM, registers and caches within the CPU fall into the former category.
  - SSDs and HDDs fall into the latter category.

## Background of Virtual Memory
In early computers, the available RAM capacity had to be larger than the address space of the largest running application. Otherwise, the application could not be executed due to an "out of memory" error.

Later, computers attempted to solve the memory shortage problem using the overlay technique, which allowed programmers to specify that only a portion of an application should be loaded into memory for execution. However, this also could not solve the overall memory shortage problem. While programs using overlays used less memory than those that didn't, if the system didn't have enough memory for the program in the first place, the same out-of-memory error would still occur.

The more advanced virtual memory technique attempts to solve the problem by not focusing on how much memory is needed to run an application, but rather on the minimum amount of memory required to run it.
- This approach is possible because memory access is sequential and localized.
- If only a portion of the application is loaded into memory, where should the rest, which is not loaded into memory, reside? -> The answer is secondary storage, i.e., disk.
- The core of virtual memory is secondary storage.

<br>

## What is Virtual Memory
> Virtual memory is a technique that makes memory appear larger than its actual physical size. It was devised based on the idea that a process can execute even if its entire contents are not loaded into memory when it runs.

- When an application runs, only the portion necessary for execution is loaded into memory, and the rest of the application remains on disk. In other words, the disk acts as secondary storage for RAM.
  - Ultimately, it merges fast, small RAM with large, slow disk storage to make them function as a single large, fast storage (virtual memory).
- To implement virtual memory, the computer must have special memory management hardware -> specifically, the MMU (Memory Management Unit).

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbMHw35%2Fbtq5gwPeRPj%2FqZQ20xnWN3mIFoKPHDazh0%2Fimg.jpg)
- The MMU performs the functions of translating virtual addresses to physical addresses and protecting memory.
- When using an MMU, memory address translation is performed before the CPU accesses each memory location.
- However, translating each memory address individually from virtual to physical would increase the workload. Therefore, the MMU divides RAM into several parts (pages) and treats each page as an independent item.
- The task of remembering page and address translation information is a critical step in implementing virtual memory.

<br>

## Demand Paging
Demand paging means loading a process's data into memory only when the CPU requests it. In other words, not all data is loaded into memory from the beginning.

## Page Faults
A page fault is an interrupt that occurs when an attempt is made to access a page that is not present in physical memory. When a page fault occurs, the operating system resolves it and then resumes execution of the same instruction.

- It refers to the phenomenon that occurs when a program attempts to access data or code that exists in its address space but is not currently present in the system's RAM.
- When a page fault occurs, the operating system brings that data into memory, allowing the program to continue running as if no page fault had occurred.
- Frequent page faults significantly degrade operating system performance, so it's important to prevent them. A method to minimize page faults is the page replacement policy.
  - When memory is full, one of the existing pages is moved from physical memory to storage, and a new page is loaded into the newly freed physical memory space. The algorithm that determines which existing page to evict is the page replacement algorithm.

## TLB Translation Lookaside Buffer, Page Information Cache
A cache used to speed up the translation of virtual memory addresses to physical addresses. It stores recently performed virtual memory to physical address translation tables. When the CPU attempts to access memory using a virtual address, it first accesses the TLB to find the corresponding physical address. If no mapping exists in the TLB, the MMU translates the virtual address to the corresponding physical address from the page table and then accesses memory.
