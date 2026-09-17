# GC Write Barrier

A Write Barrier is a small piece of code that the JVM automatically executes whenever an object's reference field is modified.

```java
obj.field = newValue;
```

When a reference write like this occurs, the JVM automatically inserts additional logging/checking code before and after this statement.

> Auxiliary logic that the JVM automatically executes at each reference write to maintain the GC's internal state.

Developers don't see this code; it's transparently embedded within the JVM.

This concept is necessary due to a core problem in Concurrent GC: while the GC is marking/evaluating, the application continues to change references without stopping (no STW). During this time, references might change to new objects, become null, be removed, or the existing object graph might momentarily become fragmented.

And when the GC runs, it must not miss these changes. Missing them could lead to a serious error where live objects are prematurely collected. In other words, when a reference changes, it acts as a trigger to inform the GC that an important change has occurred.

```java
A.b = C
A.b = null
B.child = new Node()
array[i] = x
map.put(k, v)
list.remove(x)
```
The code examples above are moments when reference writes occur, and the JVM automatically executes another piece of code – this is the write barrier.

### What a write barrier does varies depending on the type of GC.

There are typically two main uses:

#### SATB Write Barrier

For G1GC, Shenandoah, etc., to maintain snapshots.

When a field is modified, it records the old reference, allowing the snapshot TO to be restored.

#### Card Table Write Barrier

For Generational GC (Parallel, CMS) "tracking reference updates between new and old generations".

When a field is modified, it marks the card containing the modified object as dirty, which helps quickly find old -> young references during minor GC.

### Write Barrier from an SATB Perspective

An SATB Write Barrier operates as follows:

Just before a reference changes, the oldValue is placed into the SATBQueue. Pseudo-code:

```java
oldValue = obj.field;

if (GC가 marking 중 && oldValue != null) {
    SATBBuffer.add(oldValue);
}

obj.field = newValue;
```

This oldValue becomes an essential clue for restoring the snapshot (TO).

> A Write Barrier is a GC-assist guard code that the JVM executes the moment a reference field changes, and its purpose is to record and track heap changes so that the GC doesn't miss them.

```
A → B → C
```

In the initial graph, while GC marking is in progress, the application performs:

```
A.b = null
```

Let's say this happens. At this point, the write barrier executes and records `oldValue = B` for the GC. The GC can then mark C based on this record, thus maintaining the snapshot.
