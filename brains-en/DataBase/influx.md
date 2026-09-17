# InfluxDB

InfluxDB is an open-source time-series database developed in 2013 to handle write operations and query loads.

It is one of the essential components of the TICK stack (Telegraf, InfluxDB, Chronograf, Kapacitor). Among many TSDBs (Prometheus, imtescaledb, Graphite, etc.), it is the most well-known.

It is configured to be distributed and scale horizontally, allowing for easy scale-out by simply adding new nodes, and it provides a RESTful API.

- **Telegraf**: Metrics and event collection and reporting module
- **InfluxDB**: Time-series database
- **Kapacitor**: Real-time streaming data processing engine
- **Chronograf**: Visualization tool

InfluxDB provides core features such as Continuous Query (Task) and Retention Policy (Retention Period).

These can be described as the ability to process data at regular intervals and store it anew, and the ability to automatically delete data at regular intervals.

#### Continous Query, Task

The purpose of InfluxDB is to process time-based data, i.e., time-series data.

InfluxDB provides continuous queries that allow downsampling, which processes data and stores it anew, to run at regular intervals.

InfluxDB 2 provides tasks that replace continuous queries.

#### Retention Policy, Period

This is called a retention policy. Since the core purpose of InfluxDB is to insert and query time-based data, `DELETE` operations are rarely used.

However, as data continuously accumulates, issues with storage space and processing speed can arise. Therefore, InfluxDB supports retention policies that automatically delete data.

A Retention Policy, as the name suggests, is a policy that automatically deletes old data. It is defined at the database level, and typically, one database can have multiple retention policies.

If no separate settings are made, the default policy called `autogen` is applied. Since `autogen` has an unlimited retention period, data will continuously accumulate and cause problems if no specific settings are configured.

Therefore, it is necessary to manage old data by configuring separate settings.

InfluxDB 2 provides periods that replace retention policies.

## InfluxDB Internal Structure and Configuration

The differences in InfluxDB's structure compared to an RDB are as follows.

| InfluxDB | RDB(Relational Database) |
| :--- | :--- |
| **Database** | Database |
| **Measurement** | Table |
| **Tag Key** | Indexed Column (String Only) |
| **Field Key** | Unindexed Column |
| **Column** | Column |
| **Point** | Row |

### Measurement

A measurement, which carries the meaning of a measurement in a time-series database.

It plays a role similar to a table in a relational database, and like an RDB, multiple measurements can exist within a database.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdna%2FPjvOf%2FbtreQ9J4vcR%2FAAAAAAAAAAAAAAAAAAAAANl3h1nFHaEKhrgK4Z_No7HujO9eV5gVakJN8kECy1WG%2Fimg.png%3Fcredential%3DyqXZFxpELC7KVnFOS48ylbz2pIh7yKj8%26expires%3D1774969199%26allow_ip%3D%26allow_referer%3D%26signature%3D5kOo3kLp8iyV1kyTlUxwF9xNQYM%253D)

There is a core concept that differentiates it from RDBs: InfluxDB is built on the NoSQL concept and is characterized by being schemaless.

When developing with traditional RDBs, one had to design the necessary columns, their lengths, and types to configure the table schema. However, since InfluxDB is schemaless, at the moment new data is added,

columns related to the measurement are added, making schema changes very fast. By adopting this structure, InfluxDB allows for flexible handling of time-series data.

### Key(tag key, filed key, time key)

The column structure differs from traditional RDBs. While traditional RDBs store one piece of data per column,

in TSDBs, columns are divided into three types, and each column is composed of key-value pairs.

- **Tag Key**
  - Similar to an indexed column in an RDB, it is indexed and serves as the basis for querying with `SELECT` statements.
  - Only string types are allowed for tag values, and they must be enclosed in single quotes `''` when queried.
- **Field Key**
  - Similar to an unindexed column in an RDB, stored data must have at least one field.
  - Field values can be strings, floats, integers, or booleans, and once a type is set, it cannot be changed.
- **Time Key**
  - Automatically entered in microseconds, representing the time elapsed since January 1, 1970, 00:00:00 UTC.
  - It can be set manually, but it is not recommended.

To store data, you must choose between a tag or a field, and this is very important.

This is because values in tags are indexed, whereas fields are not. Therefore, if indexing is required for querying, use a tag.

Otherwise, if it's just simple data, you can choose a field.

If unnecessary data is also captured as tags, the index structure will become bloated, consuming a lot of memory and degrading processing speed.

### Series

A series is a concept unique to InfluxDB and corresponds to a collection of combinable tag keys.

For example, if `(name, age)` are designated as tag keys in a `member` measurement, then the set of all possible `(name, age)` combinations among the stored data constitutes a series.

### Shard, ShardGroup

A Shard Group belongs to an InfluxDB bucket and manages shards that store actual data according to the shard group duration.

Shards store data belonging to the period defined in the shard group, encoded and compressed.

All points (data) stored within a specific shard group duration are saved in the same shard.

A single shard consists of multiple series and a time-structured merge tree (TSM) on disk.

TSM can be thought of as a tree-based data store used internally. Data resides in memory before being stored in TSM.

When the shard group duration expires, data is stored on disk in TSM format.

InfluxDB uses shard groups and shards to shard data, adopting an approach that increases throughput and overall performance even as data grows over time.

Shards hold temporary blocks for data, which are mapped to TSM.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdna%2FpmQn2%2Fbtre093xq2w%2FAAAAAAAAAAAAAAAAAAAAACJUPEWjsyy0ufNyM1Bn-W4m0PHV6wav7qUjR3JA-F5Z%2Fimg.png%3Fcredential%3DyqXZFxpELC7KVnFOS48ylbz2pIh7yKj8%26expires%3D1774969199%26allow_ip%3D%26allow_referer%3D%26signature%3Dc%252B5fGZbllhee5VQTpfvwlNGZMsI%253D)

By using shards separated based on time and using time as a constraint, the amount of data to be loaded into memory can be reduced.

This is a fundamental concept adopted by TSDBs. https://docs.influxdata.com/influxdb/v2/reference/internals/shards/#shard-group-diagram

A shard group is a logical container divided by time periods, and InfluxDB needs to automatically discard data according to its retention period.

Therefore, if data is grouped by time units, management becomes easier, and it is defined to facilitate proper discarding according to the retention policy.

It's a 1:N relationship between groups and shards. The reason it's not 1:1 is to allow for multiple shards within a shard group, perhaps for replicas or distribution.

In summary, a shard group is a time-range container, and a shard is a physical file, stored in TSM format.

#### Shard Group Duration

Shard group duration refers to the period during which a shard group is retained in memory and new ones are created.

If this retention period passes, the shard group is removed. By default, InfluxDB sets the shard group duration based on the bucket's retention period.

If no separate settings are configured, the shard group duration is 7 days by default.

- If the Retention Period is less than 2 days, the shard group duration is 1 hour.
- If it is between 2 days and 6 months, it is 1 day.
- If it is greater than 6 months, it is 7 days.

Shard group duration and retention period can be a bit confusing.

For example, if the retention period is infinite, but the shard group duration is 7 days, one might wonder if data is deleted when the shard group duration expires or if it remains.

The shard group duration is the time data resides in memory. If the shard group duration expires, the data is moved from memory to disk.

It's easier to think of it as a cache. Therefore, if the retention period is infinite, the data is stored permanently, but if the shard group duration has ended, you would need to read the values from disk to query that data.

In other words, shard retention feels like a memory TTL, while retention period is the actual lifespan of the data.
