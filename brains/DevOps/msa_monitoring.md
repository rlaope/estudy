# MicroService Monitoring System (cc. Loki/Grafana)

요구사항을 먼저 정의하고 들어가보자.

원하는 엔드포인트(대시보드) -> 서비스 선택 -> 해당 서비스 로그만 필터링을 달성할 것이다.

예시로 Payment, Product, User 서비스가 존재할때 하나의 그라파나 대시보드에서 Payment를 선택하면 Payment의 로그만 출력

k8s, promtail daemon set이 컨테이너 로그를 수집하고 Loki에서 저장/인덱싱을 진행후에 그라파나에 띄워줄 것이다.

라벨 전략으로 job, namespace, app, pod, container, env등을 부여할거고 드롭다운에 바인딩하자

### Loki + Promtail Setting

```sh
# 네임스페이스
kubectl create ns observability

# Loki (모노리스) + Promtail
helm repo add grafana https://grafana.github.io/helm-charts
helm repo update

helm upgrade --install loki grafana/loki \
  --namespace observability

helm upgrade --install promtail grafana/promtail \
  --namespace observability \
  --set "config.clients[0].url=http://loki.observability.svc.cluster.local:3100/loki/api/v1/push"

```

### Promtail 라벨링

Pod 로그 감시후 스트림 전송을 한다. 로그에 app=<서비스명> 같은 라벨을 부여하자.

json + 라벨이 우리가 이 로그시스템을 구축하기 위해 필요한 요소들이다.

loki에서 인덱싱하기 위해, 또 분류하기위해 필요한 로그 정보들을 json으로 저장해두고 우리는 스프링의 로그를 해당 포맷으로 바꿔주는 시스템이 필요하다.

```yaml
relabel_configs:
  - source_labels: [__meta_kubernetes_pod_label_app]   # Kubernetes Deployment label: app=svc-a
    target_label: app
  - source_labels: [__meta_kubernetes_namespace]
    target_label: namespace
```

이런식으로 두고 loki grafana에서는 아래와 같은 형식으로 로그를 볼거다 json

```yaml
{
  "timestamp": "2025-10-13 14:22:15.341",
  "level": "INFO",
  "logger": "com.khope.service.XYZHandler",
  "message": "User logged in successfully",
  "service": "svc-a",
  "traceId": "a91f2d3b7e994b2",
  "namespace": "production",
  "app": "svc-a",
  "pod": "svc-a-7f8c5c7d6d-abcde"
}
```

```log
▶ app=svc-a pod=svc-a-7f8c5c7d6d-abcde namespace=production

2025-10-13T14:22:15.341Z  INFO [svc-a,,,a91f2d3b7e994b2] User logged in successfully
```

> Spring Boot 로그를 JSON + service 필드가 포함된 형태로 출력하고, Promtail이 app 라벨을 붙여 Loki에 저장하면, Grafana에서 app 값을 드롭다운 필터로 사용할 수 있다. → 결과적으로 “단일 대시보드 + 서비스 선택” UX 완성.

<br>

## Payment Product User Service Monitoring Example

```css
obs-demo/
├─ docker-compose.yml
├─ loki/
│  └─ config.yml
├─ promtail/
│  └─ config.yml
├─ grafana/
│  └─ provisioning/
│     ├─ datasources/datasource.yml
│     └─ dashboards/logs-dashboard.json
├─ services/
   ├─ product/
   │  ├─ build.gradle.kts
   │  ├─ src/main/resources/application.yml
   │  ├─ src/main/resources/logback-spring.xml
   │  └─ src/main/kotlin/com/example/product/ProductApp.kt
   ├─ payment/
   │  └─ ... (product와 동일 구조, 이름만 변경)
   └─ user/
      └─ ... (product와 동일 구조, 이름만 변경)
```

위와 같은 아키텍처로 서비스들을 띄우고 로키 그라파나 프롬테일을 띄워서 실습해볼거다.


```yaml
# docker compose
version: "3.9"

services:
  loki:
    image: grafana/loki:2.9.4
    command: -config.file=/etc/loki/config.yml
    volumes:
      - ./loki/config.yml:/etc/loki/config.yml:ro
    ports:
      - "3100:3100"

  promtail:
    image: grafana/promtail:2.9.4
    command: -config.file=/etc/promtail/config.yml
    volumes:
      - ./promtail/config.yml:/etc/promtail/config.yml:ro
      - /var/lib/docker/containers:/var/lib/docker/containers:ro
      - /var/run/docker.sock:/var/run/docker.sock
    depends_on: [loki]

  grafana:
    image: grafana/grafana:11.1.0
    environment:
      - GF_SECURITY_ADMIN_PASSWORD=admin
    volumes:
      - ./grafana/provisioning:/etc/grafana/provisioning:ro
    ports:
      - "3000:3000"
    depends_on: [loki]

  product:
    build: ./services/product
    image: demo/product:latest
    environment:
      - APP_NAME=product
      - JAVA_TOOL_OPTIONS=-Duser.timezone=Asia/Seoul
    labels:
      # 컨테이너 라벨을 promtail에서 라벨로 변환
      - 'promtail.app=product'
    depends_on: [loki, promtail]

  payment:
    build: ./services/payment
    image: demo/payment:latest
    environment:
      - APP_NAME=payment
      - JAVA_TOOL_OPTIONS=-Duser.timezone=Asia/Seoul
    labels:
      - 'promtail.app=payment'
    depends_on: [loki, promtail]

  user:
    build: ./services/user
    image: demo/user:latest
    environment:
      - APP_NAME=user
      - JAVA_TOOL_OPTIONS=-Duser.timezone=Asia/Seoul
    labels:
      - 'promtail.app=user'
    depends_on: [loki, promtail]
```

**로키 설정 loki/config.yaml**
```yaml
auth_enabled: false
server:
  http_listen_port: 3100
common:
  ring:
    instance_addr: 127.0.0.1
  path_prefix: /loki
schema_config:
  configs:
    - from: 2024-01-01
      store: boltdb-shipper
      object_store: filesystem
      schema: v13
      index:
        prefix: index_
        period: 24h
storage_config:
  boltdb_shipper:
    active_index_directory: /loki/index
    shared_store: filesystem
  filesystem:
    directory: /loki/chunks
compactor:
  working_directory: /loki/compactor
  shared_store: filesystem
limits_config:
  retention_period: 168h  # 7일 예시
```

**Promtail 설정 (promtail/config.yaml)**

```yaml
server:
  http_listen_port: 9080
  grpc_listen_port: 0

positions:
  filename: /tmp/positions.yaml

clients:
  - url: http://loki:3100/loki/api/v1/push

scrape_configs:
  - job_name: docker
    static_configs:
      - targets: [localhost]
        labels:
          __path__: /var/lib/docker/containers/*/*-json.log
    pipeline_stages:
      - docker: {}
    relabel_configs:
      # 컨테이너 라벨 promtail.app -> app
      - source_labels: ['__meta_docker_container_label_promtail_app']
        target_label: app
      # 컨테이너 이름, 이미지도 라벨로 보존(선택)
      - source_labels: ['__meta_docker_container_name']
        target_label: container
      - source_labels: ['__meta_docker_container_image']
        target_label: image
```

> Docker 환경에서 promtail이 컨테이너 라벨을 읽어오려면 위와 같은 meta 라벨을 relabel해야 한다. Kubernetes라면 __meta_kubernetes_pod_label_app로 동일하게 처리합니다.

그라파나를 자동으로 프로비저닝 한다.
```yml
# grafana/provisioning/datasources/datasource.yml
apiVersion: 1
datasources:
  - name: Loki
    type: loki
    access: proxy
    url: http://loki:3100
    isDefault: true
```
대시보드 (grafana/provisioning/dashboards/logs-dashboard.json)

- 변수 service = label_values(app)
- 로그 패널 쿼리: {app="${service}"} | json
아래 최소 동작 JSON(축약). 필요시 붙여서 사용해보자

```json
{
  "overwrite": true,
  "dashboard": {
    "title": "Microservices Logs",
    "templating": {
      "list": [
        {
          "name": "service",
          "type": "query",
          "datasource": "Loki",
          "query": "label_values(app)",
          "refresh": 1,
          "includeAll": false,
          "multi": true
        }
      ]
    },
    "panels": [
      {
        "type": "logs",
        "title": "Service Logs",
        "targets": [
          { "datasource": "Loki", "expr": "{app=~\"${service}\"} | json" }
        ],
        "gridPos": { "x": 0, "y": 0, "w": 24, "h": 18 }
      }
    ],
    "schemaVersion": 38
  }
}
```

**각 spring boot application 공통 설정**

```kt
plugins {
    id("org.springframework.boot") version "3.3.4"
    id("io.spring.dependency-management") version "1.1.6"
    kotlin("jvm") version "1.9.24"
    kotlin("plugin.spring") version "1.9.24"
}
group = "com.example"
version = "0.0.1"
java.sourceCompatibility = JavaVersion.VERSION_17

repositories { mavenCentral() }

dependencies {
    implementation("org.springframework.boot:spring-boot-starter-web")
    // JSON 로깅용(택1) - logstash encoder
    implementation("net.logstash.logback:logstash-logback-encoder:7.4")
    testImplementation("org.springframework.boot:spring-boot-starter-test")
}

tasks.withType<Test> { useJUnitPlatform() }
kotlin { jvmToolchain(17) }
```

```yaml
# application.yaml
server:
  port: ${PORT:8080}

spring:
  application:
    name: ${APP_NAME:unknown}

logging:
  level:
    root: INFO
```

logback-spring.xml
```xml
<configuration>
  <appender name="JSON" class="net.logstash.logback.appender.LogstashConsoleAppender">
    <encoder class="net.logstash.logback.encoder.LoggingEventCompositeJsonEncoder">
      <providers>
        <timestamp>
          <timeZone>Asia/Seoul</timeZone>
        </timestamp>
        <logLevel/>
        <loggerName/>
        <pattern>
          <pattern>
            {
              "service":"${APP_NAME:-unknown}",
              "thread":"%thread",
              "traceId":"%X{traceId:-}",
              "spanId":"%X{spanId:-}"
            }
          </pattern>
        </pattern>
        <message/>
        <stackTrace/>
      </providers>
    </encoder>
  </appender>

  <root level="INFO">
    <appender-ref ref="JSON"/>
  </root>
</configuration>
```

왜 이런 구성이 필요하는지 다시 리마인드하자면
- json log: 필드 기반 검색/파싱, 메시지/필드 조합으로 대시보드 한 곳에서 필터링할 수 있기 때문이다.
- 라벨(app=product ...): 대시보드 한 곳에서 서비스별 빠른 필터를 위해서다.
- 확장성: 서비스 추가시 promtail.app=newsvcname만 붙이면 즉시 반영된다/
- traceId, spanId유지: Tempo 연동으로 로그 트레이스 연결이 가능하다.

실제 로그가 아래와 같이 보인다.

```json
# Product
{
  "timestamp": "2025-10-13T14:22:15.341+09:00",
  "level": "INFO",
  "logger": "com.example.product.ProductController",
  "message": "Product queried",
  "service": "product",
  "thread": "http-nio-8080-exec-1",
  "traceId": "a91f2d3b7e994b2",
  "spanId": "c1f2ab03e1dcd9"
}

# Payment
{
  "timestamp": "2025-10-13T14:22:17.102+09:00",
  "level": "INFO",
  "logger": "com.example.payment.PaymentController",
  "message": "Payment authorized",
  "service": "payment",
  "thread": "http-nio-8080-exec-2",
  "traceId": "bf12a0f2aa1c4e12",
  "spanId": "0ab9dd11223344"
}

# User
{
  "timestamp": "2025-10-13T14:22:19.887+09:00",
  "level": "ERROR",
  "logger": "com.example.user.UserController",
  "message": "User not found: id=42",
  "service": "user",
  "thread": "http-nio-8080-exec-3",
  "traceId": "9912cc0e7c8d4db",
  "spanId": "55ddaa77889900",
  "stack_trace": "..."  // 예외 시 자동 포함
}
```

> Tempo는 그라파나 분산 트레이싱 백엔드로 트레이스와 스팬 흐름 정보를 저장함. otlp jaeger zipkin 프로토콜로
>
> 그리고 도커에서 relabel이 필요하다고 나왔었는데 위에서 로그를 수집할때 Loki 라벨로 변환하는 과정으로 라벨같은 정형 메타데이터를 달아달라는 뜻임. 도커에는 컨테이너 라벨을 달아두고 Promtail이 `__meta_docker_container_label_<KEY>` -> `app`으로 매핑함.