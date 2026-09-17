# Spring Boot Batch

## Spring Boot Batch
Spring Batch is a framework used to implement **batch processing capabilities** in the backend.

Spring Boot Batch simplifies Spring Batch configuration elements, helping to quickly set up Spring Batch.

> Batch Job: Refers to a task that processes data collectively at once, rather than in real-time.

## Advantages of Spring Boot Batch
- Optimized for large-volume data processing, delivering high performance.
- Supports reusable essential features such as effective logging, statistics processing, and transaction management.
- Automated to avoid manual processing.
- Includes defensive mechanisms against exceptions and abnormal operations.
- Understanding the repetitive work processes in Spring Boot Batch allows you to focus on business logic.

## Precautions
Spring Boot Batch is a project that wraps Spring Batch to make it easier to use.
Therefore, the following precautions should be kept in mind for both Spring Boot and Spring Batch:

- Simplify as much as possible, avoiding complex structures and logic.
- Since direct data manipulation occurs frequently, defensive measures such as validation are necessary to maintain data integrity.
- Minimize I/O usage in batch processing systems. Frequent I/O can increase database connection and network costs, potentially affecting performance. Therefore, it is best to retrieve data all at once, store it in memory, process it, and then save the results to the database in a single operation.
- Generally, web APIs, batches, and other projects used in the same service affect each other. Therefore, care must be taken to ensure that batch processing does not impact elements of other projects.
- Spring Boot does not provide a batch scheduler. It only offers batch processing functionality, so scheduling features must be implemented using frameworks like Quartz provided by Spring. While the Linux `crontab` command is the simplest to use, it is not recommended. `crontab` requires managing schedulers separately for each server and, more importantly, does not offer clustering capabilities. In contrast, using a scheduling framework like Quartz provides various benefits, including clustering, diverse scheduling features, and execution history management.

## Understanding Spring Boot Batch
A typical batch scenario consists of the following three steps:
1. Read: Reads specific data records from a data store (typically a database).
2. Process: Transforms/processes the data as desired.
3. Write: Saves the modified data back to the data store (database).

Batch processing follows a Read -> Process -> Write flow. The following diagram illustrates how Spring implements this batch processing and shows the relationships between related objects.

![](https://github.com/cheese10yun/TIL/raw/master/assets/batch-obejct-relrationship.png)
- Job and Step have a 1:M relationship.
- Step, ItemReader, ItemProcessor, ItemWriter have a 1:1 relationship.
- A single large task (Job) has multiple stages (Step), and each stage is configured according to the basic flow of batch processing.

### Job
- A Job is an object that represents a batch processing sequence as a single unit. It is always at the highest level of the overall batch process.
- As explained above, a single Job can contain multiple Steps. In Spring Batch, a Job object is a **container** that holds multiple Step instances.
- There are several builders for creating Job objects. You can easily create the desired Job using `JobBuilderFactory`, which is a factory that integrates various builders.

```java
public class JobBuilderFactory {
    private JobRepostiroy jobrepository;

    public JobBuilderFactory(JobRepository jobRepository){
        this.jobrepository = jobrepository;
    }

    public JobBuilder get(String name){
        JobBuilder builder = new JobBuilder(name).repository(jobrepository);
        return builder;
    }
}
```
- `JobBuilderFactory` includes a `get()` method that can create a `JobBuilder`. The `get()` method creates and returns a new `JobBuilder`.
- All `JobBuilder`s created by `JobBuilderFactory` use the repository.
- `JobBuilderFactory` only plays the role of creating `JobBuilder`s. To create a Job using the `JobBuilder` thus created, let's examine the functionalities of `JobBuilder` through its methods.

```java
public SimpleJobBuilder start(Step step){
    //(1)
    // Adds a Step to create the most basic SimpleJobBuilder.
    return new SimpleJobBuilder(tihs).start(step);
}

public JobFlowBuilder start(Flow flow){
    //(2)
    // Creates a JobFlowBuilder to execute a Flow.
    return new JobFlowBuilder(tihs).start(flow);
}

public JobFlowBuilder flow(Step step){
    //(3)
    // Creates a FlowJobBuilder to execute a Step.
    return new JobFlowBuilder(tihs).start(step);
}
```
`JobBuilder` does not directly create a Job but instead creates and returns a separate concrete builder, allowing for flexible handling of different Job creation methods depending on the case.

### JobInstance
A `JobInstance` is a single execution unit of a Job when it runs in batch processing. If a batch Job runs once a day, yesterday's and today's executions can each be called a `JobInstance`.

Each `JobInstance` does not necessarily have only one `JobExecution`. If a Job runs today and fails, it will run again tomorrow with the same `JobInstance`.

This is because a failed Job execution is not considered to have completed the `JobInstance`. In this case, the `JobInstance` will have two `JobExecution`s: yesterday's failed one and today's successful one. In other words, a **JobExecution can have multiple instances.**

### JobExecution
An object representing a single execution of a `JobInstance`.

If a Job fails today and is run again tomorrow, both today's and tomorrow's executions will use the same `JobInstance`.

In fact, the `JobExecution` interface contains domain objects that hold information about the Job execution.

`JobExecution` contains information such as the `JobInstance`, batch execution status, start time, end time, and failure messages. The `JobExecution` object includes various execution details.

### JobParameters
`JobParameters` is an object that specifies parameters required when a Job runs, in a Map type.

`JobParameters` also serves as a criterion for distinguishing `JobInstance`s.

`JobParameters` and `JobInstance` have a 1:1 relationship.

### Step
A `Step` is a domain object that contains all the necessary information to define and control actual batch processing. It serves as the practical unit for processing a Job.

Every Job must have at least one Step.

### StepExecution
Just as a Job has `JobExecution` for Job execution information, a Step has a `StepExecution` object that contains Step execution information.

### JobRepository
`JobRepository` is a mechanism that stores batch processing information. It stores metadata about batch processing, such as which Job was executed, how many times it ran, and when it finished.

For example, when a Job runs, `JobRepository` creates a `JobExecution` domain object containing information related to the batch execution.

`JobRepository` also stores `StepExecution`, which contains the execution information of a Step, in the repository, playing a role in storing and managing overall metadata.

### JobLauncher
`JobLauncher` is an interface that executes a batch with a Job and `JobParameters`.

### ItemReader
`ItemReader` is an interface that reads batch data to be processed by a Step. It can read various types of data, such as files, XML, or databases.

### ItemProcessor
`ItemProcessor` transforms batch data read by `ItemReader`. The reasons for separating this are as follows:

- Separation of business logic: `ItemWriter` performs saving, and `ItemProcessor` only performs logic processing, clearly separating their roles.
- It can handle cases where the type of batch data read and the type of data to be written are different.

### ItemWriter
`ItemWriter` saves batch data. Typically, it saves to a database or a file.

`ItemWriter` implements a similar approach to `ItemReader`. It receives the desired type via generics, and its `write()` method takes a `List` of that type as a parameter for saving.
