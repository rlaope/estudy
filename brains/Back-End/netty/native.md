# Netty Native Memory Tuning, BackPressure

네티에서 네이티브 메모리는 보통 아래를 합쳐서 말하는데

- **Direct ByteBuf(off-heap)**: ByteBufAllocator가 ByteBuffer.allocateDirect() 계열로 잡는 메모리
- **Netty Pooled allocator의 청크/아레나 구조**: 미리 큰 덩어리를 잡아두고 쪼개 쓰는 구조
- 부자거으로 JNI/SSL(OpenSSL), 압ㅊ축 라이브러리등이 있으면 거기도 네이티브가 추가로 붙음

예시로 전문통신에 전문사이즈가 14kb라면 direct도 14kb만 쓰는게 아니라 풀링구조와 캐시 backlog 때문에 수치가 더 커질 수 있다.

### why off-heap

소켓 io는 커널 버퍼 <-> 유저 공간으로 데이터가 오가는데, **Direct는 중간 복사 특히 heap -> direct** 비용을 줄일수가 있어 유리한 구간이 있다.

netty는 고성능을 위해서 기본적으로 Pooled + Direct 쪽을 사용한다.

그러나 단점은 gc가 관리하지 않는 영역이기 때무넹 누수나 backlog가 생기면 힙은 멀쩡한데 direct만 터지는 상황이 생긴다.

그래서 운영에서는 상한 + 관측 + 백프레셔 + 누수 방지가 필수이다.

예를들어 14kb 전문에서 200rps까지의 요청이 들어온다고 가정을 해보자.

페이로드의 처리량은 매우 적을것이다 `14kb x 200/s = 2,800kb/s` 즉 2.8mb/s 인데 RTT를 50~100ms로 가정하면 inflight는 50ms: 200 x 0.05 = 10, 100ms: 20인것이다.

요청 + 응답을 넉넉히 두배로 잡아도 20 x 14kb x 2 = 560kb로 순수 데이터로는 1mb도 안되는 규모다.

그런데 운영에서는 direct 메모리 usage를 보면 수백mb로 보일텐데 원인은 항상 두가지다

- **Pooled allocator의 바닥 사용량(청크/아레나/스레드캐시)**
  - 작은 버퍼를 그때그때 os에서 잡지않고 큰 덩어리(청크)로 확보 후 재사용하는것
  - 워커 스레드 수가 늘수록 arena/caching 때문에 바닥이 생김
- **outbound backlog(상대가 느릴때 write큐가 쌓임)**
  - write가 os 송신버퍼에 다 못 들어가면 netty outbound buffer에 쌓임
  - 이게 direct peak를 수백 mb로 끌어올리는 1순위 요인이다.

그래서 측정전 가설, 예산으로 제시후에 실측으로 검증 시나리오를 해보면

- 평시(바닥) 150~250mb
  - 풀링 구조/스레드 캐시로 인해 트래픽이 작아도 이정도 차있는 상태
- 피크(상대지연 + backlog): 300 ~ 450mb
  - 상대 지연이 생겨 flush가 밀리면 이 구간으로 튈 수도 있다.
- 운영 예산 상한에서는 기본 256mb로 주면 될거고 지연 변동/연결수없음/ssl포함이면 512mb로 주자.

<br>

### OutOfDirectMemoryError

heap gc는 괜찮은데 direct만 지속증가하고 트래픽이 일정해도 증가하는 현상을 겪을때가 있을수있다.

흔한 원인으로는 retain(), retainSlice()이후에 release()를 안해줘서 메모리 영역을 계속 들고있어 누수가 생기는 문제가 있을수있다.

SImpleChannelInboundHander에서는 autoRelease 동작을 모르고 참조를 밖으로 뺐다거나

예외 경로에서 release하는 코드를 누락했다거나 등이 있다.

스테이징에서 LeakDetector로 정확한 스택 트레이스를 확보해 소유권 규칙을 정하고 코드에 강제시켜 방지해볼수있다.

> Netty의 ByteBuf(참조 카운트 객체) 누수를 잡기 위한 디버깅 기능이다

```java
@Override
public void channelRead(ChannelHandlerContext ctx, Object msg) {
  try {
    // 처리 로직
  } finally {
    ReferenceCountUtil.release(msg);
  }
}
```

누수는 LeakDetector로 스택 잡고 소유권 규칙으로 재발을 방지하자.

### LockDetector

`ResourceLeakDetector`로도 부르는데 Netty의 ByteBuff(참조카운트객체) 누수를 잡기위한 디버깅 기능이다.

- Netty ByteBuf(특히 Direct)는 gc가 자동으로 회수하지 않고 release()로 참조 카운트를 0으로 만들어야 메모리가 반환된다.
- 어디선가 retain/retainedSlice()를 들고 release를 안하면 direct 메모리가 증가한다
- LeakDetector는 이 bytebuf가 해제되지 않았다를 감지해 누수가 발생한 코드 경로를 스택트레이스로 남긴다.

Netty가 ByteBuf를 할당할때 샘플링/옵션에 따라서 해당 버퍼에 대한 추적 객체(약한 참조 기반)를 붙여두고 버퍼가 gc대상이 되었는데도 release()로 정상 해제된 기록이 없으면 "LEAK: ByteBuf.release() was not called"류의 로그를 찍는다.

로그에서는 이 버퍼가 어디에 retain 되어있는지 스택에 포함된다 즉 direct memory를 os에서 바로 읽는게 아니라 refCnt 해제 누락을 추적하는 방식이다.

#### Level

LeakDetector는 성능 비용을 줄이기 위해 얼마나 빡세게 추적할지를 기준으로 레벨이 존재하는데 버전마다 세부는 다르지만 

- **Simple:**: 샘플링으로 일부만 추적한다 누수는 잡을 수 있지만 스택 정보가 제한적일 수 있다. 비용이 비교적 낮다.
- **ADVANCED**: Simple보다 더 많은 더 자세한 스택을 추적하고 스테이징에서 누수 잡을때 많이 쓴다 비용은 올라가지만 어느 코드가 누수 냈는지 찾기 쉽다
- **PARANOID**: 거의 모든 할당(또는 매우 높은 비율)을 추적해서 가장 잘잡는다 대신 비용이 가장 크다(성능저하 로그 폭증 가능) 보통 재현이 잘 안되는데 반드시 잡아야할때 사용.

그리고 운영에서는 오버헤드 때문에 꺼놓아도 괜찮다. 테스트를 잘했다고 확신하면 말이다.

```
-Dio.netty.leakDetection.level=advanced
# 또는 paranoid / simple / disabled

or

ResourceLeakDetector.setLevel(ResourceLeakDetector.Level.ADVANCED);
```

<br>

### Write Backlog 폭증

상대 서버가 느려지면 응답이 느려질것이고 동시에 direct memory 사요ㅛㅇ량이 피크로 치솟을 수 있다. 계속 들고있기 때문에

채널이 한동안 계속 write를 못따라갈수있다 (partial write)

읽기는 계속 당기고 AUTO_READ=false, 쓰기는 못내보내서 outbound buffer 적재때문에 버퍼가 커져 발생할 수 있는데

해결방법은 WriteBufferWaterMark 설정으로 unwritable 전환을 해주는 것이다.

channelWritabilityChanged에서 **읽기 중단/재개**가 가능하고 flush 배치를 하자(가능하면)

```java
childOption(ChannelOption.WRITE_BUFFER_WATER_MARK,
    new WriteBufferWaterMark(32 * 1024 * 1024, 64 * 1024 * 1024)); // 32MB/64MB

childOption(ChannelOption.AUTO_READ, false); // 수동 read 제어

@Override
public void channelWritabilityChanged(ChannelHandlerContext ctx) {
  if (ctx.channel().isWritable()) {
    ctx.read(); // 재개
  } // unwritable이면 읽기 중단 상태 유지
  ctx.fireChannelWritabilityChanged();
}
```

정리하면 피크는 대부분 backlog가 원인이라 watermark, read 제어로 폭증을 막아볼 수 있는것.

#### unwritable 이후에 실제 일어나는 동작

Netty 내부에서 
- totalPendingWriteBytes >= highWaterMark이 된다면
- channel.isWritable()이 false로 바뀐다
- channelWritabilityChanged() 이벤트가 발생한다
- 원인 대부분: 소켓 write가 밀려서 ChannelOutboundBuffer에 미전송 데이터가 쌓인다.

서버가 해야하는 동작은 추가 write 생성을 중단해야한다 if(!ch.isWritable) { produce 중단 } 등의 로직을 넣고 unwritable인데도 계속 write하면 outbound가 더 쌓여 direct 폭발하니 그걸 하지않도록 한다.

inbound 유입도 차단해두는게 좋은데 프록시 동기 응답형 서버면 AUTO_READ = false라면 unwritable에서 read를 멈춤 상태를 유지시키고 AUTO_READ=true면 setAutoRead(false)를 줘서 유입을 차단하게 만들어야한다.

이미 받아버린 요청에 대한 정책은 세가지중 하나인데
1. bounded queue에 적재시켜놓고 대기시킨다(상한은 필요함)
2. 즉시 거절(busy/overload)
3. 연결 종료 fail-fast

핵심은 unwritable구간은 처리 계속이 아니라 증가를 멈추고 유입 생산 중단, 쌓인것을 빼는 시간이다.

#### 다시 writable이 되었을때 동작

Netty 내부에서

- outbound가 전송되어 totalPendingWriteBytes가 감소한다.
- totalPendingWriteBytes <= lowWaterMark가 된다.
- channel.isWritable()이 true로 복귀된다
- channelWritabilityChanged() 이벤트가 발생한다.

서버는 멈춰둔 read(AUTO_READ=false면 ctx.read호출, true면 setAutoRead(true))를 재개시킨다.

멈춰둔 write도 재개시켜야하고 unwritable동안 멈춰있던 전문 생성/큐소비/프록시 전달을 다시 시작한다.

단 재개시 한번에 폭주하지 않게 배치 속도 제한을 둔다.

대기중인 내부 큐가 있으면 drain해야한다. unwritable일때 큐잉한경우 전송을 재개시키고 isWritable 체킇하면서 조금씩 drain하자.

<br>

### Observability

JVM NMT(Native Memory Tracking)

운영/stage에서 NMT를 켜면 네이티브 메모리 구성을 볼 수 있다.

```
-XX:NativeMemoryTracking=summary
-XX:+UnlockDiagnosticVMOptions
```

```bash
jcmd <pid> VM.native_memory summary
jcmd <pid> VM.native_memory detail
```

[Netty allocattor metric](https://github.com/micrometer-metrics/micrometer/blob/main/micrometer-core/src/main/java/io/micrometer/core/instrument/binder/netty4/NettyAllocatorMetrics.java)으로 Pooled allocator는 사용량/아레나 상태를 메트릭으로 뽑을 수 있는데 운영에서는 이값 + 프로세스 rss + nmt를 같이 봐야 진짜 원인이 보인다.

LeakDetector도 쓸수있는데 사용원칙은 개발 스테이징에서는 ADVANCED 필요시 PARANOID고 운영에서는 비용때문에 위에서 언급한것처럼 보통 끈다.

<br>

### 정리

WriteBufferWaterMark + AUTO_READ 제어로 backlog 지연 방지하고

direct memory의 상한을 두어 폭발도 방지해놓자.

```
-XX:MaxDirectMemorySize=256m
```

누수가 의심된다면, LeakDetector or Observability를 구축해 모니터링 해보도록

