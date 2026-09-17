# JIT Compiler

In the JVM, the JIT Compiler is a compiler that converts bytecode into machine code at runtime.

The JVM compiles Java code into .class files and executes them as bytecode (intermediate code). This allows it to achieve the 'write once, run everywhere' characteristic, independent of any platform.

Of course, using bytecode can have the disadvantage of longer build times compared to other native languages.

In the JVM, bytecode is originally executed line by line in an interpreted manner, which is very slow in terms of execution time and code optimization.

To solve this, the JIT Compiler detects frequently executed code, converts it into machine code, caches it, and then executes it.

### Operation Structure

After class loading, the JVM initially interprets bytecode and detects frequently called methods as 'hotspots,' counting their invocation frequency (InvocationCounter).

Once a certain threshold is reached, the JIT compiles that method into native machine code and replaces the interpreted execution with the compiled native code. From then on, that method is no longer interpreted.

The performance of JIT is exceptional, and as will be mentioned below, if the c2 compiler runs and optimization is perfected, it can deliver outstanding performance in high-performance server applications.

JIT uses representative optimization techniques such as the following:

- Inlining: Replaces method call code with its actual implementation to eliminate call overhead.
- Loop Unrolling: Duplicates code to reduce the number of loop iterations.
- Escape Analysis: Allocates objects on the stack instead of the heap if they exist only within a method.
- Dead Code Elimination: Removes code that will not be executed.
- Constant Folding: Pre-calculates computations possible at compile time.

```java
public class Hello {
    public static void main(String[] args) {
        for (int i = 0; i < 10_000; i++) {
            sayHi(); // Repeatedly called, becomes a HotSpot → JIT compiled
        }
    }

    public static void sayHi() {
        System.out.println("Hi");
    }
}
```

You can check which methods have been JIT-compiled using the `-XX:+PrintCompilation` option.

<br>

## c1, c2 compiler

C1 is optimized for applications that aim for fast compilation (short latency). While its optimization level is low, its compilation speed is fast. It is characterized by good performance during initial execution.

C2 has a slow compilation time but is characterized by high-performance optimization (a long-term investment). It is ideal for server applications and long-running services, starting slow but gradually speeding up.

### Tiered Compilation Structure

Considering both performance and compilation cost, the JVM categorizes compilation into about 4-5 tiers to decide which compiler to use.

- tier 0 Interpreter(C1): Simple bytecode interpretation
- tier 1 without profiling(C1): Fast compilation (simple optimization)
- tier 2 with profiling(C1): Fast compilation + execution information gathering
- tier 3 with full profiling(C1): More profiling information gathering
- tier 4 with profiling(C2): C2 generates high-performance native code with full optimization

The promotion information from C1 to C2 is determined by recording values like the invocation counter and back edge count, i.e., the number of loop executions.

```
- Default thresholds:
  - Invocation count: 10,000 (client), 10,000~15,000 (server)
  - Loop count: 15,000 or more

- Customizable via JVM options:
  - -XX:CompileThreshold=10000
```

In other words, if the same method is called thousands or tens of thousands of times, the JVM deems it important and promotes it. There's also a technique called JVM warm-up, where methods are called multiple times during compilation to make the JIT aware of them, which might be worth looking into.

```bash
java -XX:+UnlockDiagnosticVMOptions -XX:+PrintCompilation -XX:+TieredStopAtLevel=4 YourApp

---
  101   1       YourClass::yourMethod (5 bytes)
  230   3%  4   YourClass::yourMethod (5 bytes)
```

You can observe the operation as shown above. The first number is the method ID, followed by the tier level (0-4), and the percentage symbol indicates whether it's a profiling-based compilation.

One might wonder what profiling is here; the collection of profiling data is a key differentiator in the level of optimization.

When profiling data is collected, metrics such as the following are gathered:

- hot method: How often is it called?
- hot loop: How many times does the loop iterate?
- branch prediction: Probabilities of conditional branches (actual execution ratio for each if statement)
- method inlining candidate: Is it a frequently called, short method?
- type profiling: What types are actually being passed? Is an object always a string? (because JVM compiles with type erasure)

Based on these metrics, if the JVM determines a branch is not executed, it performs advanced optimization tasks such as removing the code, pre-changing types, inlining, or eliminating dead code. Therefore, the quality of optimization varies depending on the profiling information collected. (Of course, compilation time increases due to more optimization work.)

There are rumors that JIT is so well-optimized that it outperforms most native applications; I haven't measured it myself.

However, due to the difficulty of maintaining code in HotSpot VM, Graal, implemented solely in Java, is said to be emerging. It reportedly supports AOT compilation based on native images, and since I haven't used it yet, it might be worth looking into.
