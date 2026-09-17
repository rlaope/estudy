# Memory Optimization for Native Images

Today, we will explore memory optimization techniques for Native Images.

To recap Native Images, they are a technology that compiles Java code ahead of time. This means converting it directly into a binary, allowing Java code to run without a JVM.

## Memory Optimization for Native Images

- Profile-Guided Optimizations (PGO) provide additional performance improvements and higher throughput.
- Choosing an appropriate GC and modifying garbage collection policies can reduce GC time.
- Loading application configurations during image build can speed up application startup.

### Memory Management

When executed, Native Images run not on the Hotspot VM but on GraalVM's runtime system. Java objects created at runtime are allocated in the heap.

The heap is allocated when the Native Image initially runs and can grow or shrink during execution.

When the heap is full, the garbage collector is triggered to reclaim unused objects from memory.

Native Image provides several GCs for Java heap management:

- Serial GC
- G1 GC
- Epsilon GC

### Serial GC

`native-image --gc=serial HelloWorld` is an example of using Serial GC.

Serial GC is provided by default, so if not specified, it is automatically assigned.

If a maximum Java heap size is not provided, Native Image sets the heap size to 80% of the actual memory size. (This setting is only a maximum; actual Java programs may use less).

It's important to note that additional memory space is required when using GC. In some cases, up to twice the maximum heap memory space might be needed.

Therefore, RSS (Resident Set Size) may temporarily increase.

This means that in memory-constrained environments (such as containers), problems can arise, so tuning must be done carefully.

We'll look into tuning next time.

### G1 GC

GraalVM provides G1 GC based on Hotspot VM's G1 GC.

Currently, G1 GC is only available for Native Images built on Linux for AMD64, and to activate it, you must pass the `--gc=G1` option.

`native-image --gc=G1 HelloWorld`

If a maximum Java heap size is not specified, the heap for Native Images using G1 GC is set to 25% of the memory space.

### Class Initialization

In typical JVM Java applications, classes must be initialized when they are first accessed.

Therefore, initializing all classes upfront has negative impacts when performing Ahead-of-Time (AoT) compilation for Java applications.

- It significantly degrades the performance of native executables. Every time a class field or method is accessed, it must be checked for initialization, and without optimization, performance degrades significantly.
- It increases the computational load and time required to start the application. For example, a simple Hello World application may need to initialize over 300 classes.

To mitigate the negative effects of class initialization, Native Image supports class initialization at build time.

By initializing classes when building the executable, runtime initialization and checks can be made unnecessary.

All static state of initialized classes is stored in the executable.

Access to static fields of classes initialized at build time appears transparent to the application and behaves as if initialized at runtime.

There are various policies that complicate Java class initialization, but Native Image addresses this through the following two approaches:

- Build-time initialization
- Automatic initialization of only safe classes

+ The background for researching GraalVM and Native Images includes the need for fast startup to handle surging traffic and infrastructure configuration based on container environments. However, GraalVM's performance is inferior to a fully optimized JIT compiler.
Furthermore, the community version only allows the use of Serial GC. Therefore, we proceeded with an approach that utilizes the existing JIT compiler and prepares for its warm-up.
