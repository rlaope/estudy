# Incremental Update

Incremental Update는 SATB와 반대되는 철학을 가진 알고리즘이다.

이 둘의 차이를 이해하는것이 Modern GC(G1, Z)랑 Legacy GC(CMS)를 가르는 분수령이 된다.

G1GC가 SATB(Snapshot At the Beginning gc트리거시점에 참조를 기준으로 수집)를 쓴다면

CMS는 Incremental Update 방식을 사용한다.

SATB는 과거 지향이고 마킹 시작 시점의 스냅샷을 만들어 중간에 변해도 신경쓰지 않는다

반대로 Incremental Update (CMS)는 미래지향으로 마킹 도중에 뭔가 바뀌면 어? 너 바뀌었네 그럼 나중에 다시 검사할게 하고 체크해둔다.

### Post-Write Barrier

SATB가 Pre-Write Barrier(바뀌기 전 값을 기록)를 쓴다면, Incremental Update는 Post-Write Barrier (바뀐후의 상태를 기록)를 사용한다.

#### Tri-color Marking

객체의 상태 3가지를 먼저 알아야한다.
1. **White**: 아직 검사 안함 (가비지 후보)
2. **Grey**: 자신은 검사했지만, 참조들은 아직 검사 안했을때 (진행중일때)
3. **Black**: 나도 검사했고, 참조들도 검사 끝남

여기서 문제상황은 gc가 Object A가 다 훑어서 Black으로 칠하고 나갔다고 치자.

그런데 갑자기 애플리케이션이 Object A가 Object C(white)를 참조하게 만든다면? (`A.child = C`)

그럼 gc는 A가 이미 black이니까 다시 안쳐다본다. 그래서 C는 누군가 자기를 참조함에도 불구하고 white로 남아서 삭제당할 위기에 처한다.

### Incremental Update 해결책: 다시 Grey로 강등시킨다.

위의 문제를 막기위해서 write-barrier가 동작하는데 

이미 검사가 끝난 black 객체에 새로운 참조가 연결되면, 그 객체를 다시 Grey로 바꿔서 나중에 다시 스캔하게 만든다.

```java
// Incremental Update Logic (Pseudo code)
void write_barrier(Object src, Object new_ref) {
    // src(이미 검사 끝난 객체)가 new_ref(새 객체)를 가리키게 됨
    if (is_black(src)) {
        // "잠깐, 너 Black이었는데 새 친구 생겼네? 다시 검사받아!"
        mark_grey(src); 
        // 혹은 Dirty Card에 기록해둠 (Remark 단계에서 다시 보려고)
    }
    src.field = new_ref; // 실제 할당
}
```

SATB는 Snapshot At The Beginning으로 시작할 때 살아있으면 끝까지 살려주고 Pre-Write Barrier로 암조 끝기전에 옛버전 주소를 기록해 수집하지 않게한다. 보수적이며 실제로 죽은 객체도 살려둔다. 대신 Remark 비용이 매우 짧다. 버퍼에 쌓인 옛날 주소만 찍으면 되니까. 대신 힙메모리 낭비가 된다. 대신 cpu사용률이 더 적어짐

Incremental Update는 참조가 바뀌면 다시 추적해서 정확히 살리는 철학이고, Post-Write Barrier로 새로 연결된 새참조 관계를 기록한다 더 정교하며 실제 살아있는 놈을 잘 골라낸다, 대신 Remark 비용이 더 높다. Grey로 강등된 Dirty 객체들의 자식을 다시 스캔해야하기 때문에 Remark 시간이 길다.

<br>

### G1GC가 SATB를 선택한 이유

Incremental Update 문제인데, Remark단계에 STW가 변경된 단계에서 객체 참조를 다시 스캔하도록 동작한다.

그런데 마킹 도중에 애플리케이션이 객체 참조를 엄청나게 많이 바꾸었다면> 다시 검사해야할 Dirty Card의 양이 많아진다.

결국 Remark 단계에서 STW가 얼마나 길어질지 **예측이 불가능해진다.** (CMS가 망한이유중 하나기도함)

SATB는 Remark 단게에서 해야할 일은 SATB 버퍼에 기록된 끊어진 참조들만 후다닥 마킹하고 끝낸다.

다시 깊이 우선 탐색을 할 필요가 없다. 

Floating Garbage가 생겨서 메모리는 좀 낭비하지만 STW 시간은 확실하게 짧게 보장이 가능하다.