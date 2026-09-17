# JVM GC

## Garbage Collection
Garbage Collection is one of **Java's memory management methods**, a process that periodically deletes memory areas that are no longer needed from the **JVM Heap area** after being dynamically allocated. Unlike C and C++, which lack such garbage collection and require programmers to manually allocate and deallocate memory, Java's JVM-embedded garbage collector handles memory management. This allows developers to focus solely on development without having to perfectly manage memory or memory leak issues, which is a significant advantage.

### Disadvantages
1. Developers cannot precisely know when memory is deallocated.
2. Garbage collection causes overhead because other operations are paused while it runs.

<br>

## Objects Subject to Garbage Collection

![](https://blog.kakaocdn.net/dn/bW5c5r/btrvAb4nrdH/lYncQZya8ECvEndRkQchjk/img.png)

Objects are actually created in the Heap area, and Root Areas like the Method Area or Stack Area only reference the addresses of objects created in the Heap Area. However, when reference variables holding the memory addresses of these Heap Area objects are deleted due to specific events, such as a method ending, **objects that are not referenced anywhere in the Heap area** (like the red object in the image above) become present. These objects are called 'unreachable' and are periodically removed by the garbage collector.  
  
> Reachable: The state where an object is being referenced
> Unreachable: The state where an object is not being referenced

### Mark And Sweep Algorithm

![](https://blog.kakaocdn.net/dn/bGghBW/btrvvDgIHRO/HxoX3w9skgah3xFVhfEgD0/img.png)

The Mark And Sweep algorithm is the principle by which garbage collection operates, using the accessibility of an object from the root as the criterion for memory deallocation. Mark And Sweep is divided into three main stages, as shown in the figure above.  

- Mark
  - First, it traverses the graph from the roots to find connected objects, identifies which objects each references, and marks them.
- Sweep
  - It removes unreferenced objects, i.e., Unreachable objects, from the Heap.
- Compact
  - After the sweep, it moves fragmented objects to the beginning of the Heap, compacting the allocated and unallocated memory areas. (Some garbage collectors may not perform this step.)

<br>

## Heap Area Subject to GC

![](https://blog.kakaocdn.net/dn/bti1oP/btrvtcdoBC9/upBBOdB4mJF6tfyhL8GPbK/img.png)

For efficient GC, the Heap Area is divided into Eden, Survivor, and Old Generation, as shown above.

<br>

## GC Operation Process

### First Stage
![](https://blog.kakaocdn.net/dn/7pVmj/btrvu28jcRt/Iy5eB9flQ8L4eIkc0a1FX1/img.png)  
When an object is first created, it is allocated in the Eden space of the Heap with an age-bit of 0. This age-bit increments by 1 each time it survives a Minor GC.

### Second Stage
![](https://blog.kakaocdn.net/dn/cTWRqo/btrvxlfT2KU/gIDFZpUapbTZTKR1Gi16M0/img.png)  
After some time, when the Eden space of the Heap Area is full of objects, a Minor GC occurs. Objects are then either moved to the Survivor space or reclaimed, depending on their reachability.

### Third Stage
![](https://blog.kakaocdn.net/dn/b42htO/btrvuPvhcQ2/HwXDNMku8NbhSkaGoJEywK/img.png)  
New objects continue to be created in the Eden space. When the Eden space fills up again, objects in the Young Generation (Eden + Survivor) are moved to the empty Survivor space, Survivor1, and the age of all surviving objects increases by 1.

### Fourth Stage
![](https://blog.kakaocdn.net/dn/dpYphN/btrvocRXzk2/PANFhltyaGtzuDak9nqd61/img.png)  
When the Eden space is again filled with new objects, another Minor GC occurs. Objects in the Young Generation are moved to the empty Survivor space, Survivor0, and their age is incremented by 1. This process repeats continuously.

### Fifth Stage
![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FGvbFe%2FbtrvqaMQzF5%2F2pYF0QKjwBZWF7EtYNJ8OK%2Fimg.png)  
As this process repeats, the age-bit may exceed a certain number. When it reaches the age-bit threshold configured in the JVM, the object is deemed long-lived and moved to the Old Generation space. This process is called promotion.

### Final Stage
![](https://blog.kakaocdn.net/dn/b015X4/btrvtcRX3Go/DG6GyfMsZv0xgJRujfOeRK/img.png)  
Over time, if the memory allocated in the Old space exceeds its limit, a GC is executed that inspects all objects in the Old space and simultaneously deletes unreferenced objects. This GC, which reclaims memory in the Old Generation space, is called Major GC. Major GC is a time-consuming operation, and during its execution, all threads except the GC thread are paused. This is known as Stop-the-World. If this operation occurs too frequently, it can negatively impact program performance.
