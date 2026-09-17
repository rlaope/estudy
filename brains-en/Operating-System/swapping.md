# Swapping in OS

## Swapping
For a process to execute, its instructions and the data it accesses must be in memory. However, a process or a portion of a process can be temporarily moved out to a backing store during execution and then brought back into memory to continue execution.

![](https://media.geeksforgeeks.org/wp-content/uploads/20200406111356/Untitled-Diagram66-3.jpg)

Swapping can reduce the constraints of physical memory and increase the degree of multiprogramming. (degree of multiprogramming)

- Moving out of memory is Swap out
- Moving into memory is Swap in
This is in and out based on memory.


<br>

## Basic Swapping
It moves the entire process between main memory and the backing store. (Partial swapping can also occur.)
The backing store must be large enough to accommodate processes regardless of their size, for storage and subsequent access.
And it must be able to directly access these memory images.

When a process or part of it is swapped to the backing store, the data structures associated with the process must be recorded in the backing store.

Advantages of standard swapping
- Physical memory can be over-allocated to accommodate more processes than the actual physical memory.
- Processes that are idle or spend most of their time idle are good candidates for swapping.


## Swapping in Paging
In fact, standard swapping is mostly not used currently,
because moving an entire process between memory and storage is difficult.

Therefore, it is often used with only a small number of pages.

![](https://t1.daumcdn.net/cfile/tistory/232C453455A737A119)

## Swapping in Mobile Systems
It is generally not supported in mobile systems.
