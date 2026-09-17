# Parallel GC to G1GC

Our core banking system application is based on OpenJDK 8. Although an upgrade is necessary, the legacy system is so extensive that we are still running on JDK 8.

We are currently using Parallel GC, but I'd like to upgrade to JDK 17 and implement G1GC, especially for client-side applications (e.g., loan application/investment platforms).

Now, you might be wondering: why? Why do I want to switch?

And you might counter that G1GC can also be used with OpenJDK 8. I will explain, one by one, why we are still using Parallel GC and why I want to make this change.

### OpenJDK 8 G1GC

First off, there's a world of difference between G1GC in JDK 8 and G1GC in JDK 11, especially in terms of performance and stability.

There's a good reason why Parallel GC is the default in JDK 8. G1GC isn't well-optimized in this version. Its performance degrades drastically, not just in normal operation, but especially during incidents like a Full GC.

When a Full GC is triggered in G1GC due to an Allocation Failure, G1GC essentially 'surrenders' and performs a Full GC. While a Full GC might occur, JDK 8 handles it using **Serial GC (single-threaded)**.

Cleaning an 8GB heap by itself is incredibly slow, and the same 'hang' issues (problems that occur with Parallel GC) can still happen even when using G1GC.

This issue was improved in JDK 10 and later versions, allowing Full GC to run in parallel.

In addition,

1.  Memory Efficiency: G1GC in JDK 8 uses inefficient data structures for managing metadata, consuming more memory than Parallel GC.
    1.  (I haven't looked into the specific data structures, but let's just note that internal structure improvements were made.)
2.  String Deduplication: While present in JDK 8, it's further optimized in JDK 11, leading to more efficient heap memory usage.
3.  Docker Container Awareness: Early versions of JDK 8 couldn't recognize container memory limits, making them vulnerable to OOMKiller (this was patched later).

Therefore, we did not adopt JDK 8 G1GC due to its weakness of extremely long STW (Stop-The-World) times when Full GC occurs, as it operates in a single thread.

Why? Because the very problem I intended to solve by adopting G1GC in our current system was this Full GC 'hang'.

## Solved Problem

Our loan/investment system is a service that experiences relatively high traffic, as customers come to apply for loans and invest in bonds.

The P95 was around 1.2s, which isn't particularly fast, but considering the many calls that process heavy accounting data, it maintained a reasonably acceptable figure.

However, there were cases where the P99 climbed to 3-5s. This phenomenon originated from Parallel GC's Full GC. If we consider 500,000 users, 5,000 users could become frustrated and churn, meaning we would lose approximately 5,000 investment and loan customers. (In reality, one investment customer uses our app for at least a year, and a personal credit loan applicant for at least five years, excluding early repayments and considering normal cases.)

To summarize, when processing 500,000 traffic requests over 2 hours, the average response time was a respectable 500ms, or up to 1s. However, the P99 was spiking.

And as the heap size grew, the all-or-nothing approach of Parallel GC inevitably led to longer STW times.

### Long Tail

This is more accurately described as a **Long Tail phenomenon** rather than a 'hang,' because due to the nature of Parallel GC's structure, a Full GC that occurs when the Old Generation is full results in a long STW that pauses the application.

Parallel GC scans and empties the entire heap memory, so the pause time increases proportionally with the heap size.

While its 'batch cleaning' approach offers good throughput, response times are bound to be spiky.

In contrast, G1GC manages the entire heap by dividing it into small regions.

Instead of stopping the entire application at once, it prioritizes and incrementally cleans only the regions with a high concentration of garbage. This might slightly reduce overall throughput, but it allowed us to break down and distribute the 2-second spikes in pause times into smaller, sub-0.5-second pauses.

Because G1GC manages the heap in small, region-based units, it can adjust STW times to meet my configured target pause time. This allows us to secure P99.

## G1GC

The reason I want to use G1GC is to lower P99 by introducing a GC that offers predictable pause times and can resolve the long tail phenomenon caused by Full GC.

Of course, the average processing time might increase from 500ms to 520ms. This is because G1GC employs a technique called a write barrier, which records where data has changed each time it's modified, to facilitate faster cleaning later.

There's a slight delay each time code executes because G1GC performs a 'stealthy' recording operation whenever we assign values to variables or change references.

G1GC cleans based on an object reference graph taken at a specific point in time, known as SATB (Snapshot At the Beginning). This is done to reduce CPU throughput and achieve faster STW. While it might lead to dead objects persisting for a while, they can be quickly collected during the next cycle, representing a trade-off where reducing STW was prioritized.

In any case, sacrificing 20ms in average response time won't make customers leave. On the contrary, this change allowed us to maintain P99 within the 1-second range and potentially retain an additional 5,000 customers.

<br>

## Performance Comparison

There was a server system where calls typically took around 500ms, using either G1GC or Parallel GC. I had set up a server for arbitrary testing, not tied to core banking, and I decided to use this server in our development environment to conduct a comparative test between G1GC and Parallel GC.

The test was conducted with a Heap Size of 4GB, sending 100 RPS requests for a duration of 5 minutes, aiming to induce Full GC by filling up the old generation.

For each request, 1MB to 5MB of dummy data is created and then discarded. This rapidly fills the Eden space, causing objects to move from Survivor to Old Gen, thereby putting pressure on GC.

```java
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RestController;
import java.util.ArrayList;
import java.util.List;
import java.util.Random;

@RestController
public class GcTestController {

    @GetMapping("/api/load-test")
    public String heavyProcess() throws InterruptedException {
        // [1] 메모리 부하 생성 (Garbage Generation)
        // 요청마다 약 1MB 정도의 객체를 생성하여 Heap에 압력을 줌
        List<byte[]> garbage = new ArrayList<>();
        for (int i = 0; i < 10; i++) {
            garbage.add(new byte[100 * 1024]); // 100KB * 10 = 1MB
        }

        // [2] 비즈니스 로직 시뮬레이션 (CPU 연산 + I/O 대기)
        // 단순히 sleep만 하면 GC가 돌 틈이 생기므로, 약간의 연산도 섞어주면 더 리얼함
        simulateBusinessLogic();

        return "processed";
    }

    private void simulateBusinessLogic() throws InterruptedException {
        long start = System.currentTimeMillis();
        // 500ms 동안 대기 (외부 API 호출이나 DB 조회라고 가정)
        Thread.sleep(500); 
    }
}
```

Then, each is launched on a different port.

```
java -Xms4g -Xmx4g -XX:+UseParallelGC -jar simulator.jar

java -Xms4g -Xmx4g -XX:+UseG1GC -XX:MaxGCPauseMillis=200 -jar simulator.jar
```

As per the scenario mentioned earlier, a k6 script is also prepared.

```js 
import http from 'k6/http';
import { check, sleep } from 'k6';

export const options = {
  // 부하 시나리오 설정
  scenarios: {
    constant_load: {
      executor: 'constant-arrival-rate', // 초당 일정 요청 수를 유지
      rate: 100,      // 초당 100 RPS
      timeUnit: '1s',
      duration: '5m', // 5분간 지속 (Full GC 유도)
      preAllocatedVUs: 100, // 가상 유저 미리 확보
      maxVUs: 200,
    },
  },
  thresholds: {
    http_req_duration: ['p(99)<2000'], // P99가 2초 넘으면 실패로 간주
  },
};

export default function () {
  const res = http.get('http://localhost:8080/api/load-test');
  
  check(res, {
    'status is 200': (r) => r.status === 200,
  });
}
```

```
k6 run test.js
```

#### Parallel GC Results

A key characteristic is that it performs well initially, but a long pause occurs the moment the Old Gen fills up.

```
http_req_duration..............: avg=505.32ms  min=501.12ms  med=504.22ms
                                   p(90)=515.45ms  p(95)=550.12ms
                                   p(99)=2850.42ms  <-- !!! (2.8초)
                                   max=4200.15ms    <-- !!! (4.2초)
```

The average is respectable, close to 500ms, but P99 and Max spike to 2-4 seconds. This is because the server was paused during the Full GC.

#### G1GC Results

A key characteristic is that while the average response time might be slightly slower, there are no extreme pauses.

```
http_req_duration..............: avg=512.45ms  min=502.10ms  med=510.33ms
                                   p(90)=540.12ms  p(95)=580.44ms
                                   p(99)=850.12ms   <-- (0.8초, 안정적)
                                   max=1100.23ms    <-- (1.1초)
```

The average is about 7ms slower than Parallel GC (due to CPU overhead), but P99 is very stable, staying under 1 second.

<br>

### Conclusion

In conclusion, my answer to why I want to use G1GC is that I aimed to lower P99. OpenJDK 8 was unsuitable for this goal due to its serial Full GC, among other unoptimized aspects. When testing with identical code and a 500ms delay logic on G1GC in JDK 11+, Parallel GC showed good average throughput but experienced Long Tail pauses of up to 4 seconds. In contrast, G1GC's average response time increased by 1-2%, but it defended P99 latency to around 0.8 seconds, ensuring consistent user experience.

As a result, we sacrificed about 1% in average response time but gained a P99 improvement of 2-3 seconds.
