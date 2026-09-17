# GC Write Barrier

Write Barrier는 JVM이 객체의 참조 필드가 변경될 때 항상 자동으로 실행되는 작은 코드조각이다.

```java
obj.field = newValue;
```

같은 참조쓰기가 발생할 때 jvm이 이 앞문장 앞뒤에 자동으로 삽입해놓은 추가 로그/검사 코드이다.

> GC가 내부 상태를 유지하기 위해, 참조 쓰기 시점마다 JVM이 자동으로 실행하는 보조 로직

개발자는 이 코드는 못보고 jvm 내부에 투명하게 실려있음

Concurrent GC의 핵심 문제때문에 필요한 개념인데 GC가 marking/evaluation 하는 동안 애플리케이션은 멈추지 않고 계속 참조를 바꾸기때문에(stw x) 이때 참조가 새 객체로 바뀌거나 null이 되거나 제거되거나 기존 객체 그래프가 순간적으로 흩어질 수 있다.

그리고 gc가 돌때는 이 변화를 놓치면 안된다. 놓치면 살아있는 객체를 죽여버리는 심각한 오류를 발생시킬 수 있기 때문이다. 즉, 참조가 바뀔 때, gc에게 중요한 변화가 일어났음을 알려주는 트리거가 된다.

```java
A.b = C
A.b = null
B.child = new Node()
array[i] = x
map.put(k, v)
list.remove(x)
```
위 코드 예시들이 바로 참조 쓰기가 발생하는 순간이고 jvm은 자동으로 코드 한조각을 또 실행하는데 이거 write barrier 이다.

### writer barrier가 하는일은 gc 종류마다 다르다.

대표적으로 다음 두가지 용도가 있는데

#### SATB Write Barrier

G1GC, Shenandoah 등, 스냅샷 유지용

필드가 변경될 때 old reference를 기록해서 스냅샷 TO를 복원할 수 있게 한다.

#### Card Table Write Barrier

Generational GC (Parallel, CMS) "new old 간 참조 업데이트 추적용"

필드가 변겨오딜 때 변경된 객체가 속한 카드를 dirty로 표시해 minor gc에서 old -> young 참조를 빠르게 찾아야한다.

### SATB 관점의 Write Barrier

SATB Write Barrier는 이렇게 동작한다.

참조가 변경되기 직전 oldValue를 SATBQueue에 넣어둔다. Pseudo-code:

```java
oldValue = obj.field;

if (GC가 marking 중 && oldValue != null) {
    SATBBuffer.add(oldValue);
}

obj.field = newValue;
```

이 oldVaue가 스냅샷(TO)를 복원하는데 필수 단서가 된다.

> Write Barrier는 JVM이 참조 필드가 변경되는 순간 실행하는 gc 보조용 가드 코드고 목적은 gc가 heap 변화를 놓치지 않도록 기록하고 추적하는데있다.

```
A → B → C
```

초기 그래프에서 gc 마킹실행중 애플리케이션에서 

```
A.b = null
```

이 되었다고 해보자 이 시점에 write barrier가 실행되며 oldValue = B를 GC에 기록한다 GC는 이 기록을 보고 G C를 마킹할 수 있으므로 스냅샷을 유지할 수 있다.