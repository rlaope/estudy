# Probe

쿠버네티스는 어떻게 컨테이너들의 상태를 분석하고 처리할 수 있는 것일까?

바로 kubelet의 Probe가 이 역할을 해준다.

Probe는 파드 안에 컨테이너의 상태를 진단하고 분석하여 재시작, 종료가 필요한지에 여부를 확인할 수 있게 도와준다.

<br>

### Probe 동작 방식

Probe는 총 세 가지의 방식으로 컨테이너의 상태를 진단한다.

1. HTTP 방식: HTTP GET요청을 보내서 2xx, 3xx 응답이 오는지 체크한다.
2. TCP 방식: 3-way-handshake가 잘 맺어져 있는지 확인한다.
3. Exec 방식: 컨테이너 특정 명령어를 실행해서 종료코드가 0인지 확인한다.

이러한 진단 결과는 크게 `Success`(성공), `Failure`(실패), `Unknown`(진단 실패)의 세 가지로 구분된다. 만약 진단 결과가 `Failure`로 나온다면, **kubelet**이 이를 감지하여 필요한 작업을 수행한다.

컨테이너 상태 진단을 위한 프로브는 크게 **Liveness**, **Readiness**, **Startup**의 세 종류로 구분된다. 이들의 용도와 동작 방식을 요약하면 다음과 같다.

![](https://seongjin.me/content/images/2022/02/probes-1.png)


### Liveness 
위의 세 가지 방식(HTTP, TCP, Exec)으로 컨테이너의 상태를 확인하고 Failure가 나온다면 컨테이너를 재시작한다.

이 프로브를 통해 컨테이너 내에 일부 문제가 있더라도 높은 가용성을 보장받을 수 있다.

그러나 내부 병목 현상이나 메모리 누수와 같은 문제가 생기면 프로세스가 아예 멈출 수도 있어 주의해야한다.

아래 예시는 컨테이너에 직접 명령을 실행해서 상태를 진단하는 **Exec 방식**의 Liveness 프로브를 설정하는 경우다. 이 내용은 파드 배포에 쓰이는 YAML 파일 중 `spec.containers` 아래의 컨테이너 명세에 추가된다.

```yaml
livenessProbe:
  exec:
    command:
    - cat
    - /tmp/healthy
  initialDelaySeconds: 5
  periodSeconds: 5
```

- `initialDelaySeconds`를 통해서 지정한 시간동안 대기한다. 그 이후 kubelet이 `periodSeconds`의 주기로 cat을 /tmp/healthy로 실행한다.
- 만약 위와 같은 형식으로 파드(컨테이너) 생성때 이 프로브가 별도로 선언되지 않았다면, `Success`가 기본 상태인 것으로 간주된다.
- 만약 컨테이너가 완전히 RUNNING 상태에서 실행시키고 싶다면 initialDelaySeconds값을 충분히 주거나 Probe의 Startup Probe를 함께 사용하는 방법이 있다.

<br>

### Readiness

Readiness방식은 Liveness 방식과 유사하며 다른 점은, Readiness 방식에서 Failure가 떴을 경우에 해당 파드로 **로드밸런싱을 하지 않는다는 점이다.**

외부에서 들어오는 요청을 적절히 노드에 분배하는 역할은 클러스터에 서비스가 하는 역할이다. 만약, 특정 파드에 대해서 응답이 있지 않다면? 해당 파드는 문제가 있다고 판단되므로 요청을 보내지 않는다.

선언하는 방식은 Liveness와 유사하며 `readinessProbe` 아래에 작성한다는 점만 다르다.

```yaml
readinessProbe:
  exec:
    command:
    - cat
    - /tmp/healthy
  initialDelaySeconds: 5
  periodSeconds: 5
```

- 만약 위와 같은 형식으로 파드(컨테이너) 생성때 이 프로브가 별도로 선언되지 않았다면, `Success`가 기본 상태인 것으로 간주된다.
- Liveness 프로브는 Readiness 프로브와 별개로 동작한다. 어느 하나가 `Success` 될 때까지 대기하지 않는다. 따라서 둘을 함께 쓸 경우에는 주의해서 설정해야 한다.

<br>

### Startup Probe

컨테이너에 탑재된 애플리케이션의 시작 시기를 확인하는 프로브다. Success가 되기 전까지 Liveness, Readiness 프로브들이 진행되지 않게 막고, 혼자 파드의 시작을 확인한다.

앞서 살펴본 프로브들(Liveness, Readiness)는 모두 파드의 컴퓨팅 자원을 사용하여 동작한다. 때문에 파드가 처음 시작할 때부터 이들이 많은 활동을 보이게 되면, 컨테이너와 파드가 `Running` 상태로 도달하는 시간이 그만큼 길어지게 된다. 따라서 파드가 정상적으로 구동되기 시작했다고 판단될 때까지는, 이 프로브가 다른 프로브 기능을 비활성화 시킨 상태에서 대신 상태 진단을 수행한다.

이 프로브는 특성상 파드나 컨테이너 상태가 `Failure`로 판명되어도 일단 넘어가게 되어있다. 실제로 YAML 파일을 통해 이 프로브를 선언할 때, 아래와 같이 **실패 허용 횟수**를 지정할 수 있다.

```yaml
startupProbe:
  httpGet:
    path: /health-check
    port: liveness-port
  failureThreshold: 30
  periodSeconds: 10
```

위의 방식은 http 방식으로 상태를 검사한다.

- `failureThreshold`를 통해서 실패 허용 횟수를 지정했다. `periodSeconds`마다 총 failureThreshold(30)번 만큼 상태를 검사한다.
- `startupProbe.failureThreshold` : 최대 실패 허용 횟수를 의미한다.
- `startupProbe.periodSeconds` : 프로브가 상태 진단하는 주기(초)를 의미한다.

만약 컨테이너와 파드의 정상 구동이 완전히 확인되었다면, Startup Probe는 상태 체크 Loop를 멈추고 프로브 기능을 Liveness와 Readiness에게 넘긴다.