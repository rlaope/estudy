# Efficiently using `use` when working with classes that require `.close()`

There are classes that need to be explicitly closed using the `.close()` method when they are no longer needed.

For example:
- InputStream, OutputStream
- java.sql.Connection
- java.io.Reader
- java.new.Socket
- java.util.Scanner

These resources implement the `Closeable` interface, which inherits from `AutoCloseable`.

All these resources are eventually handled by the garbage collector when their references are no longer available.

However, this approach is very slow and not easily managed, which is why the client using them explicitly handles the `.close()` operation.

Traditionally, this approach is used with `try-catch-finally` blocks.

However, writing code this way is not only complex, but if an exception occurs in `catch` or `finally`, it's not handled separately. This leads to a problem where, if an error occurs within the block, only one of the exceptions is propagated.

Of course, it's possible to implement it so that both can propagate, but it becomes very complex.

```kt
val reader = BufferdReader(FileReader(path))
try {
    return reader.lineSequence().sumBy { it.length }
} finally {
    reader.close()
}
```

However, if you appropriately modify the previous code using the `use` function, it looks like this. This code can be used for all `Closeable` objects.

```kt
reader.use {
    return reader.lineSequence().sumBy { it.length }
}
```

> Additionally, `useLines` is provided for cases where files are often used as resources and need to be read line by line.

It's similar to how `try-with-resources` is used in Java.

### Conclusion

Using `use` allows for easy and safe handling of objects that implement `AutoCloseable`/`Closeable`.

Furthermore, when processing files, it is recommended to use `useLines` to read files line by line.
