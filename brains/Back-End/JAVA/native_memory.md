# Native Memroy

jvm의 native memory는 jvm이 힙 외부에서 사용하는 메모리를 의미하는데 다시 말해서 객체를 저장하는 힙이 아닌 운영체제가 직접 할당해준 메모리를 의미한다.

java heap은 jvm이 gc로 관리하며 -Xmx로 보통 사이즈를 지정하지만 native memory는 jvm 내부 구조 또는 라이브러리가 운영체제에서 직접 요청해서 사용하는 메모리다.

그래서 jvm은 전체 메모리 사용량이 java heap + native memory + code cache 정도가 되겠다.

native memory는 보통 metaspace, theread stack, direct byte buffer, jni allocated area등이 사용한다.

```java
ByteBuffer buffer = ByteBuffer.allocateDirect(1024 * 1024 * 100); // 100MB
```

위에처럼 할당한 메모리는 java heap에 나타나지도 않고 gc대상도 아니라서 직접 수거해야한다.

<br>

### jcmd

`ps aux`로 자바 프로세스 찾고 jcmd로 jvm의 상태를 파악해볼 수 있는데

jcmd는 jvm 모니터링을 위한 툴로 java 프로세스를 식별하고 힙 덤프를 따거나 스레드 상태를 확인하는등의 분석 정보를 얻을 수 있는 도구다.

운영중인 톰캣 서버가 갑자기 메모리가 부족해지거나 cpu가 튈때 같은 문제들이 발생하면 jcmd로 어느 프로세스가 문제가 있는지 식별하고 힙이나 스레드 관련 정보를 즉시 덤프 받아 원인을 확인해볼 수 있다.

```bash
jcmd <pid> VM.native_memory summary
```

```yaml
Native Memory Tracking:
Total: reserved=256MB, committed=192MB
- Java Heap: 128MB
- Class: 10MB
- Code: 20MB
- GC: 15MB
- Thread: 8MB
...
```

위처럼 확인해보고 식별부터 메모리 누수, 데드락이나 gc의 비효율을 찾아 jvm 문제를 파악해볼 수 있다

```bash
# thread stack trace
jcmd <pid> Thread.print

# thread heap dump
jcmd <pid> GC.heap_dump /path/to/dump.hprof
```

### NMT

JVM은 Native Memory Tracking(NMT) 기능을 통해서 네이티브 메모리 사용량 추적이 가능하다.

```bash
java -XX:NativeMemoryTracking=summary -XX:+UnlockDiagnosticVMOptions -XX:+PrintNMTStatistics ...
```

위처럼 옵션을 주고 jar를 돌려서 활성화 시킬 수 있다. 저걸 활성화 시켜놓고 jcmd를 통해서 네이티브 메모리를 트래킹해볼 수 있다.

```bash
# 특정 ps
jcmd <PID> VM.native_memory summary

# 전체 ps
jcmd | grep java
```

```yaml
Native Memory Tracking:

Total: reserved=242MB, committed=178MB
- Java Heap (reserved=128MB, committed=128MB)
- Class (reserved=20MB, committed=15MB)
- Thread (reserved=15MB, committed=15MB)
- Code (reserved=25MB, committed=20MB)
- GC (reserved=30MB, committed=25MB)
- Compiler (reserved=8MB, committed=7MB)
- Internal (reserved=5MB, committed=4MB)
```

> 참고로 기본적으로 NMT는 꺼져있고 켜두면 추가 오버헤드 1~10퍼정도 감수하고 추적을 한다, 성능이 민감한 운영 환경에서는 summary로 해두자.

사실 모순인게 성능이 민감한 운영 환경에서는 더더욱 해당 정보가 필요할듯 싶은데 pmap으로 확인하거나 ps top smem으로 메모리 사용량 보고 혹은 jemalloc으로 메모리 할당하고 jemalloc_stats_print로 보는 방법과 같은 외부 할당기, 프로파일러를 사용하는것도 추천이다.

https://www.facebook.com/notes/facebook-engineering/scalable-memory-allocation-using-jemalloc/480222803919

FreeBSD에서 사용하려고 개발된 메모리 할당기고 다중 스레드 환경에서 효율적인 메모리 할당과 관리를 제공하고 특시 대규모 멀티스레드 애플리케이션 성능을 최적화한다.

jemalloc은 다음글에서 더 알아보겠다.