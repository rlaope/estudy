# MicroService Monitoring System (cc. Loki/Grafana)

Let's start by defining the requirements.

We aim to achieve the following: desired endpoint (dashboard) -> service selection -> filter only logs for that service.

For example, if Payment, Product, and User services exist, selecting Payment on a single Grafana dashboard will display only Payment's logs.

A k8s, promtail daemon set will collect container logs, Loki will store/index them, and then display them in Grafana.

We will assign labels such as job, namespace, app, pod, container, and env as part of our labeling strategy and bind them to a dropdown.

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

### Promtail Labeling

After monitoring Pod logs, streams are sent. Let's assign labels like `app=<service_name>` to the logs.

JSON + labels are the essential elements we need to build this logging system.

We need to store the log information required for indexing and classification in Loki as JSON, and we need a system to convert Spring logs into this format.

```yaml
relabel_configs:
  - source_labels: [__meta_kubernetes_pod_label_app]   # Kubernetes Deployment label: app=svc-a
    target_label: app
  - source_labels: [__meta_kubernetes_namespace]
    target_label: namespace
```

With this setup, Loki Grafana will display logs in the following JSON format:

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

> Outputting Spring Boot logs in JSON format including a `service` field, and having Promtail attach an `app` label before storing them in Loki, allows Grafana to use the `app` value as a dropdown filter. → This completes the "single dashboard + service selection" UX.

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

We will set up services with the above architecture and deploy Loki, Grafana, and Promtail for a practical exercise.

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

**Loki Configuration loki/config.yaml**
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

**Promtail Configuration (promtail/config.yaml)**

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

> In a Docker environment, for Promtail to read container labels, meta labels like the above must be relabeled. For Kubernetes, `__meta_kubernetes_pod_label_app` is handled similarly.

Grafana is automatically provisioned.
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
Dashboard (grafana/provisioning/dashboards/logs-dashboard.json)

- Variable `service = label_values(app)`
- Log panel query: `{app="${service}"} | json`
Minimal working JSON below (abbreviated). Use it if needed.

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

**Common Configuration for Each Spring Boot Application**

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

To reiterate why this configuration is necessary:
- JSON logs: Enable field-based searching/parsing and filtering on a single dashboard using a combination of messages and fields.
- Labels (`app=product` ...): For quick service-specific filtering on a single dashboard.
- Scalability: Adding `promtail.app=newsvcname` for a new service is immediately reflected.
- `traceId`, `spanId` preservation: Allows connecting log traces with Tempo integration.

Actual logs appear as follows:

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

> Tempo is Grafana's distributed tracing backend, storing trace and span flow information using OTLP, Jaeger, and Zipkin protocols.
>
> It was mentioned that relabeling is needed in Docker; this refers to attaching structured metadata like labels during log collection by converting them into Loki labels. In Docker, container labels are set, and Promtail maps `__meta_docker_container_label_<KEY>` to `app`.
