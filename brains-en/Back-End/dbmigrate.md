# Large-Scale DB and Data Migration (DB & Storage)

When changing schemas or migrating large volumes of data in a zero-downtime environment, it's an engineering task that requires achieving both system availability and data integrity simultaneously.

Let's explore migration strategies based on technical considerations and architecture, assuming an environment with resource constraints.

### Sharding Strategy and Zero-Downtime Migration

**Sharding** is an architecture that horizontally partitions data across multiple independent physical databases to overcome the limitations of traffic and data capacity concentrated on a single database node. It typically uses **hash function partitioning** based on specific column values or **range-based partitioning.**

**Zero-Downtime Migration** is the process of safely replicating data from a legacy source database to a new sharded cluster and then switching traffic, without blocking or delaying read and write transactions of a live service.

### CDC Delta Loss and Data Inconsistency

When migrating terabytes of data in an environment with thousands of transactions per second, the specific failure scenarios encountered are as follows:

**Delta loss during snapshot replication**: Dumping a snapshot from the SourceDB to the TargetDB for initial data loading can take tens of hours or more. If data changes (additions, modifications, deletions) in the Source DB during this time are not reflected in the TargetDB, critical data inconsistency will occur.

**State inconsistency due to dual-write failure**: When performing a dual write, where data is written to both Source and Target at the application layer, a timeout due to a bottleneck in the TargetDB (e.g., 100% CPU or lock contention) can lead to a partial failure state where data is written to the Source but not to the Target.

**CDC pipeline delay (Replication Lag)**: When using CDC tools like Debezium to read the Source's binlog and asynchronously replicate to the Target, replication lag can gradually increase due to write performance limitations on the Target side. If lag remains at the time of traffic cutover, it can lead to issues where outdated data is retrieved.

### Example

Migration typically proceeds in 4 phases: Source Only -> Dual Write -> Read Target -> Target Only. Below is an example of transaction handling to maintain consistency during the dual write phase.

```kt
import org.springframework.stereotype.Service
import org.springframework.transaction.annotation.Transactional

@Service
class MigrationUserService(
    private val legacyUserRepository: LegacyUserRepository, // Source DB
    private val shardUserRepository: ShardUserRepository    // Target DB
) {
    // Migration Phase: DUAL_WRITE (Source as main, Target as asynchronous or Fallback processing)
    @Transactional
    fun createUserDualWrite(userData: UserData): User {
        // 1. Write to Source DB (Single Source of Truth)
        val savedUser = legacyUserRepository.save(userData.toLegacyEntity())

        // 2. Attempt to write to Target DB (Shard)
        try {
            // Isolate exceptions so that Target DB write failures do not lead to Source DB transaction rollbacks.
            // In resource-constrained situations, asynchronous processing (Event Publisher) is safer.
            shardUserRepository.save(userData.toShardEntity(savedUser.id))
        } catch (e: Exception) {
            // If Target DB write fails, log the record's ID and change history
            // to a separate Dead Letter Queue (Kafka) or retry table,
            // and reconcile consistency with a batch job after snapshot loading.
            migrationFailureLogger.logFailedSync(savedUser.id, userData)
        }

        return savedUser
    }
}
```

Let's configure a multi-data source environment.

-   **Multi-datasource integration**: This technique involves excluding Spring Boot's default single DB auto-configuration and explicitly registering two or more database connection pools, entity managers, and transaction managers as beans, allowing a single application to control physically separated databases.
-   **Package-separated routing**: This pattern involves separating and mapping repository entities for the source DB and target DB into distinct physical directory structures, so JPA knows which DB to query.

There are also issues with bean conflicts and transaction boundary problems that arise during configuration. When setting up multiple DBs, auto-configuration can sometimes conflict. Since Spring Boot expects a single data source, simply providing two URLs in YAML will cause a `NoUniqueBeanDefinitionException` and server startup will fail.

Transaction managers can also cause confusion; if you don't explicitly specify which DB's transaction to control when using the `@Transactional` annotation, you might encounter consistency issues where a transaction is initiated on the source DB, a write to the target DB fails, and the source transaction isn't rolled back.

**Connection pool exhaustion** can also occur. Maintaining two connection pools (HikariCP) in server memory means twice the number of connection objects are created compared to existing traffic, risking memory overload and exceeding the DB's max connections limit.

First, to customize by excluding the automatic data source configuration class, enter the source and target DB information in YAML.

```yaml
spring:
  # Optional setting to prevent DataSource creation by auto-configuration
  autoconfigure:
    exclude: org.springframework.boot.autoconfigure.jdbc.DataSourceAutoConfiguration

app:
  datasource:
    source:
      jdbc-url: jdbc:mysql://source-db.internal:3306/legacy_db
      username: mig_user
      password: secure_password
      driver-class-name: com.mysql.cj.jdbc.Driver
      hikari:
        pool-name: Source-HikariCP
        maximum-pool-size: 30
    target:
      jdbc-url: jdbc:mysql://target-db.internal:3306/shard_db
      username: mig_user
      password: secure_password
      driver-class-name: com.mysql.cj.jdbc.Driver
      hikari:
        pool-name: Target-HikariCP
        maximum-pool-size: 30
```

For one of the two data sources, be sure to add `@Primary` to designate it as the default, so the framework doesn't get confused.

```kt
import com.zaxxer.hikari.HikariDataSource
import org.springframework.boot.context.properties.ConfigurationProperties
import org.springframework.boot.jdbc.DataSourceBuilder
import org.springframework.context.annotation.Bean
import org.springframework.context.annotation.Configuration
import org.springframework.context.annotation.Primary
import org.springframework.data.jpa.repository.config.EnableJpaRepositories
import org.springframework.orm.jpa.JpaTransactionManager
import org.springframework.orm.jpa.LocalContainerEntityManagerFactoryBean
import org.springframework.orm.jpa.vendor.HibernateJpaVendorAdapter
import javax.sql.DataSource

@Configuration
@EnableJpaRepositories(
    basePackages = ["com.example.migration.repository.source"], // Repository path for Source DB
    entityManagerFactoryRef = "sourceEntityManagerFactory",
    transactionManagerRef = "sourceTransactionManager"
)
class SourceDataSourceConfig {

    @Primary
    @Bean(name = ["sourceDataSource"])
    @ConfigurationProperties(prefix = "app.datasource.source")
    fun sourceDataSource(): DataSource {
        return DataSourceBuilder.create().type(HikariDataSource::class.java).build()
    }

    @Primary
    @Bean(name = ["sourceEntityManagerFactory"])
    fun sourceEntityManagerFactory(): LocalContainerEntityManagerFactoryBean {
        val em = LocalContainerEntityManagerFactoryBean()
        em.dataSource = sourceDataSource()
        em.setPackagesToScan("com.example.migration.entity.source") // Entity path for Source DB
        
        val vendorAdapter = HibernateJpaVendorAdapter()
        em.jpaVendorAdapter = vendorAdapter
        // Use em.setJpaPropertyMap() for additional Hibernate settings (dialect, etc.)
        return em
    }

    @Primary
    @Bean(name = ["sourceTransactionManager"])
    fun sourceTransactionManager(): JpaTransactionManager {
        val transactionManager = JpaTransactionManager()
        transactionManager.entityManagerFactory = sourceEntityManagerFactory().`object`
        return transactionManager
    }
}
```

The above is the source (default) database configuration, and below is the target. You can use the exact same configuration, just remove `@Primary` and change the scan package path.

```kotlin
// ... import omitted ...

@Configuration
@EnableJpaRepositories(
    basePackages = ["com.example.migration.repository.target"], // Repository path for Target DB
    entityManagerFactoryRef = "targetEntityManagerFactory",
    transactionManagerRef = "targetTransactionManager"
)
class TargetDataSourceConfig {

    @Bean(name = ["targetDataSource"])
    @ConfigurationProperties(prefix = "app.datasource.target")
    fun targetDataSource(): DataSource {
        return DataSourceBuilder.create().type(HikariDataSource::class.java).build()
    }

    @Bean(name = ["targetEntityManagerFactory"])
    fun targetEntityManagerFactory(): LocalContainerEntityManagerFactoryBean {
        val em = LocalContainerEntityManagerFactoryBean()
        em.dataSource = targetDataSource()
        em.setPackagesToScan("com.example.migration.entity.target") // Entity path for Target DB
        em.jpaVendorAdapter = HibernateJpaVendorAdapter()
        return em
    }

    @Bean(name = ["targetTransactionManager"])
    fun targetTransactionManager(): JpaTransactionManager {
        val transactionManager = JpaTransactionManager()
        transactionManager.entityManagerFactory = targetEntityManagerFactory().`object`
        return transactionManager
    }
}
```

#### Subsequent Transaction Management and the Limitations of JTA (Java Transaction API)

Configuring as above allows JPA to route queries to different DBs based on packages, but there's a significant limitation encountered during the migration phase.

**Lack of global transactions**: When inserting all data into both the source DB and target DB within a single method, `@Transactional` cannot group the two DBs into a single unit of work. This means that even if an error occurs during target DB saving, the source cannot be rolled back using only that annotation.

**Avoid chained transactions**: In the past, a chaining approach was used to link two transaction managers, but it has been deprecated in the Spring ecosystem due to recovery complexities and uncertainties.

Rather than configuring heavy distributed 2PC transactions, let's ensure eventual consistency by making the source succeed and, upon target DB failure, logging to Kafka or a retry table for asynchronous reconciliation, as shown in the previous example.

### DB Status and Query Performance Monitoring

In a resource-constrained environment, it's crucial to continuously monitor in real-time whether migration tools or application queries are impacting existing services.

Below are some DB queries for monitoring, based on MySQL (MariaDB).

-   `SHOW PROCESSLIST` or `SELECT * FROM information_schema.processlist WHERE command != 'SLEEP';`
    -   This query checks currently running threads and queries in the DB. It helps identify queries with abnormally high `Time` values or those stuck due to `Waiting for table metadata lock`, thereby detecting lock contention caused by DDL or large DML operations.
-   `EXPLAIN ANALYZE SELECT ...`
    -   This query goes beyond simply viewing the execution plan; it actually executes the query and returns the time spent and rows processed at each stage. It's used to precisely tune performance bottlenecks for migration validation queries or newly written queries after sharding.
-   `SHOW ENGINE INNODB STATUS\G`
    -   This outputs the internal status of the InnoDB storage engine. It allows for detailed diagnosis of transaction deadlocks, buffer pool memory usage efficiency, and disk I/O bottlenecks.

### Application-Level Trade-offs After Resolution

Even after successfully completing the migration and switching to a sharded cluster, new problems arise due to architectural limitations.

-   **Cross-Shard Join Not Possible**: SQL-level joins are not possible between databases residing on different physical machines. To address this, data must be fetched separately into application memory for joining, or the need for joins must be eliminated entirely through denormalization that allows for data duplication.
-   **Increased Distributed Transaction Complexity**: When data needs to be modified across two or more shards, the ACID transaction guarantees provided by a single DB become impossible. Instead of 2PC, which causes performance degradation, consider adopting patterns like Saga or Outbox to ensure eventual consistency.
-   **Limitations of Auto Increment IDs**: If multiple shards independently use auto-incrementing IDs, primary keys can become duplicated. Therefore, a separate global unique identifier generator for distributed environments, based on the Snowflake algorithm or UUIDs, must be built.
