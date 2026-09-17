# Java String Pool

Java's String Pool is a special storage area located in the JVM's heap memory.

**This is specifically a space for storing and sharing string literals.**

In Java, a string literal refers to a string written directly in the code, and is stored in the String Pool at compile time. This allows the JVM to efficiently reduce memory usage for strings.

### Example

For example, if the string literal "Hello" is used in multiple places, only one "Hello" string instance is created in memory for all these cases, and only references to that instance are shared.

In other words, since string literals with the same value are stored only once in the String Pool, memory usage can be significantly reduced when using strings with the same value multiple times.

However, this characteristic does not apply when explicitly creating a new String object using the `new` operator. That is, if you create a string like `String s = new String("Hello");`, a new String object is created in heap memory without using the String Pool.
