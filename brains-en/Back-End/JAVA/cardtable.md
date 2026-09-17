# Card Table

A Card Table is an array that records the Heap, divided into small Card units.

It is a core component of Generational GC, specifically among Write Barriers.

Each card typically represents a heap region of about 512 to 1024 bytes.

The JVM divides the heap as follows:
```css
[card0][card1][card2][card3] ... [cardN]
```

And a separate array that stores the states of these cards is the Card Table.

The Card Table looks like this:

```
byte cardTable[N];
```

One byte is used to represent each card.

In Generational GC, we want to scan only the young region during young GC and not scan the old gen region every time.

However, there's a problem: what if an old object references a young object?

```
oldObj -> youngObj
```

During young GC, youngObj is reachable from both the root and oldObj.

However, young GC does not scan the entire old gen.

This means that because it doesn't traverse oldObj's fields, YoungObj might be missed. (It could be collected even though an old object holds a reference to it.)

Solution: We must know the pointers connecting old objects to young objects. This is called an old -> young cross-generation reference.

The problem is that the old generation is too large. Scanning it entirely would cause GC costs to explode. So, a method is used where only small units of the heap are marked as dirty to find references quickly.

This is the purpose of the Card Table.

### What the Card Table Does

The Card Table records the following information:

**Within this card's range, an old -> young object reference has changed.**

When a reference changes, the JVM marks the value of the corresponding card in the Card Table as Dirty via a write barrier.

The Dirty value is typically 0/1 or a specific byte value.

When a reference write occurs, the Card Table is updated as follows:

```ini
oldObj.child = youngObj;
```

At this point, the JVM write barrier:
1. Calculates the card number from the heap address oldObj belongs to.
2. Sets cardTable[cardIndex] = DIRTY.

The pseudo-code for the Card Table is below:

```csharp
function write_barrier(obj, field, newValue):
    obj.field = newValue

    if obj is in OldGen AND newValue is in YoungGen:
        cardIndex = (obj.address - heapStart) / cardSize
        cardTable[cardIndex] = 1   # dirty
```

How the Card Table is used when young GC runs.

Minor GC root scanning process:
1. Root scan (thread stack, global roots)
2. Card table scan
   1. Reads only dirty-marked cards
   2. Scans within that card's range, as there might be old -> young references
3. Mark young objects
4. Survivor/Eden operations

This way, old -> young references can be accurately tracked without examining the entire old gen.

This is key to young GC performance.

```
|----Card0----|----Card1----|----Card2----|----Card3----|
   [OldObjs]     [OldObjs]     [Young]       [Old]
```
[cardTable = [0, 1, 0, 0]]

If card1 is dirty here, young GC scans only card1's range and ignores the rest.

This means old objects in card0 are not explored. The old gen is not touched.

### vs. SATB Write Barrier

| Category | SATB Barrier (G1/Concurrent GC) | Card Table Barrier (Gen GC) |
| -------- | ------------------------------- | --------------------------- |
| Purpose  | Maintain snapshot (T0)          | Track Old → Young references |
| Recorded content | old reference                 | dirty card index            |
| Required GC | Concurrent marking              | Generational GC             |
| Scope    | Protect marking                 | Maintain remembered set     |

In summary:
- A Card Table is an array that records the state of each card after dividing the heap into card units.
- If a reference pointing from old gen to young gen is created, it's marked as dirty.
- During young GC, only dirty cards are scanned to identify old -> young references.
- This allows for fast young GC without scanning the entire old gen every time.
- Card Table updates are automatically handled by write barriers.
