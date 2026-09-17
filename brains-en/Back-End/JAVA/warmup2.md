# Applying JVM Warm Up

1.  **JVM compiles and caches frequently executed code.**
2.  **Classes are loaded into memory via Lazy Loading when needed.**

These two points are the core ideas behind JVM Warm Up.

Just as we warm up with stretches or light jogging before exercise to maximize efficiency, the JVM also needs a warm-up to achieve peak performance. This is called `JVM Warm UP`.

We can identify two reasons for latency immediately after deploying a Java application: classes not being loaded into memory, and code not being compiled into optimized machine code.

The solution is simple. Pre-load classes into memory and compile the code with optimization. Do we do it ourselves? No. **At the time the application starts, we just need to execute the code at points expected to be frequently called, sufficiently many times.** Like preheating a machine.

Let's look at an example in Spring. As shown in the code below, you can write code to execute specific logic when a Spring application starts up, using `ApplicationRunner`. The code below is an example of warming up by calling the `findDetailCategoryById` method, which is expected to be used frequently. **(Classloader Warm-up)**

```java
@Component
public class WarmupRunner implements ApplicationRunner {

    private final CategoryController categoryController;

    public WarmupRunner(final CategoryController categoryController) {
        this.categoryController = categoryController;
    }

    @Override
    public void run(final ApplicationArguments args) throws Exception {
        try {
            categoryController.findDetailCategoryById(1L);
        } catch (Exception e) {
            // do nothing
        }
    }
}
```

```bash
xx... INFO 17860 --- [nio-8080-exec-2] c.a.d.c.presentation.CategoryController: before 소요시간 25ms
```

```bash
xx... INFO 17860 --- [nio-8080-exec-2] c.a.d.c.presentation.CategoryController: after 소요시간 2ms
```

Comparing before and after, the difference is stark, about 12 times.

Next, let's try JIT Compiler Warm-up. The method is to repeatedly execute methods designated as hotspots in advance to encourage the JIT Compiler to optimize them. **The default threshold for the C1 compiler is 1,500 executions, and for the C2 compiler, it's 10,000 executions.** You can refer to these values to warm up with an appropriate number of executions.

Since warming up enough to satisfy each threshold might take a long time, you should consider the trade-off between warm-up time and startup time when performing the warm-up.
