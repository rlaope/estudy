# PGO (Profile-Guided Optimization)

PGO is an optimization technique that helps compilers generate faster and more efficient machine code based on data (profiles) collected during the actual execution of a program.

It is a core technology that enables similar implementation in AOT (Native Image) build environments, where JIT Compilers (c2) naturally perform optimizations at runtime.

### Why is PGO necessary?

AOT compilation methods, such as GraalVM Native Image, generate all machine code before execution (at build time).

Since it's unknown which code will be frequently used during execution, and there's no certainty about which branch will primarily execute or which concrete implementation object will actually be assigned to an interface variable, conservative code is generated that accounts for all possibilities.

PGO pre-injects these execution statistics into the AOT compiler in the form of an external file, thereby encouraging the generation of aggressive and efficient machine code, much like a JIT compiler.

### 3-step process of PGO in GraalVM

The process of optimizing a native image by applying PGO in a real environment is as follows.

1.  **Instrumentation Build**: First, a special native image build is performed with code inserted to collect execution statistics. (e.g., enabling the `--pgo-instrument` option)
2.  **Profiling Data Collection**: The generated special image is run with a workload that closely resembles the production environment (e.g., generating dummy traffic). Once the program finishes execution, a profile file (typically `*.iprof`) containing frequently called methods, branch reachability probabilities, type statistics, etc., is extracted to disk.
3.  **Final Optimized Build**: The collected profile file is provided as input to the AOT compiler to rebuild the final native image. (e.g., using the `--pgo=default.iprof` option)

#### Key benefits of PGO

The compiler analyzes the provided data and performs C2-level optimizations such as the following.

-   **Accurate Inlining**: It identifies methods in hot paths that are actually called most frequently, eliminating function call overhead and merging the code directly into the call site.
-   **Devirtualization**: For example, if runtime statistics confirm that a `List` type variable always contains `ArrayList` objects 100% of the time, it bypasses complex virtual method table lookups and directly links the machine code to call `ArrayList`'s methods.
-   **Branch Prediction Optimization**: In an `if-else` statement, if the `if` block is executed 99% of the time, its code is arranged sequentially in a CPU-friendly location to maximize instruction pipeline efficiency.

As a result, native binaries with PGO applied retain the inherent advantages of traditional AOT (instant startup time, low memory usage) while achieving high throughput approaching the peak performance of JIT compilers.
