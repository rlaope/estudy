# Apache Kafka Key Components: Producer, Consumer, Topic, Partition, Segment

## Topic, Producer, Consumer

![](https://velog.velcdn.com/images%2Fkidae92%2Fpost%2F4e6ab402-6016-4efd-b8ef-baf77a6627f7%2Fimage.png)

- Producer: An application that produces messages and sends them to a Kafka Topic.
- Consumer: An application that fetches and consumes messages from a Topic.
- Consumer group: A collection of Consumers that cooperate to consume messages from a Topic.
- A single Consumer belongs to one Consumer Group, and Consumers within a Consumer Group collaborate to process messages from a Topic in a distributed and parallel manner.

## Producer and Consumer Basic Operation

- Commit Log: An append-only, immutable data structure. Events (data) are always appended to the end of the log and are never modified.
- Offset: The position of an Event in the Commit Log. In the image below, you can see offsets from 0 to 10.

![](https://velog.velcdn.com/images%2Fkidae92%2Fpost%2F0d37ccaa-a260-4a6f-bda9-137448e57250%2Fimage.png)

Producers and Consumers are unaware of each other, and each performs writes and reads to the Commit Log at its own pace.

Consumers belonging to different Consumer Groups are unrelated to each other and can read Events from the Commit Log simultaneously at different positions.

## Event Position in Commit Log

![](https://velog.velcdn.com/images%2Fkidae92%2Fpost%2F521237d9-96bc-415e-b08b-dfbeac85725c%2Fimage.png)

A difference can occur between the LOG-END-OFFSET that the Producer writes and the CURRENT-OFFSET that a Consumer in a Consumer Group reads, processes, and then commits.

### Several issues when an offset difference occurs
1. Data loss
2. Duplicate processing
3. Ordering issues

## Logical View (Topic, Partition, Segment)

![](https://velog.velcdn.com/images%2Fkidae92%2Fpost%2Fa99600f1-d62b-44c8-83b3-22c9f858e8f%2Fimage.png)

- Topic: The place where messages are stored within Kafka, a logical representation.
- Partition: A Commit Log. A single Topic consists of one or more Partitions. Multiple Partitions are used for parallel processing.
- Segment: The actual physical file where messages are stored. When a Segment File exceeds a specified size or age, a new file is opened, and messages are appended to the new file.

## Physical View (Topic, Partition, Segment)
When creating a Topic, the number of Partitions is specified, and each Partition is distributed among Brokers and consists of Segment Files.

Rolling Strategy: `Log.segment.bytes(default 1 GB), log.roll.hour(default 168 hours)`

![](https://velog.velcdn.com/images%2Fkidae92%2Fpost%2Fcb5c4b43-e2c5-4189-a462-6e8aad46258b%2Fimage.png)

## One Active Segment per Partition
Only one Segment is active per Partition -> data is continuously being written.

![](https://velog.velcdn.com/images%2Fkidae92%2Fpost%2F523e209f-8df1-44c9-b922-ca19fe9bc4e7%2Fimage.png)
