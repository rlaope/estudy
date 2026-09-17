# Object Header - Mark Word

JVM objects are created on the heap, and each object's header, once on the heap, includes a Mark Word.

Based on HotSpot JVM, the object header is structured as follows:

1. mark word
2. klass pointer (class metadata pointer)
3. array length

In other words, the Mark Word is located in the header space within the object instance and is included in the heap memory.

The Mark Word is a core structure of the HotSpot JVM and stores the following information:

- lock state: biased lock, thin lock, fat lock
- Identity HashCode
- GC-related bits
- Age (object's survival age: increases when moving from young to survivor to old)

#### EX.

| Bits   | Meaning                                       |
| ---- | --------------------------------------------- |
| 25 bits | hashCode                                      |
| 4 bits  | age                                           |
| 1 bit  | biased lock flag                              |
| 2 bits  | Lock state (unlocked, thin, fat, marked for GC, etc.) |

### How Mark Word is Used in GC

In the GC Marking phase, the Mark Word is utilized in the following two ways:

Marking with Stop-the-World (Serial, Parallel GC)
When an object is marked as live, specific bits within the Mark Word are used, or it's tracked separately in a Mark Bitmap or Mark Stack.

G1GC's SATB + Card Table
- In G1GC, the Mark Word is involved in the SATB (Snapshot At The beginning) mechanism.

The following occurs in the SATB Write Barrier:

1. When a field of object A changes to object B
2. The old reference before the change is recorded in the SATB buffer.
3. Internally, a write barrier is executed concurrently to maintain a snapshot of the reference.
4. At this point, the SATB log of the Mark Word is read to check if the object is live.
5. The GC thread reads the SATB log and re-confirms if the object is live.

In other words, the Mark Word's marking bit is linked to G1's marking stage and is used to record whether an object is alive and whether it's a target for tracking.

### Mark Word, Thread Lock

The Mark Word is interpreted with a completely different structure depending on the lock state.

unlock: stores hashCode, age, GC bits

Biased Lock: The Mark Word changes to point to the ID of a specific thread.

Thin: The Mark Word transforms into a Lock Record Pointer on the thread stack.

Fat Lock: The Mark Word changes to a monitor object pointer.

In essence, the Mark Word becomes a variable-structure header that encapsulates everything from GC information to lock mechanisms in a single space.

| Component         | Location                               |
| ----------------- | -------------------------------------- |
| Object Instance   | Heap                                   |
| Object Header     | Start of each object within the Heap   |
| Mark Word         | **First 8 bytes (64bit) of the object header** |
