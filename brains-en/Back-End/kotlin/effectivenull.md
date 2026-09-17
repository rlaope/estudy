# Proper Null Handling with lateinit and Delegates.notNull<>()

In Kotlin, it's best to avoid nullability as much as possible.

There are ways to handle nullable variables so that they are processed only when they are not null, as follows:

```kt
printer?.print()
if(printer != null) printer.print() // smart cast
printer!!.print()
```

Cases 1 and 2 are set to operate only when not null, so case 3 (not null assertions) carries the risk of an NPE.

While nullable variables can be handled appropriately, eliminating unnecessary nullability altogether is considered ideal.

You can prevent unnecessary nullability using the following methods. (Of course, you need to distinguish carefully between cases that should be nullable and those that shouldn't.)

<br>

## lateinit

Using lateinit allows you to defer initialization.

```kt
private var printer: Printer? = null

private lateinit var printer
```

For example, when operating in code like @BeforeEach in a Mock test, explicitly stating that initialization will be deferred because it will be initialized soon, rather than initializing with null, provides a significant difference and stability.

If you declare it as a nullable type, even after BeforeEach runs (and printer is not null), you still have to cast the type using !!. This is inconvenient. However, using lateinit eliminates such tedious tasks.

Of course, using lateinit can incur additional overhead. Therefore, use it only in situations where it must be initialized before use, and intended to be used immediately after initialization.

Using lateinit offers the following advantages:

- You don't have to repeatedly unpack with !!.
- You can make it a nullable type if you want to represent null in some sense.
- It does not revert to an uninitialized state.

<br>

## Delegates.notNull<>()

There are cases where lateinit cannot initialize properties, specifically when initializing properties of types linked to JVM primitive types like Int, Double, or Boolean.

In such cases, you can delegate it using Delegates.notNull<>().

```kt
private var isOk: Boolean by Delegates.notNull()
```

Of course, it is slightly slower than lateinit.

By using property delegation in this way, you can handle various issues arising from nullability.
