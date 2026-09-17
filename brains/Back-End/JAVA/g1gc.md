# G1GC

G1 GC에 대해서 자세히 알아보도록 하겠다.

G1GC는 대용량 메모리가 있는 다중 프로세서 시스템을 대상으로 하는 서버-스타일 가비지 컬렉터다.

G1GC는 서버의 프로세스 메모리의 사이즈가 점점 커짐에 따라 개선된 GC다.

기존의 GC가 Stop the world를 극단적으로 가져가지 않기 위해 구현되었다면, G1GC는 FullGC를 최대한 피하기 위해 Stop the world(PauseTime)를 짧게 가져가기 때문에 실시간 성능을 구현하였다.

- 높은 처리량과 낮은 Stop the world 지향
- CMS의 개선안으로 계획되었다.
- 스레드 비중이 높은 heap region을 집중적으로 수집한다.
- 자바 9부터 디폴트로 설정되어 있으며 `-XX:+UseG1GC`옵션을 사용해 수동으로 활성화 시킬 수 있다.

G1은 회수가 가능한 영역, 즉 쓰레기가 많을 것으로 예상되는 영역에 대한 수집 및 압축이 주가 된다.
이것이 Garbage First, G1으로 명명된 이유다.


|  용어 | 내용  |
|---|---|
|Evacuation|G1GC에서 일어나는 객체의 Copy 및 Moving을 뜻한다.|
|Region|G1GC에서 관리하는 힙메모리 영역을 고정된 크기로 나눈 것|
|Humongous Region|새로 할당하는 인스턴스가 리전 하나의 메모리의 반절이 넘는경우 Humongous Region이라 칭하고 G1GC에서 별도로 관리하는 영역이 된다.|
|Available/Unused Region|아무것도 할당되어있지 않은 영역, Evacuating때 이주 대상이 된다.|
|CollectionSet (CSet)|GC가 수행될 Region 집합 (타겟)CSet 내 데이터는 GC 동안 모두 비워진다. (복사 혹은 이동됨)Region 집합은 Eden, Survivor, Old Generation으로 이뤄질 수 있다.CSet이 JVM에서 차지하는 비율은 1% 이내이다.|
|Remembered Set (Rset)|Reference를 가진 객체가 어느 Region에 있는지 기억하기 위해 사용하는 자료구조각 Region 당 하나의 RSet이 존재하며 이를 통해 Region의 병렬 및 독립된 수집을 가능하게 한다.단일 Old generation Region을 피하기 위해 cross-region references 위치를 추적하는 것 = 한 Region에서 다른 Region을 참조한다.RSet이 차지하는 비율은 5% 이내이다.|
|MixedCollection|Young/Old 영역의 GC가 일어나는것을 Mixed GC 또는 MixedCollection이라 한다.|


<br>

### G1 Heap Allocation

![](https://wwz-frontend-asset.s3.ap-northeast-2.amazonaws.com/techblog/sandbox/aW1hZ2U%3D%287%29.png)

이전 GC들과 다르게 Heap 영역에 고정된 메모리 크기로 각 Generation을 구분하지 않고, 힙 영역을 동일한 크기의 Region으로 나누어 관리하는 것을 볼 수 있다.

`-XX:G1HeapRegionSize`: JVM 힙은 2048개의 region으로 나눌 수 있으며, 해당 옵션을 통해 1MB ~ 32MB 사이로 지정될 수 있다.

새롭게 정의된 Humongous, Available/Unused 영역이 존재한다.

G1은 힙 내의 하나 이상의 Region을 단일 Region으로 객체를 복사하는데 이 때 메모리를 압축/해제 시킨다.

다중 프로세서에서 병렬 작동을 통해 STW 시간은 줄이고 처리량은 증가시킨다.

<br>

### 동작 방식

G1GC의 객체 수집 동작 방식은 다음과 같다.

![](https://wwz-frontend-asset.s3.ap-northeast-2.amazonaws.com/techblog/sandbox/aW1hZ2U%3D%288%29.png)

FullGC가 수행될 때 Initial Mark - Root Region Scan - Concurrent Mark - Remark - Clean up - Copy 순으로 단계를 거치게 되며, STW 시간을 줄이기 위해 각 스레드가 자신만의 region을 잡고 작업하는 방식의 병렬 GC를 수행한다.

1. Initial Mark: Old Region에 존재하는 객체들이 참조하는 Survivor Region을 찾는다. 이 과정에서는 STW가 발생한다.
2. Root Region Scan: Initial Mark에서 찾은 Survivor Region에 대한 GC 대상 객체 스캔 작업을 진행한다.
3. Concurrent Mark: 전체 힙의 Region에 대해 스캔 작업을 진행하며, GC 대상 객체가 발견되지 않은 Region은 이후 단계를 처리하는데 제외되도록 한다.
4. Remark: STW를 발생시키고 최종적으로 GC에서 제외될 객체(살아남을 객체를 식별한다.)
5. Clean up: STW를 발생시키고 살아있는 객체가 가장 적은 Region에 대한 미사용 객체 제거를 수행한다. 이후 STW를 끝내고 앞선 GC 과정에서 완전히 비워진 Region을 Freelist에 추가해 재사용될 수 있게 한다.
6. Copy: GC 대상 Region이였지만 Cleanup 과저엥서 완전히 비워지지 않은 Region의 살아남은 객체들을 새로운(Available/Unused) Region에 복사하여 Compaction 작업을 수행한다.

> G1GC 마킹
> SATB(Snapshot-At-The-Beginning) 알고리즘을 통해 마킹 작업을 한다. 일시정지가 아닌 일어난 시점 직후의 라이브 객체(스냅샷)에만 마킹을 하므로, 마킹 도중 죽은 객체도 라이브 객체로 간주된다. Remark 단계의 응답 시간이 다른 GC에 비해 더 빠른 경향이 있다.

### GC Cycle

![](https://wwz-frontend-asset.s3.ap-northeast-2.amazonaws.com/techblog/sandbox/aW1hZ2U%3D%289%29.png)

관점에 따라서 2, 3개의 페이즈로 구성된 GC로 young/old 영역의 메모리를 관리한다.

Young only Phase에서는 일반적인 GC에서 Young 영역의 Generation이 이루어지는 곳 (Old 승격까지 이루어진다.)이다.

Young only Phase with Initial Mark: YoungGC와 동시에 일어나는 Concurrent Marking Cycle을 수행하는 페이즈. 이 페이즈를 진입하기 위해서는 특정 조건을 만족해야한다.

MixedCollection은 특정 조건을 만족할 때 까지 수행한다.

- G1HeapWastePercent
    - `The allowed unreclaimed space in the collection set candidates as a percentage. G1 stops the space-reclamation phase if the free space in the collection set candidates is lower than that.`
- G1MixedGCLiveThresholdPercent
    - `라이브 객체 점유율이 이 값보다 높은 old gen은 space-reclamation 단계에서 수집되지 않는다.`
- G1MixedGCCountTarget
    - `The expected length of the space-reclamation phase in a number of collections.`

