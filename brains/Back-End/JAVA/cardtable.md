# Card Table

카드 테이블은 Heap을 작은 Card 단위로 잘라서 기록해 둔 배열로

Write Barrier 중에서도 Generation GC의 핵심 구성 요소다.

각 카드는 보통 512~1024 바이트 정도의 힙 영역을 나타낸다.

jvm은 힙을 아래처럼 나눈다.
```css
[card0][card1][card2][card3] ... [cardN]
```

그리고 이 카드들의 상태를 저장하는 별도 배열이 바로 Card Table 이다.

Card Table은 다음처럼 생겼다.

```
byte cardTable[N];
```

카드 한개를 표시하는 용도로 byte를 하나씩 쓰는것이다.

Generation GC 에서는 young gc에서 young의 지역만 스캔하고 싶고 old gen 지역은 매번 스캔하고 싶지가 않다.

하지만 문제가 있는데 만약 old객체가 young을 참조하고 있다면?

```
oldObj -> youngObj
```

youngGC시 youngObj는 root, oldObj 둘 다 reachable하다.

하지만 young gc는 old gen의 전체를 스캔하지 않는다.

즉, oldObj의 필드를 훑지 않기 때문에 YoungObj를 놓쳐버릴 수도 있다. (old가 들고있는데 수집시켜버림)

해결: old 객체에서 young 객체로 연결되는 포인터를 반드시 알아야한다 이를 old -> young cross-generation refernece라고 한다.

문제는 old는 너무 크다. 그래서 그걸 전부 스캔하려면 gc 비용이 폭발해 heap의 작은 단위만으로 dirty로 표시햇거 빠르게 찾는 방식을 사용한다. 

이게 카드 테이블의 목적이다.

### 카드 테이블 하는일

카드 테이블 안에는 다음 정보를 기록한다

**이 카드 범위 안에서 old -> young 객체 참조가 변경되었다.**

참조가 변경되면 jvm은 write barrier를 통해 카드 테이블에서 해당 카드의 값을 Dirty 상태로 표시한다.

Dirty 값은 통상적으로 0/1 혹은 특정 byte값이다

참조 쓰기 발생시에 카드테이블은 아래처럼 업데이트 되는데

```ini
oldObj.child = youngObj;
```

이때 jvm write barrier는
1. oldObj가 속한 힙 주소 -> 해당 카드 번호 계산
2. cardTable[cardIndex] = DIRTY로 설정

카드 테이블의 pseudo-code는 아래다

```csharp
function write_barrier(obj, field, newValue):
    obj.field = newValue

    if obj is in OldGen AND newValue is in YoungGen:
        cardIndex = (obj.address - heapStart) / cardSize
        cardTable[cardIndex] = 1   # dirty
```

young gc가 실행될 때 카드 테이블이 어떻게 쓰이는가.

minor gc 루트 스캔 과정.
1. root scan(thread stack, global roots)
2. card table 스캔
   1. dirty marked 카드들만 읽음
   2. 그 카드 범위 안에서 old -> young인 참조가 있을 수 잇으므로 스캔
3. young 객체 마크
4. survivor/eden 작업

이렇게되면 old gen 전체를 검사하지 않고도 old -> young 참조를 정확히 추적 가능해진다.

이것은 young gc 성능애 핵심이다.

```
|----Card0----|----Card1----|----Card2----|----Card3----|
   [OldObjs]     [OldObjs]     [Young]       [Old]
```
[cardTable = [0, 1, 0, 0]]

여기서 card1이 dirty라면 young gc 또는 card1 범위만 스캔하고 나머지는 무시한다.

card0의 oldobj는 탐색안한다는 뜻. 올드를 안건듬

### vs SATB Write Barrier

| 구분    | SATB Barrier(G1/Concurrent GC) | Card Table Barrier(Gen GC) |
| ----- | ------------------------------ | -------------------------- |
| 목적    | 스냅샷(T0) 유지                     | Old → Young 참조 추적          |
| 기록 내용 | old reference                  | dirty card index           |
| 필요 GC | Concurrent marking             | Generational GC            |
| 범위    | marking 보호                     | remembered set 유지          |

요약하면
- 카드 테이블은 힙을 카드 단위로 나눈 후 각 카드 상태를 기록한 배열
- old gen -> young gen을 가르키는 참조가 생기면 더티로 표시함
- young gc시 dirty 카드만 스캔해서 old -> young 참조를 확인 가능
- 이로써 old gen 전체를 매번 스캔하지 않고 빠르게 young gc 가능
- 카드 테이블 업데이트는 write barrier 자동 처리

