# 대규모 DB 및 데이터 마이그레이션 (DB & Storage)

무중단 환경에서 스키마를 변경하거나 데이터를 대규모로 이관해야할 때는 시스템의 가용성과 데이터 무결성을 동시에 달성해야하는 엔지니어링 작업이다.

리소스 제약이 존재하는 환경을 가정하고 기술적 사고와 구조에 기반한 마이그레이션 전략을 알아보자.

### 샤딩 전략과 무중단 이관

**샤딩**은 단일 데이터베이스 노드에 집중되는 트래픽과 데이터 용량의 한계를 극복하기 위해 데이터를 여러 개의 독립적인 물리적 데이터베이스로 수평 분할하는 아키텍처이다. 일반적으로 특정 컬럼 값을 기준으로 하는 **해시 함수 분할**이나 **범위Range기반 분할을 사용한다.**

**무중단 데이터 이관 Zero-Downtime Migration**은 라이브 서비스의 읽기 및 쓰기 트랜잭션을 차단하거나 지연시키지 않고, 레거시 데이터베이스 source의 데이터를 새로운 샤드 클러스터로 안전하게 복제한 뒤 트래픽을 전환하는 과정이다.

### 데이터 변경분 CDC 누락 및 정합성 깨짐

초당 수천건의 트랜잭션이 발생하는 환경에서 테라바이트 단위의 데이터를 이관할 때 직면하는 구체적인 장애 상황은 다음과 같다.

**스냅샷 복사 중에 발생하는 Delta 누락**: 초기 데이터 적제를 위해 SourceDB의 스냅샷을 TargetDB로 덤프하는데 수십 시간 이상이 소요될 수 있다. 이 시간동안 Source DB에 추가 수정 삭제된 데이터 변경분이 TargetDB에 반영되지 않으면 치명적인 데이터 정합성 불일치가 발생한다.

**이중 쓰기 실패에 따른 상태 불일치**: 애플리케이션 계층에서 Source와 Target 양쪽에서 데이터를 기록하는 이중쓰기 dual write를 진행할 때, TargetDB의 병목(cpu 100% or Lock 경합)으로 인해 타임아웃이 발생하면 Source에는 데이터가 기록되고 Target에는 기록되지 않은 부분 실패 상태가 된다.

**CDC 파이프라인 지연(Replication Lag)**: Debezium 같은 CDC를 활용해 Source의 binlog를 읽어 Target으로 비동기 복제할때 Target 쪽의 쓰기 성능 한계로 인해 복제 지연 시간 Lag가 점진적으로 증가할 수 있다. 트래픽 전환 시점에 Lag가 남아있으면 과거 데이터가 조회되는 문제가 발생한다.

### Example

마이그레이션은 보통 4단계 Source Only -> Dual Write -> Read Target -> Target Only로 진행된다. 아래는 dual write 단계에서 정합성을 방어하기 위한 트랜잭션 처리 예시다.

```kt
import org.springframework.stereotype.Service
import org.springframework.transaction.annotation.Transactional

@Service
class MigrationUserService(
    private val legacyUserRepository: LegacyUserRepository, // Source DB
    private val shardUserRepository: ShardUserRepository    // Target DB
) {
    // Migration Phase: DUAL_WRITE (Source를 메인으로, Target은 비동기 또는 Fallback 처리)
    @Transactional
    fun createUserDualWrite(userData: UserData): User {
        // 1. Source DB에 쓰기 (단일 진실 공급원)
        val savedUser = legacyUserRepository.save(userData.toLegacyEntity())

        // 2. Target DB (Shard)에 쓰기 시도
        try {
            // Target DB 쓰기 실패가 Source DB 트랜잭션 롤백으로 이어지지 않도록 예외를 격리합니다.
            // 리소스 제약 상황에서는 비동기(Event Publisher)로 처리하는 것이 더 안전합니다.
            shardUserRepository.save(userData.toShardEntity(savedUser.id))
        } catch (e: Exception) {
            // Target DB 쓰기 실패 시, 해당 레코드의 ID와 변경 내역을 
            // 별도의 Dead Letter Queue(Kafka)나 재시도(Retry) 테이블에 기록하여 
            // 스냅샷 적재 이후 배치 잡(Batch Job)으로 정합성을 맞춥니다.
            migrationFailureLogger.logFailedSync(savedUser.id, userData)
        }

        return savedUser
    }
}
```

다중 데이터 소스 환경 구성을 해보자

- **다중 데이터소스 연동**: spring boot가 기본적으로 제공하는 단일 db 자동 설정 (auto configuration)을 배제하고 개발자가 명시적으로 두 개 이상의 데이터베이스 커넥션 풀, 엔티티 매니저, 트랜잭션 매니저를 빈으로 등록해 물리적으로 분리된 디비를 하나의 애플리케이션에서 제어하는 기법이다.
- **패키지 분리 라우팅**: JPA가 어떤 db에 쿼리를 날려야할지 알수있도록 source db용 repository entity와 target db용 repository entity를 물리적인 디렉터리 구조로 분리하여 매핑하는 패턴이다.

설정 시 발생하는 빈 충돌 및 트랜잭션 경계 문제도 존재하는데 다중 db를 세팅할때 auto configuration이 충돌나는 경우가 있다. spring boot는 단일 data source를 기대하기 때문에 yaml에 url만 두개 두면 `NoUniqueBeanDefinitionException`을 발생시키며 서버 기동이 실패한다.

트랜잭션 매니저도 혼선될 수 있는게 `@Transactional` 어노테이션 사용시 어떤 디비의 트랜잭션을 제어할지 명시하지 않으면 source db에 트랜잭션을 걸어놓고 target db에 쓰기를 시도하다가 롤백이 되지 않는 정합성 이슈가 발생할 수 있다.

**커넥션 풀 고갈**이 될수도 있는게 서버 메모리에 두개의 커넥션(히카리풀)을 유지해 기존 트래픽 대비 커넥션 객체가 2배로 생성되어 메모리 부하 및 db측의 max connections 한계를 초과할 위험이 있다.

일단 데이터 세팅 자동 config 클래스를 제외하고 커스터마이징하기위해서 yaml에 소스와 target db 정보들을 입력한다.

```yaml
spring:
  # 자동 설정에 의한 DataSource 생성을 방지하기 위한 선택적 옵션
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

두 개의 데이터소스중 하나는 반드시 `@Primary`를 붙여서 프레임워크가 헷갈리지 않도록 기본값을 지정해주자

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
    basePackages = ["com.example.migration.repository.source"], // Source DB용 레포지토리 경로
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
        em.setPackagesToScan("com.example.migration.entity.source") // Source DB용 엔티티 경로
        
        val vendorAdapter = HibernateJpaVendorAdapter()
        em.jpaVendorAdapter = vendorAdapter
        // 추가적인 Hibernate 설정(방언 등)은 em.setJpaPropertyMap() 활용
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

위에가 source (기본값) 데이터베이스 설정이고 아래가 타겟이다. `@Primary`를 제거한 버전으로 스캔 패키지 경로만 바꿔서 그대로 똑같이 쓰면 된다.

```kotlin
// ... import 생략 ...

@Configuration
@EnableJpaRepositories(
    basePackages = ["com.example.migration.repository.target"], // Target DB용 레포지토리 경로
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
        em.setPackagesToScan("com.example.migration.entity.target") // Target DB용 엔티티 경로
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

#### 이후 트랜잭션 관리와 JTA(JAva Transaction API)의 한계

위와 같이 구성하면 패키지에 따라 jpa가 알아서 다른 db로 쿼리를 분기하는데, 하지만 마이그레이션 단계에서 가장 크게 부딪히는 한계가 있다.

**글로벌 트랜잭션 부재**: 하나의 메서드 안에서 source db와 target db에 모든 데이터를 넣을때 `@Transactional`로는 두 디비를 하나의 작업단위로 묶을 수 없다. 즉 target db 저장중 에러가나도 source 롤백을 저 어노테이션만으로는 할 수 없음.

**체인 트랜잭션 지양**: 과거에는 두 트랜잭션 매니저를 묶는 체인 방식을 썼으나, 장애 복구와 불확실성 때문에 spring 생태계에서는 deprecated 되었다.

무거운 분산 트랜잭션 2pc를 구성하기 보다는 아까 예제처럼 source만 성공시키고 target db 실패시 kafka나 재시도 테이블에 기록해 비동기로 정합성을 맞춰 eventually consistency하게 동작시키자.

<br>

### DB 상태 및 쿼리 성능 모니터링

리소스가 제한된 호나경에서 마이그레이션 툴이나 애플리케이션의 쿼리가 기존 서비스에 영향을 주지 않는지 실시간으로 확인해야한다.

아래는 MySQL(MariaDB) 기준으로 모니터링하는 db 쿼리등을 소개한다.

- `SHOW PROCESSLIST` 또는 `SELECT * FROM information_schema.processlist WHERE command != 'SLEEP';`
  - 위 쿼리는 현재 디비에서 실행중인 스레드와 쿼리를 확인하는 것으로 Time 값이 비정상적으로 높거나 `Waiting for table metadata lock` 등으로 멈춰있는 쿼리를 식별해 DDL이나 대량 DML로 인한 락 경합을 파악한다.
- `EXPLAIN ANALYZE SELECT ...`
  - 단순히 실행계획 explain을 보는것을 넘어 실제로 쿼리를 실행하고 각 단계별 소요 시간과 처리된 행 수를 반환하는 쿼리인데, 이관 검증 쿼리나 샤딩 이후에 새롭게 작성도니 쿼리의 성능 병목을 정확하게 튜닝할 때 사용한다.
- `SHOW ENGINE INNODB STATUS\G`
  - InnoDB 스토리지 엔진의 내부 상태를 출력한다. 트랜잭션 데드락 내역, 버퍼풀의 메모리 사용 효율, 디스크 IO 병목 여부를 상세히 진단할 수 있다.

### 해결 이후 애플리케이션 레벨의 트레이드 오프

마이그레이션을 성공적으로 마치고 샤딩 클러스터로 전환하더라도 구조적 한계로 인한 새로운 문제들이 발생한다.

- **크로스 샤드 조인(Cross-Shard Join) 불가**: 서로 다른 물리 장비에 있는 데이터베이스간에는 sql 레벨의 join이 불가능해 이를 해결하기 위해 애플리케이션 메모리에서 데이터를 각각 가져와 조인하거나 중복을 허용하는 비정규화를 통해 조인 필요성 자체를 제거해야한다.
- **분산 트랜잭션 복잡성 증가**: 두 개 이상의 샤드에 걸쳐 데이터를 수정해야할 때, 단일 디비에서 제공하던 ACID 트랜잭션 보장이 불가능해진다. 성능 저하를 유발하는 2PC보다는 eventually consistency를 보장하는 saga나 아웃박스 패턴을 도입하자.
- **Auto Increment ID의 한계**: 여러 샤드에서 독립적으로 A-I를 사용하면 primary key가 중복되고 따라서 Snowflake 알고리즘 혹은 UUID 기반의 분산 환경용 글로벌 유일 식별자 생성기를 별도로 구축해야한다.