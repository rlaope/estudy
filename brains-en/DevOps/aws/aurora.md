# Amazon Aurora, Aurora Architecture

## Aurora

Amazon Aurora is a MySQL and PostgreSQL-compatible relational database built for the cloud, combining the performance and availability of high-performance commercial databases with the simplicity and cost-effectiveness of open-source databases.

It is up to 5 times faster than a standard MySQL database and 3 times faster than a standard PostgreSQL database.

It provides the security, availability, and reliability of commercial databases at one-tenth the cost.

Amazon Aurora is fully managed by Amazon Relational Database Service (RDS), which automates time-consuming administrative tasks such as hardware provisioning, database setup, patching, and backups.

<br>

## Aurora Architecture

### Single-Master

![](https://user-images.githubusercontent.com/28394879/141725880-66b8285c-0f21-45e1-a751-20ab4f5d92b7.png)

### Multi-Master

![](https://user-images.githubusercontent.com/28394879/141935073-90caf347-f14e-4603-a77b-e02e83b1d49d.png)

<br>

## Features of Aurora

MySQL/PostgreSQL support

Two modes
- Multi-Master, which allows read and write operations with multiple nodes
- Single-Master, configured with one write-only node and multiple read-only nodes (Aurora Replicas)

Automatic capacity scaling: Starts from 10GB and increases in 10GB increments (up to 128TB)

Compute capacity: Can scale up to 96 vCPUs and 768GB (db.r5.24.xlarge)

Distributed data storage: 2 data replicas stored per AZ x minimum 3 AZs = minimum 6 replicas
- Maintains write capability until 3 or more replicas are lost
- Maintains read capability until 4 or more replicas are lost
- Lost replicas are self-healing: Continuously checks for and recovers lost parts
- Uses a Quorum model

<br>

## Single-Master Mode

Consists of one Writer instance and multiple read-only instances

A total of 15 Replicas can be created

Async replication

Can be created within a single region

If the Writer goes down, one of the Replicas automatically fails over to become the Writer -> can be promoted to main without data loss during failover

Ensures high availability

<br>

## Aurora Global Database

Allows data access with sub-second latency from all regions worldwide

Can be used for disaster recovery
- In case of emergency, one of the secondary regions can be promoted to main
- 1-second RPO (Recovery Point Objective)
- Sub-1-minute RTO (Recovery Time Objective)

A total of 16 read-only nodes can be created in secondary regions (originally 15)

![](https://user-images.githubusercontent.com/28394879/141939570-65e285fb-8ffc-447c-afc3-9a29d3c00a20.png)

<br>

## Parallel Query

A mode that processes queries in parallel through multiple read nodes
- Fast
- Load balancing (CPU, Memory)

Only supported in MySQL 5.6/5.7.

Not supported on remaining instances (e.g., db.t2, db.t3).

<br>

## Aurora Backup

Supports Read Replicas (a different concept from Aurora Replicas)
- Binary log replication for MySQL DB
- However, can only be created in different regions

Similar to RDS, automatic/manual backups are possible
- Automatic backups are retained for 1 to 35 days (stored in S3)
- Manual backups (snapshots) are possible
- Restoring backup data creates a new database

<br>

## Aurora Database Cloning

Replicates a new database from an existing database
- Faster and cheaper than creating a new database via snapshots

Uses Copy-On-Write protocol
- Works correctly even if the original cluster is deleted

<br>

## Backtrack

Reverts an existing DB to a specific point in time (the existing DB, not a new one)

- Easily recovers from DB management mistakes (e.g., `DELETE` without a `WHERE` clause)
- Much faster than creating a new DB
- Can move forward and backward in time, allowing quick navigation to the desired point

Backtrack Window
- Target Backtrack Window
  - How much data to store to revert the DB to a certain point in time
    - Cannot backtrack before the specified point
- Actual Backtrack Window
  - How far back in time can actually be reverted
  - Must be smaller than the Target Backtrack Window

When Backtrack is enabled, it stores DB changes per hour
- Costs are incurred based on the stored capacity
- More DB changes mean more logs = higher costs
- If DB logs are too numerous and the Actual Backtrack Window becomes smaller than the Target Backtrack Window setting, a notification is issued.

Only available for MySQL

Only DBs configured with Backtrack during Aurora creation can use Backtrack
- The feature can be activated by restoring a snapshot or cloning

Backtrack is not possible in Multi-Master state

<br>

## Multi-Master

Up to 4 nodes handle read and write operations
- Each node is independent: they do not affect each other during stops/reboots/deletions

Provides continuous availability

Primarily offers good performance for applications with Multitenant or Sharding
