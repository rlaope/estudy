# InfluxDB

influx db는 쓰기 작업과 쿼리부하를 처리하기위해 2013년도에 개발된 오픈소스 시계열 데이텁 ㅔ이스다

tick stack이라고 telegraf influxdb chronograf kapacitor의 필수 컴포넌트중 하나고. 많은 tsdb(prometheus, imtescaledb, graphite등)중에서 가장 유명하다.

distributed, scale horizontally하게 설정되어 새로운 노드만 추가하면 쉽게 scale out이 가능하고 restful api를 제공한다.

- **Telegraf**: Metrics, Event 수집 리포팅 모듈
- **InfluxDB**: 시계열 데이터베이스
- **Kapacitor**: Real-time streaming 데이터 전송 엔진
- **Chronograf**: 시각화 도구

InfluxDB는 Continuous Query(Task), Retention Policy(Retention Period)라는 핵심 기능을 제공하는데 

일정 주기마다 데이터를 처리하여 새롭게 저장하는기능, 일정 주기마다 데이터를 자동 삭제하는 기능으로 설명할 수 있다.

#### Continous Query, Task

influx db의 목적은 시간에 따른 데이터. 시계열 데이터의 처리다.

influx db는 데이터를 처리하여 새롭게 저장하는 down sampling을 일정 주기마다 실행되도록 하는 continous query를 제공하고있다.

influx db2에서는 continous query를 대체하는 task를 제공하고 있다.

#### Retention Policy, Period

보존정책이라고 하는데, influx db의 핵심 목적은 시간에 따른 데이터 삽입과 조회이므로 delete를 이용하는 경우가 거의 없다.

하지만 데이터가 계속해서 쌓이면 저장 공간 및 처리 속도등에 문제가 생기므로 데이터를 자동으로 삭제해주는 retention policy를 지원하고 있다.

Retention Policy는 의미 그대로 오래된 데이터를 자동으로 삭제해주는 정책인데 db단위로 정의되며 일반적으로 1개 데이터베이스는 여러개의 보존 정책을 가지고 있다.

만약 별도 설정을 안하면 autogen이라는 기본정책이 적용되고 autogen은 보존 기간이 무제한이므로 벼롣의 설정을 해주지 않으면 데이터가 계속해서 쌓이고 문제가 발생한다.

그러므로 별도의 설정을 해줌으로써 오래된 데이터들을 관리하는 작업이 필요하다.

influxdb2에서는 retention policy를 대체하는 period를 제공하고 있다.

## InfluxDB 내부 구조 및 구성

influxdb 구성의 차이를 RDB와 비교하면 아래와 같다.

| InfluxDB | RDB(Relational Database) |
| :--- | :--- |
| **Database** | Database |
| **Measurement** | Table |
| **Tag Key** | Indexed Column (String Only) |
| **Field Key** | Unindexed Column |
| **Column** | Column |
| **Point** | Row |

### Measurement

시계열 데이터베이스에서 측정의 의미를 가지는 measurement다.

관계형 데이터베이스의 table같은 역할을 하며 rdb와 마찬가지로 데이터베이스 안에 여러개의 measurement가 존재할 수 있다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdna%2FPjvOf%2FbtreQ9J4vcR%2FAAAAAAAAAAAAAAAAAAAAANl3h1nFHaEKhrgK4Z_No7HujO9eV5gVakJN8kECy1WG%2Fimg.png%3Fcredential%3DyqXZFxpELC7KVnFOS48ylbz2pIh7yKj8%26expires%3D1774969199%26allow_ip%3D%26allow_referer%3D%26signature%3D5kOo3kLp8iyV1kyTlUxwF9xNQYM%253D)

rdb와 차별화된 핵심 컨셉이 존재하는데 influxDB는 NoSQL의 개념을 바탕으로 만들어져서 schemaless하다는 특징이 있다.

기존의 rdb로 개발할때는 필요한 컬럼들과 길이 및 타입등을 설계하면서 테이블 스키마를 구성해주었어야했지만 influx db는 schemaless라 새로운 데이터를 추가하는 시점에

measurement와 관련된 컬럼들이 추가되어 스키마 변경이 매우 빠르다. influxdb는 이러한 구조를 가져감으로서 시계열 데이터에 유연하게 대처할 수 있도록 하였다.

### Key(tag key, filed key, time key)

컬럼의 구성은 기존 rdb와 차이가 존재하는데 기존 rdb는 1개의 컬럼에 1개의 데이터가 저장되어있지만.

TSDB에서는 컬럼이 3가지 종류로 나뉘며 각각의 컬럼이 key value로 구성된다

- **Tag Key**
  - rdb에서 indexed 컬럼과 유사하며, 인덱싱이 되어 select 문으로 조회하는 기준이 된다.
  - tag의 value로는 string 타입만 가능하고 질의시 따옴표 ''로 감싸주어야한다.
- **Field Key**
  - RDB에서 indexed 되어있지 않은 컬럼과 유사며, 저장되는 데이터는 최소 1개 이상의 필드가 있어야한다.
  - Field Value에서는 strings, float, integer or boolean 타입이 가능하며 타입이 정해지면 변경이 불가하다.
- **Time Key**
  - UTC를 기준으로 1970년 1월 1일 0시 0분 0초로부터 지난시간을 microseconds 단위로 자동 입력된다.
  - 수동으로 설정할수있긴한데 권장은 x

데이터를 저장하기 위해서는 태그 또는 필드 중 하나를 선택해야하는데, 이것은 매우 중요하다.

왜냐하면 태그에 있는 값들은 인덱싱 되는 반면에 필드는 인덱싱되지 않기 때문이다. 그러므로 조회를 하기 위해 인덱싱이 되어야하는 경우 태그를

그렇지 않고 단순 데이터인 경우에는 필드로 선택하면 된다.

만약 불필요한 데이터까지 모드 태그로 잡는다면 인덱스 저장소 index structure가 비대해져서 메모리를 많이쓰고 처리속도가 저하될것이다.

### Series

seris는 influx db에만 존재하는 개념이고 조합 가능한 tag key의 집합에 해당한다.

에를 들어 member라는 measurement에 (이름, 나이)가 tag key로 지정되어 있다면 저장된 모든 데이터들중 가능한 (이름, 나이)의 집합이 series이다.

### Shard, ShardGroup

A Shard Group은 influx db bucket에 속하며 shard group duration(샤드 그룹 보존 기간)에 따라서 실제 데이터를 저장하는 샤드를 관리한다.

샤드는 샤드 그룹에 정의된 기간에 속한 데이터들이 인코딩 및 압축되어 저장된다.

특정 샤드 그룹 duration에 저장된 모든 포인트(데이터)들은 동일한 샤드에 저장된다.

1개의 샤드는 여러개의 series, disk에 time-structured merge tree(TSM)으로 구성된다.

TSM은 내부적으로 사용되는 트리 기반의 데이터 저장소라고 생각하면 된다. 데이터가 TSM에 저장되기 이전에 메모리에 있는 것이고.

shard group duration(샤드 그룹 보존 기간)이 만료되면 디스크에 TSM 형태로 저장되는 것이다.

InfluxDB는 shard group과 shard를 이용해 데이터를 샤딩하여 데이터가 시간이 지남에 따라 증가하여도 처리량과 전체 성능을 높이는 접근 방법을 취한다.

샤드는 데이터에 대한 임시 블록을 가지고 있어 TSM에 매핑된다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdna%2FpmQn2%2Fbtre093xq2w%2FAAAAAAAAAAAAAAAAAAAAACJUPEWjsyy0ufNyM1Bn-W4m0PHV6wav7qUjR3JA-F5Z%2Fimg.png%3Fcredential%3DyqXZFxpELC7KVnFOS48ylbz2pIh7yKj8%26expires%3D1774969199%26allow_ip%3D%26allow_referer%3D%26signature%3Dc%252B5fGZbllhee5VQTpfvwlNGZMsI%253D)

시간을 기반으로 분리되는 shard를 이용하고 시간을 제약 조건으로 사용해 메모리에 로드할 데이터 양 자체를 줄일 수 있다.

이는 tsdb들이 기본적으로 가져가는 개념이다. https://docs.influxdata.com/influxdb/v2/reference/internals/shards/#shard-group-diagram

샤드 그룹은 시간대별로 나눈 논리적 컨테이너인거고 influxdb는 데이터를 보관주기에 따라 자동으로 버려야하는데

이때 시간단위로 묶여있으면 관리가 편할것이니 retention policy에 따라 잘 버리기위해서 정의된다.

1:N 관계고 그룹하고 shard는 ㅇㅇ, 1대1이 아닌 이유는 샤드 그룹 내부에 여러개의 샤드를 만들기위해서 뭐 복제본이라던가 분산을 위해서 열어두고있다.

정리하면 shardgroup은 시간범위 컨테이너 shard는 물리적 파일인거다. tsm이라는 형태로 저장되는.

#### Shard Group Duration

shard group duration이라는 샤드 그룹 보존 기간은 샤드 그룹이 메모리에 보존되고 새롭게 생성되는 기간을 의미한다.

만약 해당 보존 기간이 지나면 샤드 그룹은 제거된다. 기본적으로 influx db는 버킷의 retention period에 따라 샤드 그룹 보존 기간을 설정한다.

별도 설정을 하지 않았다면 샤드 그룹 보존 기간은 기본적으로 7일이다.

- Retention Priod가 less than 2days라면 shard gropu duration은 1시간
- " 가 between 2 days and 6 months 라면 1일
- " 가 greater than 6 months 라면 7일

shard group duration, retention period가 약간 헷갈릴 수 있는데

예를들어 보존 기간은 무한하면 샤드 그룹 보존기간은 7일인데, 샤드 그룹 보존기간이 만료되면 데이터가 삭제되는것인지 아니면 남아있는것인지 에 대한 의문을 가질수잇다.

샤드 그룹 보존 기간은 데이터가 메모리에 상주해있는 시간이고 만약 샤드 그룹 보존 기간이 만료되면 데이터는 메모리에서 디스크로 저장되는것이다.

캐시라고 생각하면 편함. 그러므로 보존 기간이 무한하면 해당 데이터는 영구적으로 저장이 되지만 샤드 그룹 보존 기간이 끝났다면 해당 데이터를 조회하기 위해서 디스크에서부터 값을 읽어와햐는것

즉 샤드 리텐션은 memory ttl같은 느낌이고 retention period는 진짜 data의 수명인것