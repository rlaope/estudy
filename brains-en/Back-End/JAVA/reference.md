# Java's Four Reference Types (Strong, Weak, Soft, Phantom)

## Strong Reference

This is the most commonly used reference type.

As long as a strong reference is linked to an object, the garbage collector will not reclaim that object.

## Weak Reference

You can create a weak reference to an object using the `WeakReference` class.

When the garbage collector runs, objects that only have weak references remaining will be removed from memory.

## Soft Reference

Created using the `SoftReference` class.

Soft references are not subject to garbage collection until memory runs low.

This means that even if the garbage collector runs, objects pointed to by soft references will not be reclaimed if there is sufficient memory.

## Phantom Reference

Can be created using the `PhantomReference` class.

Phantom references allow the referenced object to be reclaimed by garbage collection.

This reference type is primarily used to handle special finalization tasks that need to be performed before an object is removed from memory.

> All of these reference types belong to the `java.lang.ref` package, and while their usage varies depending on the situation, they are important tools for memory management and resource cleanup.
