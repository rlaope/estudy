# GraalVM

GraalVM started with the goal of creating a new compiler to replace HotspotVM's C2 compiler, and its 1.0 version was released in May 2019, seven years after it was registered as an Open JDK subproject in 2012.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2F7dqDh%2FbtsmBZEKuE4%2FDnXBBIpyKl8RuT3OACCiZk%2Fimg.png)

GraalVM has three key features, and its architecture explains why.

- High-performance Java
- Integration of various languages
- Fast start-up through native support (Native Image)

GraalVM added GraalVM, an advanced JIT compiler written in Java, to the Hotspot JVM. It also added Truffle, a Language Implementation Framework, enabling multiple languages like Python and JavaScript to run on the JVM. This allows data to be exchanged within the same memory space.

- Hotspot JVM
- Graal Compiler
- Truffle
- GraalVM Updater

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FF6dCN%2Fbtsmz1cn4R0%2FaKKFKyUIAwkduRyjO8Dow1%2Fimg.png)

### JVM Runtime Mode

When running programs on the Hotspot JVM, GraalVM uses the GraalVM compiler as the top-tier JIT compiler by default.

Runtime applications are loaded and executed on the JVM. The JVM passes bytecode from Java or other native languages to the compiler, which then compiles it into machine code and returns it to the JVM.

Interpreters for supported languages are written on top of the Truffle framework and are themselves Java programs that run on the JVM.

The newly created Graal JIT compiler offers better performance than the existing C2 compiler.

### Native Image

Native Image is an innovative technology that compiles Java code independently into native executables or native shared libraries.

During the build of a native executable, the Java bytecode processed includes all application classes, dependencies, external libraries, and all necessary JDK classes.

The resulting self-contained native executable is specific to the architecture of each individual operating system machine, does not require a JVM, and is processed by an AoT compiler.

> I will discuss AoT compilers in the next post.

### Java on Truffle

Truffle Java is an implementation of the JVM specification built with the Truffle language implementation framework.

It includes all core components and implements the same API as JRE libraries.

It is a complete Java VM that reuses all JARs and native libraries from GraalVM.

Truffle Java provides the Polyglot API, which allows different programming languages to be combined at runtime.

**In other words, it can run programs written in different languages on the JVM!**

> **OpenJDK and GraalVM**
  First, JVMCI (Java Compiler Interface) was added to Open JDK 9 according to [JEP 243](https://openjdk.org/jeps/243). Then, GraalVM's AoT compiler was added to Open JDK 9 according to [JEP 295](https://openjdk.org/jeps/295), and later, the Graal JIT compiler was added to Open JDK 10 according to [JEP 317](https://openjdk.org/jeps/317). However, starting with Open JDK 17, the Graal JIT compiler and AoT were removed, leaving only JVMCI. This is not because Graal became unnecessary. **It's because most GraalVM users install and use GraalVM directly, rather than utilizing the features embedded in OpenJDK.** GraalVM will continue to evolve in the future.
