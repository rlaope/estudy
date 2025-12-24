# Mixed GC

Mixed GC는 Young GC에 Old Region 청소 작업을 얹혀서 수행하는 것으로 Old GC랑 같다고 생각할 수 있으나 다르다.

Young Generation 영역 Eden + Survivor 전체 + 가비지가 많은 일부 old region을 동시에 수집하는 단계다.

Mixed GC는 갑자기 발생하는 것이 아니라 Concurrent Marking Cycle이 선행되어야만 실행된다.

1. IHOP(Initating Heap Occupancy Percent) 달성 (Trigger): Heap 사용량이 특정 임계치 기본 45%를 넘으면 Concurrent Marking이 시작된다
2. Concurrent Marking: 애플리케이션 실행중에 백그라운드 스레드가 **어떤 old region에 가비지가 많은지 파악**한다. 이때 살아있는 객체 비율이 낮은 region을 식별해둔다.
3. Remark & Cleanup: 마킹을 확정하고, 텅 빈 region은 즉시 회수한다.
4. Mixed GC 시작: 이제 G1GC는 YoungGC를 수행할때마다 앞서 파악해둔 가비지 많은 Old Region(Candidate Set)을 조금씩 끼워넣어 같이 청소한다.

이 과정을 통해 Full GC(STW가 긴작업)을 하지 않고도 old 영역 메모리를 확보할 수 있다.

### 작동 방식

Mixed가 매력적인 이유는 한 번에 다 치우지 않는다는 점이다.

**점진적  수거 (Incremental Collection)**로 G1GC는 목표 정지 시간 `MaxGCPauseMillis`를 지키기 위해 회수해야할 old region 후보군(Candidate Set)을 한번에 gc에 몰아서 처리하지 않는다.
- 회수할 old region이 100개라면, 이를 8번(기본값)의 mixed gc에 나누어 처리한다.
- 1차 MixedGC: Young Gen + Old Region 12개
- 2차 MixedGC: Young Gen + Old Region 13개
- ... 반복

이렇게 나누어 처리하기 때문에 Stop the world 시간을 짧게 유지하고싶다.

### Mixed GC 관련 핵심 튜닝 포인트

Mixed GC의 효율을 높이고 Full GC를 방지하기 위한 핵심 파라미터 들이다.

#### 언제 시작할 것인지 Trigger

`-XX:InitiatingHeapOccupancyPercent` 기본값 45로 전체 힙 메모리 45퍼센트가 차면 Concurrent Marking을 시작한다.

메모리가 빠르게 차는 애플리케이션: 값을 줄여야한다 40정도로 미리미리 마킹을 시작해야 Full GC가 발생하기전에 Mixed GC로 힙을 비울수있기 때문이다.

너무 낮추면 불필요한 마킹 작업과 Mixed GC가 너무 자주일어나 CPU를 낭비하게 된다.

#### 얼마나 효율적인 놈만 골라낼 것인가 (Efficiency)

`-XX:G1MixedGCLiveThresholdPercent` (기본값 85)
- Old Region 내에 살아있는 객체가 이 퍼센트 이상이면 그 Region은 Mixed GC 대상에서 제외한다.
- 왜냐면 살아있는 객체가 너무 많으면 다른곳으로 evacuation 하는 비용이 비싸고, 회수할 메모리는 적기 때문이다.
- 값을 낮추면 정말 가비지가 많은 효율적인 Region만 골라서 청소하므로 GC 시간은 줄어들지만, old gen 확보 속도는 느려질 수 있다.

#### 몇 번에 나눠서 할 것인가? (Latencyu)

`-XX:G1MixedGCCountTarget` (기본값 8)
- Concurrent Marking 이후 찾아낸 가비지 Region들을 몇 번에 Mixed GC에 나누어 처리할지 목표를 정한다.
- 값을 늘리면 한 번의 Mixed GC때 처리량이 줄어들어 Pause Time이 짧아지지만 Old Gen 정리가 완료될 때 까지 시간이 더 걸린다.

#### 언제 멈출 것인가? (Termination)

`-XX:G1HeapWastePercent` (기본값 5)
- 전체 힙의 5퍼센트 정도는 가비지로 낭비되어도 괜찮다는 뜻이다
- MixedGC를 반복하다가, 회수 가능한 가비지 양이 이 값 미만으로 떨어지면 MixedGC를 중단하고 Young 모드로 돌아간다
- 값을 높이면 Mixed GC를 조기에 종료하여 불필요한 연산을 줄일수있다. 약간의 메모리 낭비를 감수하고 성능을 얻음

### 튜닝 권장

Full GC가 가끔  발생하면 `InitatingHeapOccupancyPercent`를 낮춰서 마킹을 더 일찍 시작하게 하라.

Mixed GC시간이 너무 길다면 `G1MixedGCCountTarget`을 늘려 작업을 더 잘개 쪼개거나 `G1MixedGCLiveThresholdPercent`를 낮춰서 가성비 좋은 region만 청소하게 하라

CPU 사용량이 너무 높다면 Mixed GC가 너무 자주 일어나는지 확인하고 `InitiatingHeapOccupancyPercent`를 높여보는것도 좋다.

### vs YoungGC

YoungGC는 막 태어난 객체들이라 누가 오래 살지 모른다, 무조건 다 살리려고 copy 시도를 한다. 그러다가 evacuation failure가 발생하게 되면 mixed gc에게 맞기도록 일단 눌러앉게 냅두고 old로 승격시켜버린다.

mixed gc 시점에서는 다시 이걸 처리하려고 하는데, evacuation failure 발생한 그영역을 시간이 좀 지나면 눌러 앉았던 객체들 중 상당수가 그 사이에 죽어서 garbage가 되어있다.(evacuation failure가 발생한 old로 승격된 리전에서 100개의 객체를 못 copy했었는데 그중 90개정도가 죽은상황이 되었다고 치자) g1은 이제 살아남은 소수만 옮기면 되므로 비용이 훨씬 적게 이걸 처리할수있으므로 좋다. 미래엔 디지것지 라는 생각으로 young이 실패해도 mixed가 처리하게 넘기는 느낌이다. 토스를

YoungGC는 일단 Eden에 있는것은 다 옮겨!처럼 동작하게 되고 MixedGC Concurrent Marking이라는 과정에서는 region별로 성적표를 매긴다 regionA는 생존률 90% 가비지 10%, B는 생존률 10% 가비지 90% 어 그러면 90퍼를 확보할 수 있는 B를 먼저 수집해야겠다. 이렇게 판단할 수 있다.

즉, Mixed GC가 더 효율적인 이유는 쓰레기가 많은 곳만 골라서 garbage first 청소하기 때문이다.

아까 눌러앉았던 곳은 정리가 안된 상태라 쓰레기 비율이 높을 확률이 크고 그래서 mixed gc의 최우선 타겟(prime targe)이 되어 아주 효율적으로 회수된다.

> 참고: mixed gc가 ihop 기준으로 트리거 된다면, young gc는 그냥 무조건 eden이 꽉 찼을때 allocation pressure가 발생될때 트리거 된다. concurrent marking에 cleanup이 하는일은 mixed gc 시작 결정이므로 eden이 꽉차있으면 mixed gc를 넣고 eden + 성적좋은 old region을 몇개 같이 청소시킨다.
>
> 당연한 이야기겟지만 young gc가 더 빠르다 그 이유는 eden만 청소하기 때문에, old까지 청소하는 mixed보다는 범위가 좁아 빠르다

$$Time(Mixed) = Time(Young) + Time(Old\_Evacuation)$$