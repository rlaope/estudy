# Page Replacement Algorithms: FIFO, LRU, LFU, MFU, NUR

## Page Replacement Algorithms

Operating systems load and use only a portion of a program in main memory to execute programs larger than main memory.

This is called virtual memory technique.

In an operating system that manages memory using paging, when a required page is not loaded into main memory (page fault), the method of deciding which page frame to select and replace is called a **page replacement algorithm**.

### Types
- OPT - Optimal: Replaces the page that will not be used for the longest time in the future.
- FIFO: First In First Out: Replaces the page that entered first.
- LRU - Least Recently Used: Replaces the page that has not been used for the longest time.
- LFU - Least Frequently Used: Replaces the page with the smallest reference count.
- MFU - Most Frequently Used: Replaces the page with the largest reference count.
- NUR - Not Used Recently: Replaces pages not used recently.

<br>

## OPT (Optimal): Replaces the page that will not be used for the longest time in the future

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FS8lUX%2Fbtq9JNyN39f%2FoYoWX91sjF34LkuFfuJxrk%2Fimg.png)

- Virtually ideal.
- Requires knowing in advance which pages the process will use in the future -> impossible.
- Used for comparative research purposes.

## FIFO First In First Out: Replaces the page that entered first

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FU5nSm%2Fbtq9PUpYAl8%2FWtyueVDWhp6E2nLGbNHYWK%2Fimg.png)

- Evicts the page that was loaded into memory first.
- Simple and suitable for initialization code.
- Stores the entry time or the order of entry in a queue.
- Intuitively, as the number of frames increases, the number of page faults decreases.
- Belady's Anomaly (FIFO anomaly)
In practice, phenomena where this is not the case can occur.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbBk7Bp%2Fbtq9Vka7iP3%2FOyRlrUsTMJ4owdIJDC1KL0%2Fimg.jpg)  
![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fukhyy%2Fbtq9NROjAZc%2Fy1yPHPRuZenlX3VTUBIq01%2Fimg.jpg)

## LRU (Least Recently Used): Replaces the page that has not been used for the longest time

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fb1csvE%2Fbtq9IZlYnx0%2FwbSZzZfsBkbmQ80nnf9LdK%2Fimg.png)

Assumption: If data has not been used for the longest time, it is less likely to be used in the future.

Considers the property of temporal locality (the tendency for recently referenced pages to be referenced again in the near future).

Stores information about when pages were used to remove data that has not been referenced for the longest time (requires a counter for each page).

Can be implemented with a queue; remove used data from the queue and move it to the top, and if frames are insufficient, delete the data at the bottom.

### Disadvantages
Significant overhead occurs because the reference time of each page must be recorded every time the process accesses main memory.

Requires separate hardware such as counters, queues, or stacks.

> Counter: A logical clock existing for each page, which is cleared to 0 every time the page is used, then increments time to replace the oldest page.

<br>

## LFU (Least Frequently Used): Replaces the page with the lowest reference count

Decides which page to replace based on its reference count.

While LRU only reflects the most recent reference time, LFU can consider long-term reference patterns through reference counts.

### Disadvantages
Most recently loaded pages might be replaced, more complex to implement, significant overhead.

<br>

## MFU: Replaces the page with the highest reference count
Assumption: The most frequently used page will not be used in the future.

<br>

## NUR Not Used Recently, Clock Algorithm

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FqfdHU%2Fbtq9PTLnM9o%2F7Xg5UGO0UYs3qeXjJwTiW1%2Fimg.png)

An algorithm that approximates LRU by replacing pages not used recently.

Does not guarantee that the replaced page's reference time is the oldest.

Reasonable performance with low overhead.

Random selection within the same group.

Two bits are used for each page: a Reference Bit and a Modified Bit (Dirty Bit).

- Reference Bit: 0 when the page has not been referenced, 1 when it has been referenced (all reference bits are periodically reset to 0).
- Modified Bit: 0 when the page content has not been modified, 1 when it has been modified.

Priority: Reference Bit > Modified Bit
