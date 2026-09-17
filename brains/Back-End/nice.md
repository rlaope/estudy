# 외부 연동 시스템 리소스 제약과 쏟아지는 트래픽 대응

아래와 같은 구조의 시스템을 설계했다.

```
외부 플랫폼 - 사내 대출 사전조회 서버 - 큐 - 사내 대출 사전조회 컨슈머 - (외부 기관 통신)
```

해당 구조에서 외부 플랫폼에서 우리 사내 대출 사전조회 서비스로 요청을 보내면 바로 응답을 준 뒤 큐에 

해당 고객 정보 키등을 포함하여 컨슈머서버에서 금리, 대출 한도등을 산출해낸다. 그리고 다시 응답을 보내면

외부 플랫폼에서 우리 시스템에서 산출한 한도들을 보여주는 방식이다.

여기서는 외부 기관 통신이 같이 포함되는데 예를들면 nice/kcb (신용 점수 평가 기관) 및 우리 내부 금리 산출 ai 모델 + 우리 코어뱅킹 시스템 (대출 신청서 데이터 작성)의 내부 로직도 돈다.

자 여기서 트래픽이 많이 밀리게 된다면, 여러가지 서비스에서 병목이 있을 수 있는데

1번 사내 대출 사전조회 api 서버가 있다. 이는 비동기적으로 응답을 처리하므로 빠른 응답을 통해 감당할 수 있는 수준이었다.

구체적인 수치는 약 2시간동안 50만 트래픽, 약 60~70TPS이며, 피크땐 200TPS까지 튄다. 그렇게 얼마 안될거같지만, 콜 하나하나가 무겁기때문에 이정도 트래픽도 굉장히 부담스럽다.

일단 사내 대출 사전조회 서버에서는 비동기적으로 처리하고 응답을 플랫폼에게 주기때문에 spring boot 서버 파드 두대로도 충분히 해당 트래픽을 감당할 수 있었다.

두번째로 큐가 병목일수도 있다. 일단 많은 트래픽을 받아 큐에 넣게된다면 큐에 많은 메시지들이 쌓이게 될것이다.

사내 대출 사전조회 컨슈머는 평균적으로 1~2s안에 해당 데이터들을 처리한다. 그냥 평범한 스레드 200개 들고있는 spring boot 서비스라면 무난하게 처리하겠지만 여기서 문제가 발생한다.

### 제약

바로 nice 시스템 때문인데 나이스 시스템에서 전용 세션을 40개까지밖에 지원을 하지 않는 구조였다.

nice 시스템에서는 약 0.5ms 정도의 시간으로 응답을 주기때문에 80TPS까지는 어찌저찌 처리한다고 보자. 40 x 2로 그렇지만, 피크타임에 트래픽이 튀게된다면? 응답시간은 1s 2s 점점 늘어나게되며, 사전조회 플랫폼의 처리량이 낮아지고 더 많은 요청들이 나이스 시스템에게도 부하가 가서 대역폭도 잡아먹고 1mbps라서 대역폭도 적어 밀렸었음.

아무쪼록 이 시스템이 문제가 된다.

그럼 어떻게 해결해볼 수 있을까? 가장 먼저 떠오르는 방법은 나이스 시스템에 백프레셔, 서킷을 걸어두는 방법이 있겠다.

일단 대충 빈값으로 응답을 받고 나중에 처리하면 되지않을까? 그럼 해당 80개 세션에 대한 제약도 일단은 없어지고, 나중에 DLT에서 꺼내서 재시도 시킨후에 하자? 음... 구조상으로는 맞는거같다. 그렇지만 안된다.

여기선 도메인 제약이 걸려있기 때문인데 나이스에서는 해당 고객의 신용점수를 준다. 

그렇기때문에 이 신용점수에 따라 금리, 대출한도를 산출하기 때문에 무조건 나이스 콜은 원자적으로 같이 이루어져야한다.

그럼 여기서는 답이 없다 **물리적으로 불가능하다.** 수많은 트래픽을 처리하기에 물리적으로 불가능하다.

나이스 시스템과 결합되어 있으니까. 그리고 추가적으로 나이스는 한도 산출을 위한 신용점수 뿐만아닌

우리시스템에서 찍어낸 대출 상태를 동기화하기위한 대출 등록, 상환/연체/채권 상태 변동에 관한 변동콜등

대출 사전조회보다 더욱 중요하고 강력한 일관성이 수반되는 해당 기능들도 나이스에 묶여있기때문에 나이스의 장애는 더더욱 치명적이다.

### 선택

여기서 어떻게 대처할수있을까? 일단 세션수 증설과 대역폭 증설이 가장 쉽겠다 돈바르면 뭐든지 된다하지 않았던가.

새로운 서비스를 오픈하면서 기존에 덜사용됐던 나이스의 전용회선 대역폭과 세션수가 더 필요하게 되었으니 당연히 늘려야한다.

그러나 80개까지로밖에 못늘렸다 대역폭도 4Mbps로 올렸는데 새벽에 도는 배치 빼고는 사전조회 트래픽으로 대역폭을 막 그렇게 크게 잡아먹진 않았다. 많이 잡아먹어도 2Mbps정도?

일단 120개까지로 올려두면 좋을거같긴한데, 어쨋든 80개 자체로 해당 트래픽들을 감당해야한다고 해보자.

일단 사전 대출 처리 컨슈머 서버부터 확인해보자. 해당 서버가 큐를 처리하지만 나이스에게 바로 요청을 찌르게 된다면, 80개의 세션만 존재하는 나이스 서버에게 큰 부하를 주게 될 것이고. 사전조회 서비스외에 다른 서비스들이 나이스를 찌르기도 하므로 장애가 확산될 수 있다.

그러므로 유량제어가 필요한데, nice-proxy 서버를 여러대 두어 해당 나이스 시스템에 대해서 트래픽 처리를 진행하도록 했다.

일단 nice와의 통신은 tcp 소켓 전문 통신이다. 우리서비스가 나이스 프록시에게 해당 신용점수 call을 send하게 되면 프록시가 먼저 받아 나이스에게 다시 send이후 receive한 데이터를 프록시 버퍼에 write한다. 그 이후 처음 send한 서비스에게 다시 응답을 준다. receive한 서비스 서버는 해당 데이터를 기반으로 뭔가를 막할것이다.

자 그럼 proxy를 둔 이유 1. 트래픽 제어다. 여튼 쨋든 나이스 세션은 80개다. 80개이상의 요청이 들어오는것은 당연하다.

내부 서비스들은 수천개넘게 해당 요청을 찌를수있다. 그러나 세션이 없으므로 해당 요청은 실패하게 된다. 

그래서 nice-proxy는 네티로 두게 되었다. 일단 그 이유는 네티로 프록시 서버를 두게된다면 일단 오래 지연되더라도 적은 쓰레드의 수로 수천개의 인입 연결을 유지하면서 실제로 nice로 나가는 파이프라인만 효율적으로 관리할 수 있다.

바로 실패가 아니라 늦더라도 응답을 할 수 있기 때문에, 그리고 ChannelHandler 파이프라인을 통해 ByteBuf를 직접 핸들링하여 프레임 디코딩, 전문해석, 타임아웃 처리를 매우 정교하게 할 수 있다.

그리고 Netty에서는 미리 요청이 몰릴게 예상되면 커넥션을 미리 연결해놓고 3wayhandshake 비용을 줄이고 80개의 통로를 효율적으로 순환시키는것이 가능하다.

```kt
class NiceChannelInitializer : ChannelInitializer<SocketChannel>() {
    override fun initChannel(ch: SocketChannel) {
        val pipeline = ch.pipeline()

        // 1. 타임아웃 설정: 10초 동안 응답(read)이 없으면 ReadTimeoutException 발생
        pipeline.addLast("timeoutHandler", ReadTimeoutHandler(10))

        // 2. 전문 디코딩: 금융 전문이 고정 길이(예: 200byte)라면 FixedLengthFrameDecoder 사용
        // 만약 가변 길이라면 LengthFieldBasedFrameDecoder를 사용합니다.
        pipeline.addLast("decoder", FixedLengthFrameDecoder(200))

        // 3. 비즈니스 로직 핸들러: 응답 전문을 파싱하여 결과를 큐나 콜백으로 전달
        pipeline.addLast("clientHandler", NiceClientHandler())
    }
}
```

세션 미리 맺기

```kt
class NiceConnectionPool(
    private val host: String,
    private val port: Int,
    private val poolSize: Int = 80
) {
    // 가용한 채널을 담아두는 큐 (Backpressure 역할 병행)
    private val pool = ArrayBlockingQueue<Channel>(poolSize)
    private val bootstrap = Bootstrap()

    init {
        val group = NioEventLoopGroup(4) // 적은 수의 쓰레드로 80개 관리
        bootstrap.group(group)
            .channel(NioSocketChannel::class.java)
            .option(ChannelOption.SO_KEEPALIVE, true)
            .option(ChannelOption.CONNECT_TIMEOUT_MILLIS, 3000)
            .handler(NiceChannelInitializer())
    }

    // 서버 시작 시 호출하여 80개 세션을 미리 맺음
    fun prewarm() {
        val futures = (1..poolSize).map {
            bootstrap.connect(host, port).addListener { future: ChannelFuture ->
                if (future.isSuccess) {
                    pool.offer(future.channel())
                    println("Session connected: ${future.channel().id()}")
                } else {
                    // SRE 관점: 연결 실패 시 재시도 로직이나 알림 필수
                    println("Failed to connect: ${future.cause().message}")
                }
            }
        }
        // 모든 연결이 완료될 때까지 대기하거나 로깅
    }

    // 세션 빌려오기
    fun acquire(): Channel? = pool.poll(5, TimeUnit.SECONDS)

    // 세션 반납하기
    fun release(channel: Channel) {
        if (channel.isActive) {
            pool.offer(channel)
        } else {
            // 끊어진 채널이면 새로 연결해서 채워넣는 로직 필요
            reconnect(channel)
        }
    }
}
```

세션을 미리 맺어두면 3-way-handshake 20ms ~ 100ms, tls/ssl handshake 및 암호화 인증등 200ms~300ms 등 합산해서 220ms ~ 400ms 정도까지 미리 아껴둘수있다는 점이 있다.

이런 커넥션을 유지하다보면 nice측 방화벽이나 L4 스위치에서 세션은 연결되어있는데 실제 데이터는 안흐르는 idle 상태를 감지하고 강제로 끊어버리는 경우도 있다. tcp keepalive가 되었다거나. 

그래서 이런 고스트 세션을 방지하기 위해서 netty 파이프라인에서 주기적으로 더미 전문을 보내는 heartbeat로직을 두어 세션을 유지시켰다 물론 tcp keepalive 자체를 늘려놓기도 했다. 

그리고 중복 유저에 대해서는 애초에 유저 자체를 우리 시스템과 나이스등에서도 키로 잡고있으므로 문제없다. 멱등하게 동작한다. 그리고 신용점수라는 데이터 자체는 많이 변경되지 않을 뿐더러, 사전조회에서 조회한것이 끝이 아닌 이후 

대출 심사, 최종 기표전 CB조회등 많은 프로세스들에서 여러번 더 조회하므로 사전조회시에 신용점수 정확성은 엄청나게 중요하지는 않다.

일단 여러서비스에서 나이스 요청이 느려지더라도 처리할 수 있는 기반을 마련해두긴했다. 

그러나 그럼에도 많은 요청이 오게되면, nice에게 더 많은 부하를 주게 될것이고, 컨슈머 서버에도 nice에도 부하가 가게된다.

이럼 여기서 **서킷 브레이커 패턴이 필요하다.**

### 서킷 브레이커

80개 세션과 sqs 컨슈머 구조를 고려할때 다음과 같은 기준으로 서킷을 오픈해둘수있겠다

지연 시간 기준, 에러율 기준, 포화도 기반

#### 지연 시간 기준

가장 먼저 반응해야 될거같은데 80개 세션 환경에서 응답이 느려지면 모든 세션이 점유 상태가 되어 사실상 시스템이 멈춘다.

- 조건: 최근 100개의 요청 중 5초 이상 걸리는 응답이 50%이상일때로 걸어뒀다.
- 이유: 5초이상 걸리는 요청이 절반을 넘어가면 산술적으로 초당 처리량이 16TPS로 떨어진다 80/5기 때문이다. 유입이 200TPS라면, 순식간에 큐가 폭발하므로 미리 차단하여 유저에게 잠시 후 다시 시도해달라고 응답을 주는것이 낫다. 포기하거나

#### 에러율 기준

nice 측의 시스템 장애나 네트워크 오류를 감지한다.

- 조건: 최근 100개의 요청중에 에러 5xx 혹은 전문 통신 오류 발생률이 20~30%이상일때
- 이유: 금융 전문 통신에서는 20% 이상의 에러는 일시적인 네트워크 순절이 아닌 상대측 기관의 심각한 장애로 간주해야한다. 이 상황에서 계속 요청을 보내는것은 의미가 없는 리소스 낭비라 상대방시스템 복구를 방해하는 행위가 될 수 있다.

20%도 좀 많다고 생각한다 개인적으로는


#### 포화도 기반 Saturation

이건 네티에 거는게 아니라 사전조회 서비스에 거는건데

일반적인 서킷브레이커 라이브러리 Resilience4j에는 없지만, 여기 sqs 구조에서는 SQS Lag, 세션 획득 대기시간을 조건으로 걸어야한다.

SQS `ApproximateAgeOfOldestMessage` (가장 오래된 메시지 나이가) 60초를 초과하게 될때 큐에 들어온지 60초가 지난 메세지를 이제 처리한다는것은 이미 유저나 upstream 플랫폼에서는 타임아웃이 났을수도 있고, 이때 여기서 서킷을 오픈해 신규 유입을 즉시 거절하고 컨슈머가 큐에 쌓인 유효하지 않은 메시지들을 빠르게 소화할 시간을 벌어줘야한다.

#### 판단 윈도우

일시적인 지연과 장애를 구분하기 위해 sliding time window 방식을 사용한다.

설정 예시로 윈도우 사이즈는 30초 정도로 설정하고 피크 트래픽 200tps 상황에서 30초면 약 6,000건의 요청이 인입된다.

100개라는 최소 요청수 조건을 충족하기엔 충분한 시간이며, 너무 짧으면 네트워크의 일시적인 튀는 현상 spike에 민감하게 반응하고, 너무 길면 장애 대응이 늦어지기 때문이다.

#### 모니터링

단순히 서킷이 열리는것에 그치지 않고 모니터링을 진행해주어야한다.

메트릭은 Resilience4j등의 라이브러리에서 제공하는 state(Closed, Open, Half-open) 지표와 failure_rate를 Prometheus로 수집하여 Grafana에 시각화해야한다.

서킷 상태가 closed 에서 open으로 변하는 즉시 slack 알람을 통해 온콜 엔지니어가 인지하도록 설정했다. 이때 nice 시스템의 응답 속도 지표와 우리 컨슈머 sqs Lag 지표를 함께 비교해서 원인이 내부인지 외부인지 파악해야한다.

#### 수동 제어

자동화된 시스템도 완벽할 수 없기에 운영의 개입 수단은 마련해놔야한다.

만약 나이스쪽에 장애가 해결되었음에도 서킷 브레이커나 에러율 계산 방식 때문에 closed 상태로 돌아오는게 늦어진다면 spring actuator나 관리용 api 엔드포인트를 통해 서킷 상태를 강제로 closed로 변경하거나 잠시 서킷 기능을 비활성화 할 수 있는 성질들을 갖추어둘 수 있다.

비즈니스 판단으로 금융 도메인의 1% 신청 고객도 소중하기 때문에 장애 복구 확인 ㅅ시 시스템의 자동회복 half-open을 기다리기 보다 엔지니어가 직접 상태를 확인하고 정상화하는 판단이 필요할 수 있기 때문이다.

netty기반의 pure kotlin system이므로 다음과 같이 제어할 수 있다.

```kt
import io.github.resilience4j.circuitbreaker.CircuitBreaker
import io.github.resilience4j.circuitbreaker.CircuitBreakerConfig
import io.github.resilience4j.circuitbreaker.CircuitBreakerRegistry
import java.time.Duration

object NiceCircuitManager {
    // 1. 서킷 브레이커 설정
    private val config = CircuitBreakerConfig.custom()
        .failureRateThreshold(20.0) // 에러율 20% 이상 시 오픈
        .waitDurationInOpenState(Duration.ofSeconds(30)) // 오픈 후 30초 대기
        .slowCallDurationThreshold(Duration.ofSeconds(5)) // 5초 이상이면 느린 호출
        .slowCallRateThreshold(20.0) // 느린 호출 20% 이상 시 오픈
        .minimumNumberOfCalls(100) // 최소 100개 요청부터 판단 시작
        .slidingWindowSize(100) // 윈도우 사이즈 100개
        .build()

    // 2. 레지스트리 생성 및 인스턴스 등록
    val registry: CircuitBreakerRegistry = CircuitBreakerRegistry.of(config)
    val niceCircuitBreaker: CircuitBreaker = registry.circuitBreaker("niceProxy")
}
```

그리고 마이크로미터를 넣고 액츄에이터가 없으므로 직접 구현한다.

resilence4j의 메트릭을 직접 바인드한다.

```kt
import io.github.resilience4j.micrometer.tagged.TaggedCircuitBreakerMetrics
import io.micrometer.prometheus.PrometheusConfig
import io.micrometer.prometheus.PrometheusMeterRegistry

object MetricsManager {
    // 1. Prometheus 레지스트리 생성
    val prometheusRegistry = PrometheusMeterRegistry(PrometheusConfig.DEFAULT)

    init {
        // 2. Resilience4j 메트릭을 Prometheus에 바인딩
        TaggedCircuitBreakerMetrics
            .ofCircuitBreakerRegistry(NiceCircuitManager.registry)
            .bindTo(prometheusRegistry)
            
        // 3. JVM 기본 메트릭(CPU, Memory)도 함께 바인딩 (SRE 필수)
        io.micrometer.core.instrument.binder.jvm.JvmMemoryMetrics().bindTo(prometheusRegistry)
        io.micrometer.core.instrument.binder.system.ProcessorMetrics().bindTo(prometheusRegistry)
    }

    // 4. Prometheus가 Scrape해갈 텍스트 결과물 반환
    fun getMetrics(): String = prometheusRegistry.scrape()
}
```

수동제어 구현

```kt
class NiceAdminService(private val circuitBreaker: CircuitBreaker) {

    // 강제로 서킷을 정상 상태로 복구 (Manual Close)
    fun forceClose() {
        circuitBreaker.transitionToClosedState() 
        println("Circuit Breaker [${circuitBreaker.name}] has been manually CLOSED.")
    }

    // 강제로 서킷을 차단 (Manual Open)
    fun forceOpen() {
        circuitBreaker.transitionToForcedOpenState()
        println("Circuit Breaker [${circuitBreaker.name}] has been manually FORCED_OPEN.")
    }

    // 서킷 상태 확인
    fun getStatus(): String = circuitBreaker.state.name
}
```

해당 서비스를 만들어두고 기존포트 80을 별도로 8081이나 그런 포트들을 만들어두고

```kt
// 관리용 전용 Netty Handler 예시
class AdminHttpHandler(private val adminService: NiceAdminService) : SimpleChannelInboundHandler<FullHttpRequest>() {
    override fun channelRead0(ctx: ChannelHandlerContext, req: FullHttpRequest) {
        val uri = req.uri()
        
        val responseText = when {
            uri == "/admin/circuit/close" -> {
                adminService.forceClose()
                "Circuit manually CLOSED"
            }
            uri == "/admin/circuit/open" -> {
                adminService.forceOpen()
                "Circuit manually FORCED_OPEN"
            }
            else -> "Unknown command"
        }
        
        sendResponse(ctx, responseText)
    }
}
```

다음처럼 제공해 circuit을 관리한다.

```kt
class NiceNettyHandler(private val circuitBreaker: CircuitBreaker) : ChannelInboundHandlerAdapter() {
    
    override fun channelRead(ctx: ChannelHandlerContext, msg: Any) {
        // 1. 서킷 권한 획득 확인 (Non-blocking)
        if (!circuitBreaker.tryAcquirePermission()) {
            // 서킷이 오픈된 상태라면 즉시 거절
            ReferenceCountUtil.release(msg)
            ctx.writeAndFlush(ErrorResponse("CIRCUIT_OPEN"))
            return
        }

        val start = System.nanoTime()
        // 2. 실제 NICE 전문 전송 로직 수행 (예시)
        ctx.fireChannelRead(msg) 
        
        // 3. 성공/실패 기록은 해당 작업이 완료되는 Future Listener에서 수행
        // circuitBreaker.onSuccess(duration, TimeUnit.NANOSECONDS)
        // circuitBreaker.onError(duration, TimeUnit.NANOSECONDS, throwable)
    }
}
```

http 말고도 jmx MBean으로 등록해서 jconsole or VisualVM 같은 툴에서 서버에 접속해 ui 상에 forceClose()를 사용할수도 있다고한다.

혹은 dynamicConfig같은걸로 config storage를 이용해볼수도있다.

자 이렇게 해서 나이스 프록시 서버에 대한 유량 제어와 세션 80개지만 수많은 요청을 받아둘 수 있는 기반이 마련되었다.

지금 요청량에선 인스턴스 하나로도 충분히 감당이 가능하므로 워커스레드는 16개임. 딱히 문제는 없을것이다.

일단 파드 1대가 장애가 나도 남은 1대가 평균 트래픽을 소화할 수 있어야하기 때문에 두대로 나이스 프록시를 띄워둔다. 

가장 중요한건 나이스 자체기때문에 coroutine이라면 Semapore, 혹은 pure java라면 ConenctionPool(80)로 80개 세션을 정확히 유지해두자.

sqs에서 30초 분량 약 2000~3000건의 버퍼를 두고 이를 초과하는 트래픽은 서킷 브레이커로 즉시 드랍이 아닌 시스템을 살려놓자.

### 사전조회 컨슈머 성능 개선

자 이번엔 사전조회 시스템을 확인해보자

사전조회 시스템은 nice 뿐만 아니라 우리 코어뱅킹 시스템에 고객등록, 내부 CSS(Credit Scoring Service)등의 작업도 진행하고 응답처리를 해야한다.

여기서 nice, kcb 콜같은 경우는 각각 동시에 처리할 수 있으므로 coroutine을 통해 병렬 처리해 레이턴시를 낮춰볼수잇다.

nice 2초 kcb 1초, 내부 css 1초, 코어뱅킹등록 2초가 걸린다면? 전부다 처리하는건 6초가 걸린다 여기서 코어뱅킹등록에는 nice kcb 내부 css 정보가 필요하니 마지막에 진행하고 nice kcb css를 동시에 처리하면 4초만 걸리게 되는것이다.

기존 동기식 코드는 아래와 같다

```kt
fun registerLoanSequential(user: User): RegistrationResult {
    val start = System.currentTimeMillis()

    // 1. NICE 조회 (2s)
    val niceData = externalService.fetchNiceData(user)
    
    // 2. KCB 조회 (1s) - NICE가 끝날 때까지 대기함
    val kcbData = externalService.fetchKcbData(user)
    
    // 3. 내부 CSS 산출 (1s) - KCB가 끝날 때까지 대기함
    val cssData = internalService.calculateCss(user, niceData, kcbData)

    // 4. 코어뱅킹 등록 (2s) - 위 모든 정보가 필요함
    val result = coreBankingService.register(user, niceData, kcbData, cssData)

    println("Total Time: ${System.currentTimeMillis() - start}ms") // 약 6000ms
    return result
}
```

```kt
suspend fun registerLoanParallel(user: User) = coroutineScope {
    val start = System.currentTimeMillis()

    // 1~3번 작업을 동시에 시작 (Non-blocking)
    val niceDeferred = async { externalService.fetchNiceData(user) }
    val kcbDeferred = async { externalService.fetchKcbData(user) }
    
    // 내부 CSS 산출 (NICE/KCB 데이터가 CSS 산출에 필요하다면 CSS는 await 후 실행해야 함)
    // 여기서는 CSS가 독립적이라고 가정하셨으므로 병렬 실행
    val cssDeferred = async { internalService.calculateCss(user) }

    // 모든 작업이 끝날 때까지 대기 (가장 오래 걸리는 NICE의 2초에 수렴)
    val niceData = niceDeferred.await()
    val kcbData = kcbDeferred.await()
    val cssData = cssDeferred.await()

    // 4. 코어뱅킹 등록 (2s) - 결과가 다 모인 시점(2s)에 실행
    val result = coreBankingService.register(user, niceData, kcbData, cssData)

    println("Total Time: ${System.currentTimeMillis() - start}ms") // 약 4000ms
    result
}
```

위처럼 병렬처리로 등록해두면 된다. 각 개별 작업에 개별 타임아웃 설정은 필수이며 쓰레드 점유는 짧지만 동시 요청 폭증은 주의해두자.

근데 만약 하나의 시스템에서 오류가 난다면? 그건 실패해야한다. async 블록 하나에서 예외가 발생하면 부모 코루틴까지 장애가 전파된다. 모두 취소된다는 뜻이다.

보통 연관이 없다면 supervisorScope를 통해 fault toleration 한 시스템을 구축해볼 수도이 있겠지만 데이터 정합성 측면때문에 실패하면 아예 실패인걸로.

예외 발생시에 수기 대응하는걸로 구축해둔다.

실패한 요청은 DLQ나 transaction outbox pattern에 기반한 db에 이벤트를 남겨두고 개발자가 수기대응 및 알림 시스템을 넣어놓고 메뉴얼하게 처리하도록 해둔다.

이 부분도 자동화해볼 수 있겠으나 일관성 측면에서 일단 둔다.

사전조회 프로세스가 시작될때 먼저 db에 pending 상태의 레코드를 생성해둔다.

코루틴 병렬 작업중에 하나라도 실패하여 exception이 발생하면 catch 블록에 해당 레코드의 status를 FAILED로 업데이트하고 실패한 원인을 상세하게 기록한다.

이때 Transation outbox Pattern으로 db업데이트와 동시에 수기 확인이 필요하는 메시지를 큐에 발행한다.

ErrorQueue는 개발자가 운영에서 확인해야할 실패한 작업이고 DLQ는 Error Queue를 처리하는 컨슈머 조차 로직 오류나 인프라 장애로 실패했을때 최종적으로 도달하는 보루다.

```kt
suspend fun handleInquiryFailure(userId: String, error: Throwable) {
    db.transaction {
        // 1. DB 상태 업데이트 (NICE_KEY 등이 null인 상태로 FAILED 마킹)
        inquiryRepository.updateToFailed(userId, error.message)
        
        // 2. Outbox 테이블에 수기 대응 이벤트 삽입
        // 이 이벤트는 별도 Relay를 통해 SQS(Error Queue)로 전송됨
        outboxRepository.save(ManualInquiryEvent(userId = userId, reason = error.message))
    }
}
```

여기서 코어뱅킹 유저등록 시스템 데이터를 db에 넣어두는데 nice, kcb와 같은 조회결과 데이터의 키를 집고있고 key가 null인것들이 실패한 기관이기 때문에

개발자가 이게 null이고 특정 시점에 대한 유저들에 대해서 다시 retry하는 콘솔을 만들어둔다면 처리할 수 있다.

그럼 그 key가 null인 데이터들에 대해서만 다시 재시도를 진행하고 모든 key가 non-null 조회 성공이 되었다면 그 이후 프로세스를 밟아

응답을 보내줄 수 있을 것이다. rdb를 사용하면 부하감당에 무리가있겠지만 현상황에서 트래픽엔 충분하다.

근데 만약 트래픽이 rdb로 사용할수 없을정도로 늘어나게되면 CDC 기반 메커니즘으로 outbox 테이블을 따로두고 insert로 추가하는 대신 db의 wal log를 직접 읽어  처리하는 방식으로 저장해둬야한다 Debezium Kafka Connect 조합이 요즘 표준이다. no-sql이나 local log append등도 있겠지만

wal을 읽는 방식이 더 빠른 이유는 out box는 insert + update + outbox insert 라는 두번의 쓰기가 발생하지만 cdc는 애플리케이션 비즈니스 로직만 수행해 wal은 디비 복구나 복제를 위해 어차피 남겨두는 데이터인데 cdc는 이미 기록된 파일을 비동기적으로 읽기만하므로 애플리케이션의 트랜잭션에는 추가적인 부하를 주지않는다.

변경이력 자체를 저장하는것이므로 outbox table이 아니라 비즈니스 로직 insert만 읽는다. 변경 이력을 보는것이니까 오 비즈니스 변경이력이 실패ㅐ네? 즉 outbox 테이블의 역할 자체를 쟤가 하는거다.

<br>

## 트래픽이 200TPS -> 2000TPS -> 20000TPS로 늘어난다면,

이렇게 되면 단순하게 서버 대수 늘린다는 수준을 넘어서 아키텍처 페러다임을 바꿔야하는 지점이다.

### 2000TPS 단계: 리소스 최적화 및 인프라 고도화

이 단계에서는 기존 rdb와 기본적인 오토스케일링이 한계에 도달한다.

HPA & Tuning: 파드수를 수십대 수준으로 늘리고 **커널 파라미터 튜닝 somaxconn, file-max** 필수적이다.

Netty의 epoll 기반 최적화와 함께, 불필요한 gc 부하를 줄이기위해 direct buffer 사용량을 정밀하게 모니터링 해야한다.

그리고 아까말한 CDC를 도입해야한다. RDB는 초당 2000개의 outboux write를 견디기가 버거워지고 wal기반 cdc(debezium kafka)로 완전히 전환해 db 쓰기 부하를 줄여야한다.

외부 연동 세션은 음 이건 답이없다 늘려야한다 물리적인 세션수를 80개에서 800개로 늘려야지. 이때 세션 관리 효율을 위해 세션 풀을 여러 파드가 공유하는 구조나, 세션 전용 게트웨이 배치를 고려해야한다.

### 20000TPS 단계

이 단계는 세계적인 트래픽 수준인데 단일 클러스터나 디비로는 감당이 힘들다.

**셀 기반 아키텍처**. 모든 트래픽을 하나의 거대한 클러스터에서 처리하지 않고 셀이라는 단위로 격리한다.

특정 사용자 그룹 ex. userid기준 샤딩 별로 독립적인 netty proxy 그룹과 디비, 큐를 배정해야한다.

특정 셀에서 장애가 발생해도 20000tps중 일부만 영향이 받으며 장애 전파를 완벽히 차단해낸다.

**적응형 부하 제어(Adaptive Concurrency Limiting)** 

단순한 고정 임계치 서킷 브레이커는 20000tps 환경에서 너무 늦게 반응하거나 불필요하게 예민할 수 있음

실제 netflix의 concurrency-limits 라이브러리처럼 리틀의 법칙을 이용해 실시간 지연 시간에 따라 유동적으로 트래픽을 허용처리하는 적응형 리미터를 도입해놓자.

nice 시스템이라고 쳤을때 시스템 응답이 10ms만 늦어져도 즉시 유입 트래픽을 조절해 시스템 붕괴를 막는다.

**엣지 로직 사전 필터링**으로 20,000건의 요청이 모두 비즈니스 로직까지 도달하게 두면 안된다. 여기선 netty proxy

AWS WAF나 API Gateway 레벨에서 redis 기반 글로벌 rate limiter나 그런것들을 두어 유효하지 않은 요청이나 중복 요청은 엣지단계에서 즉시 드랍하도록 해둘수있다.

### 외부 시스템 수용량

nice라고 쳤을때 해당 시스템 자체가 20,000tps를 견딜지가 문제다.

비즈니스 우선순위화를 먼저 해두고 만약 nice가 5000tps까지만 지원한다면 나머지 15000건을(다른 콜에 대한 요청을 고려하면 17000건정도) 버려야한다. 이때 vip 고객이나 대출 승인 임박같은 우선순위에 따라 꺼내오는 priority queing을 구현해야한다.

커널 레벨 최적화도 필요한데 tps를 위해 수천개의 tcp 커넥션을 유지하려면 파드당 FD(File Descriptor) 제한과 tcp port 고갈같은 문제를 해결해야한다.

이를 단일 ip가 아닌 VIP(virtual ip)를 활용한 아웃바운드 트래픽 분산이 필요하다.