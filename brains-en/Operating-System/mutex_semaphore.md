# Mutex and Semaphore

The biggest challenge in concurrent programming is likely `shared resource management`.

To safely manage shared resources, a technique for achieving mutual exclusion is necessary.

Mutexes and semaphores are techniques designed for this purpose, achieving mutual exclusion in different ways.

## Mutex Mutex
A mutual exclusion technique based on a `Key` that can be owned by a single thread or process.

It prevents more than one process or thread from accessing shared resource data or critical sections (synchronization target is one).

A technique that ensures threads with critical sections execute exclusively, without their execution times overlapping.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fcvk7nh%2FbtrjvSw2BoX%2FZfh0o0VsZrMmAOi6PxLvg0%2Fimg.png)

### Example
A mutex is similar to a restaurant with only one restroom, for example.

To use the restroom, you must get a key from the counter.

If you want to use the restroom and the key is at the counter, it means no one is in the restroom, and you can use that key to enter.

![](https://cdn-images-1.medium.com/max/1600/1*6JKj81oYsxQlDhjAlMXQjg.png)

While you're enjoying your time in the restroom, a man at another table wants to use it.

No matter how urgent his need, he cannot enter the restroom because he doesn't have the key. Consequently, he must wait at the counter until you finish and come out.

![](https://cdn-images-1.medium.com/max/1600/1*kbTbS09Yvah9ja7nozSAMA.png)

Soon after, a man at the next table also wants to use the restroom, and he too must wait at the counter to enter.

![](https://cdn-images-1.medium.com/max/1600/1*CgUE8ByDUKnVkkrEbMPwCQ.png)

Now you've come out of the restroom and returned the key to the counter. The person at the front of the line can now take the key and use it to go to the restroom.

![](https://cdn-images-1.medium.com/max/1600/1*dIIfI3ezb3Gt2YH1uWhauQ.png)

This is how a mutex operates. The person using the restroom is a **process or thread**, the restroom is a **shared resource**, and the restroom key is an object required to access the shared resource.

![](https://cdn-images-1.medium.com/max/1600/1*CdLr52i_BZjnEf3uWZyRVQ.png)

In other words, for a mutex, there must be an object corresponding to the key, and only the (thread, process) that owns this object can access the shared resource.

## Semaphore Semaphore
Signaling mechanism. A technique that achieves mutual exclusion by maintaining a value representing the number of threads or processes that can currently access a shared resource.

It prevents multiple processes or threads from accessing shared resource data or critical sections (synchronization target is one or more).

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FcYZOiu%2FbtrjvrzaimS%2FQtooHYav5Sj1JpT9yTtb1K%2Fimg.png)

### Example
A semaphore is like a restaurant where guests can use the restroom more easily.

In a restaurant using semaphores, the restroom has multiple stalls. And at the restroom entrance, there's a display board showing the number of currently available stalls.

![](https://cdn-images-1.medium.com/max/1600/1*ZJXrQu8rFhSQxW6LVAI-GA.png)

If you want to use the restroom, you should check the number of available stalls at the entrance. If there's at least one available stall, you must decrement the count by one before entering the restroom.

![](https://cdn-images-1.medium.com/max/1600/1*lFNABipdkdtxFvW9UZmCaw.png)

If all stalls are occupied, the number of available stalls becomes 0. At this point, if someone wants to enter the restroom, they must wait until the number of available stalls changes to 1.

![](https://cdn-images-1.medium.com/max/1600/1*wP9yqG6QBuS7A8i_kKTyRA.png)

As people leave, they increment the count of available stalls by 1. Then, the waiting person decrements this number by 1 again and rushes into the restroom.

![](https://cdn-images-1.medium.com/max/1600/1*36aMopAPHO3e80YYADmY6w.png)

In this way, a semaphore achieves mutual exclusion using a single commonly managed value.

Similar to the previous example, the restroom is the shared resource, and the people are threads or processes. The number of available restroom stalls represents the number of threads or processes that can currently access the shared resource.
