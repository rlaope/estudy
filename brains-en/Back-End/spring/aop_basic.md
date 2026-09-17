# 🌱 [Spring] What exactly is AOP (Aspect-Oriented Programming)? 🧐

### Overview

There's a team called Lifestyle within the school, and as its leader, I organized an online study session.

Of course, there were only two participants, but since it was the first time, I approached it casually.

Anyway, the mechanism of the online study was this:

Determine study topics based on the number of participants -> Each person is assigned one topic -> Present after a one-hour study session.

So, this time I was assigned the topic of AOP, and I'm going to try to summarize it.

### AOP (Aspect-Oriented Programming)

AOP stands for Aspect-Oriented Programming, also known as **관점 지향 프로그래밍** (aspect-oriented programming).

It's a technique that separates common cross-cutting concerns, extracts repetitive parts, and reduces source code duplication without affecting the core logic.

It emerged to overcome the limitations of traditional OOP, where applying common cross-cutting concerns to multiple modules led to the production of redundant code.

Simply put, aspect-oriented programming means **dividing functionality into core concerns and cross-cutting concerns** and **modularizing each based on these concerns**.

*Hmm... to put it even more simply, it means extracting code that is necessary but would otherwise be duplicated outside of the core logic, allowing us to focus on the core logic.*

![](image/aop_basic_0.png)

As shown in the image above, instead of having multiple concerns within a single class, everything is modularized so that each necessary class can use them.

***(Example)***

Let me give an example. Suppose I created a program to calculate factorials. I made a class using recursion that takes a factorial as input and returns a value. Then, an additional requirement came in: to find the start and end times of the calculation in milliseconds. So, I implemented the functionality to measure start and end times in the factorial calculation code. And then, another requirement was added: to include the start and end time measurement functionality in a program that calculates factorials using a loop. I wrote those classes again.

And this time, I received several other calculation logics and implemented the start and end time logic for them.

Let's say I created 1000 calculation programs. Then, a request came in to change the time, which was previously in milliseconds, to microseconds. This would require modifying all 1000 calculation programs.

Clearly, the core logic is the calculation, but I'm struggling because of other logic.

That's right, the "aspect" in aspect-oriented programming can also be seen as a "concern."

We can see here that it's about dividing things into **core concerns and cross-cutting concerns**.

In the example above, the core concern is the calculation logic, and the cross-cutting concern is the time measurement logic.

By separating the calculation logic and the time measurement logic, the problem can be solved by simply adding code to execute the calculation logic.

In other words, it means **extracting code that is necessary but would otherwise be duplicated outside of the core logic**, allowing us to focus on the core logic.

Furthermore, AOP is a technique for modularizing scattered Aspects.

*What is an Aspect?*

*-> It's easy to think of it as a concept similar to a class in object-oriented languages.*

*-> It's not a core feature that contains the application's domain logic itself, but rather an abstraction of cross-cutting concerns across many objects.*

### Key AOP Concepts (Terminology)

**- Aspect:** A modularized cross-cutting concern, primarily modularizing supplementary functionalities.

**- Target:** The place where an **Aspect** is applied (e.g., class, method).

**- Advice:** What actually needs to be done; the concrete implementation containing the actual supplementary functionality.

**- JoinPoint:** The point where **Advice** can be applied, an insertion point. (= method entry point, constructor call time, field value retrieval time, etc., applicable at various times).

**- PointCut:** A detailed specification of a **JoinPoint**. It allows precisely defining where an Advice should execute, such as 'call when method A is entered'.

### AOP Characteristics

- AOP implementation based on the Proxy pattern; proxy objects are used to control access and add supplementary functionality.

*What is a proxy object?*

*Here, you can simply think of it as an object that wraps a specific object and intercepts operations applied to that object, such as reading or writing properties.*

*For now, think of it as a delegate. Intercepted operations are either handled by the proxy itself or passed on to be handled by the original object.*

- AOP can only be applied to Spring Beans.

- Its purpose is not to provide all AOP features, but to support solutions for the most common problems in enterprise applications (duplicate code, hassle of writing proxy classes, increased complexity of relationships between objects...) by integrating with Spring IoC.

### @AOP

- @Aspect: This annotation is attached to a class to explicitly declare that it is an Aspect.

- @Around: This annotation means that it wraps the target method and executes a specific Advice (implementation).

**The execution point can be specified like this: `execution(\* com.saelobi..\*.EventService.\*(..))`**

It also provides a feature to execute the **Aspect** at a point where a specific annotation is attached, instead of using path-based targeting.

@Around("@annotation(ANNOTATION)")

public class ...

And it also provides a feature that can be applied to all methods of a Spring Bean.

@Around("bean(SERVICENAME)")

public class ...

Besides @Around, there are other annotations that can specify the execution time of an Aspect for a target method.

* @Before (Before): Performs the advice functionality before the target method is called.
* @After (After): Performs the advice functionality after the target method completes, regardless of the target method's result (i.e., success or exception).
* @AfterReturning (After Returning): Performs the advice functionality after the target method successfully returns a result.
* @AfterThrowing (After Throwing): Performs the advice functionality if the target method throws an exception during execution.
* @Around (Before and After Method Execution): The advice wraps the target method and performs its functionality both before and after the target method call.
