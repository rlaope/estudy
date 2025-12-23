# Parallel GC to G1GC

우리 코어 뱅킹 시스템 애플리케이션은 openjdk8 기반이고 업그레이드는 해야하지만 레거시 시스템이 너무 방대하여 아직까지 jdk8로 살고있다.

parallel gc로 쓰고있지만 17로 업그레이드하고 g1gc를 달아두고싶다. 클라이언트 사이드 애플리케이션에서 말이다(예를 들면 대출신청/투자 플랫폼같은?)

자, 그럼 여기서 의문점이 들것이다. 왜? 왜 바꾸고싶은데요

그리고 openjdk8에서도 g1gc 쓸수있는데여. 이런 반박이 나올수있는데, 그대로 parallel을 쓰고있는 이유와 왜 바꾸고싶은지 하나하나씩 알아보겠다.

### OpenJDK 8 G1GC

일단 8의 g1gc랑 11의 g1gc는 하늘과 땅차이인데, 성능과 안정성면에서 ㅇㅇ 

jdk8의 기본이 parallel gc인것도 이유가 다 있다. 일단 g1gc 최적화가 잘 되어있지 않다. 평소 성능보다도 사고가 났을때 즉 full gc가 났을때 8 g1gc는 급격하게 똥이된다.

G1GC는 FullGC가 발동하면 Allocation Failure가 발생하게 되면, g1gc는 항복을 선언하고 full gc를 돌리는데, 뭐 full gc까지는 날수있다고 쳐도 jdk8에서는 이를 **Serial GC(단일 스레드)**로 처리하게 된다.

8GB의 힙을 청소하는데 혼자서 하니까 엄청나게 느리다, g1gc를 써도 Hang같은 현상(Parallel GC에서 발생하는 문제)가 똑같이 발생하기 때문이다.

JDK 10이후에서는 이 문제가 개선되어 Full GC상황에도 병렬로 돌게된다.

그 밖에도 

1. 메모리 효율: jdk 8의 g1은 메타데이터등을 관리하는 자료구조가 비효율적이라 parallel gc보다 메모리를 더 많이 잡아먹는다
   1. 자료구조가 뭔지는 안알아봤지만 일단 내부구조 개선까지만 알아두자
2. String Deduplication(문자열 중복 제거): jdk8에도 있지만 JDK 11에서 더욱 최적화되어 힙메모리를 알뜰하게 쓴다
3. Docker Container 인식: jdk8초기버전은 컨테이너의 메모리 제한을 인식 못해서 OOMKiller에 취약했다 (나중에 패치됨)

그래서 JDK8 G1GC는 FullGC 발생시 단일 스레드로 동작하는 stw 시간이 극단적으로 길어지는 약점이 있어서 채택하지 않았다.

왜냐고? 애초에 내가 생각하는 현재 우리 시스템에서 G1GC를 채택했을때 해결하려는 문제가 바로 이 FullGC Hang이였기 때문이다.

## Solved Problem

우리 대출/투자 시스템은 비교적 트래픽이 많이 몰리는 서비스다. 대출 신청을 하러 들어오는 고객과, 채권 투자를 하러 들어오는 고객들이 있기 때문이다.

P95는 1.2s 정도로 그렇게 빠르진 않지만, 무거운 회계 데이터들을 처리하는 콜들이 많은것을 감안하면 어느정도 괜찮은 수치를 유지하고 있었다.

그러나 P99가 3~5s까지 올라가게 되는경우가 있었다. 그리고 이 현상인 Parallel GC의 Full GC로 부터 생겨나게 된 것이였고 50만명의 유저라 쳤을대 5,000명의 유저가 답답해서 이탈해버릴 수 있는거였고 5000명 정도의 투자고객과 대출고객을 놓쳐버린다. (실제로 한명의 투자 고객은 최소 1년 이상의 우리 앱을 사용하게 되기 때문에 또 개인신용 대출자면 최소 5년이다. 중도상환을 제외하고 정상적인 케이스) 

다시 정리하면 2시간동안 50만건의 트래픽을 처리할때 평균 응답속도는 500ms 길면 1s로 준수했다. 그러나 P99가 치솟는거였다.

그리고 힙사이즈가 커지면 커질수록 all or nothing 방식의 parallel gc의 동작은 stw를 더 길게 만들수밖에 없었다.

### Long Tail

이는 Hang이라기 보다는 **Long Tail 현상**으로 부르는게 더 맞는데 Parallel GC 구조 특성상, Old Generation이 꽉 찼을때 발생하는 Full GC가 애플리케이션을 멈추는 stw가 길기 때문이다.

Paralell GC는 힙 메모리 전체를 스캔하고 비우기 때문에, 힙 사이즈가 커질수록 멈춤 시간에 비례해서 늘어난다.

몰아서 청소하기 방식이라 처리량은 좋지만, 응답속도는 튈수밖에 없다. 

반면 G1GC는 전체 힙을 작은 region 단위로 쪼개서 관리한다.

전체를 한 번에 멈추는게 아니라, 가비지가 많이 찬 영역만 우선적으로, 그리고 점진적으로 청소한다. 이로 인해 절대적인 처리량은 약간 줄어들 수 있어도 2초씩 튀던 멈춤 현상을 0.5초 이내로 잘개 쪼개서 분산시킬 수 있었다.

g1gc는 region 기반으로 잘개 쪼개서 관리하기 때문에 내가 설정한 목표시간 pause time에 맞춰서 stw를 조절할수있어. P99까지 확보할 수 있게된다.

## G1GC

pause time을 예측가능하며 full gc때문에 발생하는 long tail 현상을 해결할 수 있는 g1gc를 도입해 p99를 낮춘다는 이유에서 g1gc를 쓰고싶은것이다.

물론 평균 처리시간은 500ms -> 520ms로 줄어들수있다 이는 g1gc가 나중에 빨리 청소하기 위해 데이터가 변경될 때마다 어디가 바뀌었는지 기록해두는 기술이 있는데 이를 write barrier라고 한다.

코드를 실행할때마다 미세한 딜레이가 생기는데 g1gc가 우리들이 변수에 값을 대입할때, 참조를 바꾸거나 할때 g1gc가 몰래몰래 기록을 해두는 연산이 있기 때문이다.

g1gc는 SATB라고 Snapshot At the Beginning 이라는 시점을 기준의 객체 참조 그래프를 기준으로 청소를 한다. 이는 cpu 처리량을 낮추고 빠른 stw를 위해서다. 물론 죽은 객체가 계속 살아있는 문제가 있을수있지만 다음 처리때 빠르게 수집이 가능하니 트레이드오프중에서 stw를 줄이는  선택을 한것이다.

어찌되었건 평균 응답속도 20ms를 희생한다고 고객이 떠나지는 않는다. 오히려 이번걸로 P99도 1초때로 유지를 할 수 있었으며 5000명의 고객을 더 확보할 수 있었다.

<br>

## 성능 비교

g1gc와 parallel gc를 통해 평균 500ms 정도 걸리는 콜의 서버 시스템이 존재했다. 코어 뱅킹과 엮이지 않은 내가 임의로 테스트할때 만들어둔 서버가 존재했는데 이 서버를 개발계에 두고 g1gc, parallel gc 비교 테스트를 진행해봤다.

Heap Size는 4GB로 진행하고 100RPS요청을 Duration 5m동안 진행한다. old gen이 꽉차는 full gc 유발을 유도한다.

요청당 1mb~5mb 더미를 생성했다가 버리고, 이로 인해 eden은 빠르게 차고 survivor -> old gen으로 객체가 이동해 gc압박을 준다.

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

그리고 각각 다른 포트로 띄워둔다.

```
java -Xms4g -Xmx4g -XX:+UseParallelGC -jar simulator.jar

java -Xms4g -Xmx4g -XX:+UseG1GC -XX:MaxGCPauseMillis=200 -jar simulator.jar
```

아까 말한 시나리오데로 k6 스크립트도 준비해둔다.

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

#### Parallel GC 결과

특징으로는 초반에 잘 버티다가 Old Gen이 꽉차는 순간 멈춤이 길게 발생한다.

```
http_req_duration..............: avg=505.32ms  min=501.12ms  med=504.22ms
                                   p(90)=515.45ms  p(95)=550.12ms
                                   p(99)=2850.42ms  <-- !!! (2.8초)
                                   max=4200.15ms    <-- !!! (4.2초)
```

평균 500ms 근처로 준수하지만, P99와 Max가 2~4초까지 튄다. Full GC가 도는 동안 서버가 멈춰있었기 때문이다.

#### G1GC 결과

특징으로는 평균 응답속도는 미세하게 느릴 수 있으나, 극단적인 멈춤은 없다.

```
http_req_duration..............: avg=512.45ms  min=502.10ms  med=510.33ms
                                   p(90)=540.12ms  p(95)=580.44ms
                                   p(99)=850.12ms   <-- (0.8초, 안정적)
                                   max=1100.23ms    <-- (1.1초)
```

평균 avg는 parallel보다 7ms정도 느리지만(cpu overhead), P99가 1초 미만으로 매우 안정적이다.

<br>

### 결론

결론적으로 내가 g1gc를 쓰고싶다에 대한 답으로는, P99를 낮추고 싶다는 목적이 있었으며 openjdk8은 serial full gc때문에 목적에 맞지 않았고. 다른 최적화 안된것들도 +며, 11이상의 g1gc에서 동일한 코드와 500ms로 지연로직을 테스트 했을때 paralle gc가 평균적인 처리량은 좋았지만 최대 4초까지 멈추는 LongTail이 발생했고, 반면 G1GC는 평균 응답속도가 1~2%느려졌지만 P99 지연시간을 0.8초대로 방어하여 사용자 경험의 일관성을 확보했다.

결과적으로 1%정도의 평균응답시간을 내주고 P99 -2~3s정도를얻은것이다.