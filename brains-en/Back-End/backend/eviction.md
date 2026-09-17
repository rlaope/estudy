# Cache Eviction, Expiration, Passivation

Caches mostly use memory for speed.

Memory has significantly less usable space compared to disk.

### Eviction
It's about deleting data when space is needed.

When memory is full, unused data must be deleted for new data to be stored.

Most use an algorithm called LRU (Least Recently Used), which **replaces the data that has not been referenced for the longest time**.

### Expiration

It's the shelf life of data.

Generally, the term TTL (Time To Live) is used.

### Passivation

When this feature is used, data targeted for eviction is **first saved to another storage, such as a disk, before being deleted**.

Later, if a request for the same data comes in, it is retrieved from the file and returned.
