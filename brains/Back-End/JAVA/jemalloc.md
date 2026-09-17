# jemalloc

jemalloc은 FreeBSD에서 사용하기 위해 Jason Evas에 의해 개발된 메모리 할당기로 이후 다른 운영체제에서도 지원된다고 한다. **다중 스레드 환경에서 효율적인 메모리 할당을 제공한다고 하며 대규모 멀티스레드 애플리케이션에서 성능을 최적화 할 수 있으며 낮은 메모리 오버헤드와 높은 확장성을 목표**로 한다.

기본 `glibc malloc`보다 성능과 메모리 추적 기능이 우수하다고하는데 직접 재본적은 없다.

메모리 할당 분포, 누수, 단편화 등을 추적할 수 있으며 jvm 내부에서 사용되는 native memory 의 할당을 추적하기 위해서 많이 쓰인다

```bash
sudo apt install libjemalloc-dev

# 설치되면 보통 /usr/lib/libjemalloc.so or /usr/local/lib/libjemalloc.so에 생김
LD_PRELOAD=/usr/lib/libjemalloc.so java -jar app.jar
```

위와 같이 돌리면 malloc을 호출할때마다 jemalloc을 사용하도록 된다.

jemalloc_stats_print()를 통해서 실시간 메모리 상태도 확인할 수 있다.

```
MALLOC_CONF=stats_print:true LD_PRELOAD=/usr/lib/libjemalloc.so java -jar your-app.jar
```

위처럼 환경변수를 줘서 stdout 자동 출력해도 되고 아니면 종료시 시그널 바등ㄹ때 stdout에 통계를 출력하는데 kill 줘서(ㅋㅋ) 보고싶으면 봐라 볼수있다면!

> 저 환경변수들은 따로 파일에 있는게 아니고 export, env, docker run -e 등으로 세팅해둬야한다. 그럼 jemalloc이 돌때 내부 파서가 해석해서 세팅해둔다.

결과는 보통 stderr stdout으로 출력되며 단 이 기능을 작동시키려면 jemalloc --enable-stats로 빌드했어야한다.

*출력예시*
```
___ Begin jemalloc statistics ___
Allocated: 200.2 MiB
Active: 256.0 MiB
Mapped: 512.0 MiB
Metadata: 2.3 MiB
...
Per size class statistics:
[   8] nalloc=1000000, nfree=500000, ...
___ End jemalloc statistics ___
```

<br>

### example

고성능 네트워크 서버에서 zero-copy based socket 처리시 allocateDirect()등을 사용해서 내부적으로 malloc하는 예시를 보여주겠다.

apache arrow라는 jvm에서 정형 데이터를 off-heap 메모리에 arrow 포멧으로 저장하고 분석할 수 있게 해주는 도구를 쓸거고 unsafe or malloc으로 메모리를 직접 할당해 gc에 영향을 주지 않는다.

```kt
import org.apache.arrow.memory.RootAllocator
import org.apache.arrow.vector.IntVector

fun main() {
    println("[1] Apache Arrow RootAllocator 생성 (malloc 기반)")
    val allocator = RootAllocator() // 내부적으로 native 메모리 영역 사용

    println("[2] Off-heap IntVector 생성")
    val vector = IntVector("intVector", allocator)
    vector.allocateNew(100_000)

    println("[3] 데이터 채우기")
    for (i in 0 until 100_000) {
        vector.setSafe(i, i * 2)
    }
    vector.valueCount = 100_000

    println("[4] 할당된 메모리: ${vector.valueCapacity * Int.SIZE_BYTES / 1024} KB")
    println("[5] 60초 대기: jemalloc, pmap 등으로 추적 가능")
    Thread.sleep(60_000)

    println("[6] 해제")
    vector.close()
    allocator.close()
}

```

```bash
MALLOC_CONF=stats_print:true \
LD_PRELOAD=/usr/lib/libjemalloc.so \
java -XX:+UnlockDiagnosticVMOptions -XX:NativeMemoryTracking=summary -cp . NativeMemoryExampleKt

#  죽이도 통계 출력보기, pmap -x <pid> | grep anon 으로 해도 됨
kill -USR1 <pid>
```

arrow 포멧 데이터는 cpu 캐시 친화적인 구조로 데이터를 메모리에 컬럼 단위로 정렬해 저장하는 포멧이다

예를 들어 일반 로우 기반 저장은

```
[
  {"id": 1, "name": "케이홉"},
  {"id": 2, "name": "희망"},
  {"id": 3, "name": "가재"}
]
```

위와 같은 식이라면 arrow columnar는 아래와 같다.

```
id:   [1, 2, 3]
name: ["케이홉", "희망", "가재"]
```

필드별로 연속된 메모리 공간에 저장해 벡터화후에 캐시 최적화로 성능을 끌어올린다 ~ 뭐 이런 취지다. zero copy 전송이니 빠르고 컬럼단위 연속 접근으로 cpu 지역성을 활용해 최적화된다고도 한다.

null bitmap과 최소한의 메타데이터로 고성능 애플리케이션 서버에서 사용된다.

<br>

### 결론

native memory를 직접적으로 조작하는 애플리케이션 속에서는 (bitmap 기반 통신) memeory를 조작할일이 많을텐데 프로파일링과 좀 더 고성능을 위해서는 jemalloc을 추천한다. 이건 간단 찍먹이라 어떤 알고리즘을 쓰는지 왜 더 좋은지, 구현방식이나 다른 개념(ex arena, thread cache 등)은 다음글에서 알아보겠다. 

그 밖에도 tcmalloc, hoard, ptmalloc2 등이 있다.