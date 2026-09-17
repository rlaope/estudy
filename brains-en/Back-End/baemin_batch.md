# Elegant Spring Batch by Woowa Bros. Lee Dong-wook

### Before Diving In

The talent Baemin seeks - should be able to be persuaded by someone with less experience than themselves, and should be able to persuade someone with more experience than themselves. <- This is totally me

<br>

## Batch Applications
Batch applications are not what we commonly refer to as APIs, and batch processing is the execution of a series of program tasks on a computer **without human interaction**.

The lack of human interaction is the biggest difference from web applications. (No response after request -> All tasks are processed and completed with a single request).

### Web vs Batch
- Web: Real-time processing / Relative speed / Ease of QA
- Batch: Post-processing / Absolute speed / QA complexity

<br>

### Spring Batch vs Quartz
There are many questions like, "Which one is better?", "Is Spring Scheduler better?"

Quartz is a scheduling framework. It's one of several ways to execute a batch... Saying it replaces batch is nonsense. **It serves as a complement to Spring Batch, not a replacement.**

<br>

### Situations Where Batch Applications Are Needed
- When it needs to be executed at regular intervals
- When large amounts of data that are difficult to process in real-time need to be handled
  - For large datasets like 500 million records in a main table for a settlement system, batch processing is much better than an API.
  - Some batches perform various tasks such as aggregation, statistics, and caching.

Regular intervals? Quartz? Batch? Running once a month means that **all data accumulated over that month is the target**.

In other words, large-scale data processing is an absolute requirement.

In Spring Batch, the default retrieval method does not load all data into memory (never `findAll`). { The commit size can be specified at the framework level as a `chunk` unit, and stream-based reading is supported by default. }

<br>

## Job, Step, Tasklet

Within the large concept of a Job, there are process units called Steps, and within Steps, there are Tasklets.

There's a misconception that Tasklet and Reader Writer (ChunkOrientedTasklet) operate separately, but that's not true. If you look closely, ChunkOrientedTasklet can actually be seen as an implementation of Tasklet.

+ Spring Batch does not support LocalDate. -> Solution: Utilize the characteristics of `@Value`. Declare `@Value` on a setter method, receive it as a string, and then set it to the desired type. -> After creating a `@JobScope` Bean, receive the constructor via DI.
