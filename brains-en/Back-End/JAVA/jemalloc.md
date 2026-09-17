# jemalloc

jemalloc is a memory allocator developed by Jason Evans for use in FreeBSD, and it is reportedly supported on other operating systems as well. **It is said to provide efficient memory allocation in multi-threaded environments, optimize performance in large-scale multi-threaded applications, and aims for low memory overhead and high scalability.**

It is said to offer superior performance and memory tracking capabilities compared to the default `glibc malloc`, though I haven't personally benchmarked it.

It can track memory allocation patterns, leaks, fragmentation, and is often used to track native memory allocations within the JVM.

```bash
sudo apt install libjemalloc-dev

# 설치되면 보통 /usr/lib/libjemalloc.so or /usr/local/lib/libjemalloc.so에 생김
LD_PRELOAD=/usr/lib/libjemalloc.so java -jar app.jar
```

Running it as above will cause `jemalloc` to be used every time `malloc` is called.

You can also check real-time memory status via `jemalloc_stats_print()`.

```
MALLOC_CONF=stats_print:true LD_PRELOAD=/usr/lib/libjemalloc.so java -jar your-app.jar
```

You can set environment variables like above to automatically print to stdout, or it prints statistics to stdout when it receives a termination signal. If you want to see it, you can try sending a `kill` signal (lol) if you can!

> These environment variables are not in a separate file; they must be set using `export`, `env`, `docker run -e`, etc. Then, when `jemalloc` runs, its internal parser interprets and applies these settings.

The results are usually printed to stderr/stdout, but this feature requires `jemalloc` to have been built with `--enable-stats`.

*Output Example*
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

I'll show an example where `malloc` is used internally, such as when `allocateDirect()` is employed for zero-copy based socket processing in a high-performance network server.

I'll use Apache Arrow, a tool that allows structured data to be stored and analyzed in off-heap memory in the Arrow format within the JVM. It directly allocates memory using `unsafe` or `malloc`, thus not affecting GC.

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

The Arrow format stores data in a CPU cache-friendly structure, arranging it in memory column-wise.

For example, typical row-based storage is:

```
[
  {"id": 1, "name": "케이홉"},
  {"id": 2, "name": "희망"},
  {"id": 3, "name": "가재"}
]
```

If it's in the above format, Arrow columnar is as follows:

```
id:   [1, 2, 3]
name: ["케이홉", "희망", "가재"]
```

The idea is to store fields in contiguous memory spaces, vectorize them, and then optimize performance through cache optimization. Since it's zero-copy transfer, it's fast, and column-wise contiguous access is said to leverage CPU locality for optimization.

It's used in high-performance application servers with null bitmaps and minimal metadata.

<br>

### Conclusion

In applications that directly manipulate native memory (e.g., bitmap-based communication), where memory manipulation is frequent, `jemalloc` is recommended for profiling and higher performance. This was just a quick taste; I'll explore what algorithms it uses, why it's better, its implementation details, and other concepts (e.g., arena, thread cache) in a future post.

Other alternatives include `tcmalloc`, `hoard`, and `ptmalloc2`.
