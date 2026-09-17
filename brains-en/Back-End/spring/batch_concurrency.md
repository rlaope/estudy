# Spring Batch Data Consistency Management

What are the ways to handle a situation where two Spring Batch applications that modify the same data run concurrently?

This was a question I received in an interview, so let's explore it in more detail.

### Controlling Concurrency

This method controls concurrency by placing a lock on data rows.

It's a simple and intuitive approach, but lock wait times can become long, and deadlocks may occur.

### Using Distributed Locks

When multiple applications attempt to update data concurrently, distributed locks are used to ensure each application acquires a lock before updating the data, thereby allowing only one application to update the data at a time.

While this is a powerful method for controlling concurrency even in distributed environments, it can introduce complexity and overhead for lock management, and requires a centralized system for lock administration.

### Batch Processing

This might seem like an obvious method, but it involves coordinating batches so they don't run concurrently. A scheduler ensures that data updates occur sequentially without overlapping.

### Version Control

By introducing version control for the data to be updated, if concurrent updates are attempted, each application checks if its version matches the current data version, and only performs the update if the versions align.

This method allows tracking data change history and is useful for concurrency control. However, it may require database schema changes, making it somewhat difficult to implement if not adopted from the start. It also requires additional logic for version management, which can introduce overhead.

Spring Batch is designed to create batch applications that can process large volumes of data at regular intervals. Given the nature of batch processing, running multiple processes on the same data might feel a bit awkward, but I believe it can frequently occur in real-world scenarios.

I haven't yet had to write batches like this in a professional setting, so I need to investigate further.
