# MySQL WAL

When considering the use of CDC (Change Data Capture) to reduce database load in large-scale systems, I believe it's crucial to deeply understand the internal structure of MySQL WAL, specifically InnoDB redo logs. Let's explore this.

In MySQL, data changes first occur in the in-memory buffer pool, and these change histories are then sequentially written to disk log files. This is WAL, or Write Ahead Logging.

### Redo Log

Redo logs are broadly divided into memory and disk areas.

**redo log buffer (memory)** When a user performs an update or insert, InnoDB loads the data page into the memory buffer pool and modifies it.

Concurrently, these changes are recorded in a memory space called the redo log buffer.

Because it's sequential writing rather than random I/O, it's very fast.

**redo log files (disk)** This is the stage where the contents of the memory buffer are written to actual disk files (e.g., `ib_logfile0`, `ib_logfile1`).

It basically has a circular structure, meaning that when it reaches the end of the file, it wraps around to the beginning and overwrites old data.

One might wonder if overwriting is an issue, but redo logs are not meant for permanent data storage. Instead, they serve as a temporary queue for changes that have been applied in memory but not yet written to the data files.

<br>

### LSN (Log Sequence Number)

LSN is an 8-byte monotonically increasing integer that represents the amount of data written to the redo log in bytes.

It serves as the reference point for all consistency checks.

- **Log Start LSN**: The point where writing begins in the log file.
- **Flush LSN**: The point where data has been physically written to the redo log files on disk.
- **Checkpoint LSN**: The last point where changes have been applied to the actual data files (idb).

During crash recovery, only the segment between the Checkpoint LSN and the Flush LSN is re-read and replayed to recover the data files.

<br>

### Redo Log recording mechanism innodb_flush_log_at_trx_commit

The way redo logs are written to disk upon transaction commit varies depending on the configuration, determining the trade-off between performance and data durability.

- 0: The buffer is written to the log file and flushed once per second. This offers the best performance but can result in up to 1 second of data loss if MySQL crashes.
- 1: The log file is written and physically flushed (synced) to disk with every commit. This ensures ACID compliance but results in lower performance.
- 2: The log file is written to the OS cache with every commit, but physical flushing occurs once per second. If the OS remains alive, data is safe even if MySQL crashes. If the OS crashes, there can be up to 1 second of data loss.

<br>

### Checkpointing

Redo log files have a limited size, so before they fill up, dirty pages in memory must be applied to the actual data files, and log space must be freed. This process is called checkpointing.

- sharp checkpoint: All dirty pages are applied when the database shuts down.
- fuzzy checkpoint: Pages are applied in batches to prevent performance degradation. InnoDB primarily uses this method.
- async/sync flush checkpoint: Occurs forcibly when redo log space is insufficient.

Data beyond the checkpoint is not overwritten. This prevents issues in the circular structure. If the log file becomes full and the head is about to catch up with the tail, and there's still data not yet moved to disk, MySQL temporarily halts all operations and forcibly flushes dirty pages from memory to disk.

Afterward, the checkpoint is advanced to free up space, and log writing resumes.

<br>

### Doublewrite Buffer: Preventing Data Corruption

Even with WAL, a problem that cannot be solved is Partial Page Write. Because the OS page size (4KB) differs from MySQL's page size (16KB), a page can become corrupted if power is lost during a write operation.

1. Before writing data to disk, it is first written entirely to the doublewrite buffer system tablespace.
2. Then, the actual data file is written.
3. If a page becomes corrupted during writing, the original page is retrieved from the doublewrite buffer and recovered before using the redo log for recovery.

<br>

### Connection to CDC

Redo logs are for internal InnoDB recovery, while MySQL separately maintains Binlogs for replication.

Tools like Debezium read the binary log and send it to Kafka. Upon transaction completion, redo logs and binlogs are tied together with a two-phase commit to maintain consistency.

The advantage of a WAL-based architecture is that it eliminates the additional write load incurred by separately managing an outbox table in large-scale systems, and it also guarantees order. The exact order in which changes are applied to the database is recorded in the logs, which facilitates message order guarantees.
