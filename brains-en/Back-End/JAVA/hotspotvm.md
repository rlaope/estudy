# Hotspot VM, JIT Compiler, and Their Limitations


### How C Language Works

Compiled languages like C and C++ translate directly into machine code during the compilation process and create an executable file.

During compilation, code optimization is also performed, resulting in excellent processing performance.

However, the generated machine code has the disadvantage of being dependent on the build environment (CPU architecture).

If the platform changes, it requires rebuilding.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbHFBej%2FbtsmAL1jfGz%2FwgTWSn9U10bsdsA0IeXXdk%2Fimg.png)


### How Java Works

To solve this platform dependency issue, Java introduced the JVM.

Its operation differs from languages like C. First, Java code is compiled into bytecode.

Bytecode is simpler than Java code. However, computers cannot read it directly as it's not 0s and 1s.

Therefore, bytecode can be seen as an intermediate language for the JVM. As an application runs, the JVM translates the read bytecode into machine code in real-time, and the CPU processes the translated machine code.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FdW1j9K%2Fbtsmz3HTV3H%2FUFiL9zon70psPwHMAoyNGk%2Fimg.png)

Here, Java is not platform-dependent. However, the JVM can be considered platform-dependent because its machine code translation differs across platforms.

But translating bytecode to machine code in this manner is relatively slow.

To address this issue, a JIT compiler, which compiles bytecode into machine code, has been adopted.

The goal of JIT compilation is fast compilation and optimization tailored to specific environments. To achieve this, it checks execution profile information.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FlNWQk%2FbtsmzIYn2KP%2Flu5xb2awy3ZwqjeW3FDHBK%2Fimg.png)


### JIT Compiler

**Hotspot VM** was added starting from Java 1.3, and Hotspot VM includes two JIT compilers.

> **JIT?**
> JIT stands for Just In Time, a concept introduced in .NET and Java environments.
> The method of repeatedly compiling Java -> bytecode -> interpreter was very inefficient in terms of speed. To solve this problem, JIT was created. A JIT compiler first examines the entire source code and pre-compiles and stores duplicated parts as machine code. Afterwards, when it encounters a duplicated part during interpretation, it reuses the already translated machine code. (Initial operations might be slower due to preliminary tasks like reserving memory.)


![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbRIWSX%2FbtsmBZEKoz7%2FJGTkxTQH2awKzvMKfxn4B0%2Fimg.png)

C1 is a client compiler; it performs less code optimization but starts faster. It is suitable for desktop applications that execute immediately.

C2 is a server compiler; it starts slower but performs extensive code optimization, making it fast after warm-up. It is suitable for long-running server applications.

> JVM Warm-up is the process of initializing and optimizing a program or application in advance before it runs.


Hotspot VM initially uses the C1 compiler. The Hotspot VM continuously monitors method calls, and if the number of method calls for a specific method increases, it recompiles that method using the C2 compiler. **This is also known as Tiered Compilation.**

C2 performs extreme optimization, sometimes even outperforming compiled languages. Optimization uses profiling information (number of devices, clock cycles), and Java's performance improvement largely depends on the JIT compiler's optimization role, based on information obtained through such profiling.

Conversely, in the initial stages, problems such as slower execution speed can occur during the warm-up process.

### Limitations of JIT

The C2 compiler also faced limitations. The main problem is that it's implemented in C++, making it difficult to find developers, and its age makes it very complex. For these reasons, there haven't been significant optimizations developed in recent years, and even experts find maintenance challenging. It's now reaching its End of Life...

Therefore, efforts have been made to add the `HotspotIntrinsicCandidate` annotation to code that is difficult to optimize further.

In short, the JIT compiler is not slow; rather, it offers excellent optimization performance. However, the biggest issue is its difficulty in maintenance.

Due to these problems, [[GraalVM]] emerged.
