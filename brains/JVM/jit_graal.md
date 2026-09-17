# JIT 컴파일러, GraalVM 자바코드 기계어 번역 과정

## `MethodData`의 구조와 역할 (C1 -> C2 프로파일링 데이터 전송)

과거 Java7 이전 PermGen 시절에는 프로파일링 데이터를 담는 구조체가 Java Heap/PermGen에 존재하는 객체(Oop, Ordinary Object Pointer)였기에 `MethodDataOop`라고 불렸다.

하지만 Java8 Metaspace 도입 이후에 이 구조체는 더이상 Oop가 아닌 네이티브 메모리에 할당되는 `MethodData` 클래스(Cpp레벨)로 변경이 되었다.

최신 핫스팟 vm관점에서 MethodData의 역할과 구조는 다음과 같다.

- **역할**: Tiered Compilation 환경에서 인터프리(Tier 0)와 프로파일링이 활성화된 C1 컴파일러(Tier 2, 3)가 수집한 런타임 실행 통계(프로파일링된 데이터)를 저장한다. C2 컴파일러(Tier 4)는 이 데이터를 읽어들여 공격적이고 낙관적인 최적화(optimistic optimization)을 수행한다.
- **내부 구조** (DataLayout): MethodData는 단순한 카운터 집합이 아니라, 바이트코드 인덱스 BCI(Bytecode Index)에 매핑되는 가변 길이의 데이터 스트림이다. 내부적으로 DataLayout이라는 단위로 구성된다.
  - `ReceiverTypeData`: `invokevirtual`이나 `invokeinterface` 호출시 실제로 어떤 클래스 타입이 수신자(receiver)로 들어왔는지 기록한다. (Monomoriphic/Bimorphic Call 판별에 핵심)
  - `BranchData`: if문 등 분기문에서 어느 방향 taken,not taken으로 실행 흐름이 넘어갔는지 확률을 기록한다
  - `CounterData`: 특정 메서드나 루프가 몇 번 실행되었는지 추적한다.
- **협력 메커니즘**: c1은 컴파일 시점에 소스코드 사이사이에 `MethodData`를 업데이트하는 네이티브 인스트럭션(profiling stubs)를 삽입한다. 이후 c2가 해당 메서드를 컴파일할 때 이 MethodData를 읽어 이 분기문은 한 번도 실행된적이 없구나. 이 인터페이스는 항상 string 객체만 구현체로 들어는구나 등을 가정을 세우고 코드를 단순화한다.

<br>

## C2 Compiler 탈출 분석 (Escape Analysis)와 스칼라 치환(Scalar Replacement)

C2는 Sea of Nodes라는 형태의 Ideal Graph(IR)을 사용한다.

탈출 분석과 스칼라 치환은 이 IR을 변환하는 강력한 최적화 과정이다

```java
class Point {
    int x, y;
    Point(int x, int y) { this.x = x; this.y = y; }
}

public int calculateArea() {
    Point p = new Point(10, 20); // 힙 할당 발생?
    return p.x * p.y;
}
```

내부적으로 이 코드가 C2의 IR 수준에서 어떻게 변환되는지 단계별로 살펴보자

1. **초기 IR을 먼저 생성한다**. 처음에 바이트코드를 파싱하면 `Allocate` 노드(힙메모리 할당) -> `Initialize` 노드 -> `StoreI`(x, y필드에 10, 20 저장) -> `LoadI` (x, y읽기) -> `MulI`(곱하기) 노드들이 복잡한 의존성 간선으로 연결된다
2. **탈출 분석 단계에 돌입하는데** c2는 연결그래프를 구성하기 위해 객체의 참조가 메서드 스코프 밖으로 벗어나가는지 추적한다. 위 예시에서는 P객체는 다른 스레드와 공유되거나 반환되지 않으므로 상태가 `NoEscape로 마킹된다.
3. **스칼라 치환 Scalar Replacement**단계에서는 NoEscape 판정을 받은 객체에 대해서 c2는 힙 할당을 완전히 제거한다. 
   1. `Allocate` 노드와 메모리 동기화를 위한 `MemBar` 노드들이 Ideal Graph에서 삭제된다
   2. 객체의 필드 x, y는 더 이상 메모리에 종속된 데이터가 아니라, 독립적인 스칼라(마치 지역 변수 처럼)으로 취급된다
   3. C2의 옵티마이저는 `LoadI` 노드가 데이터를 읽어올 출처를 기존의 힙 메모리 주소에서, 바로 이전에 값이 할당된 상수 노드(Constant Node: 10, 20)로 직접 선을 이어버린다.
4. **최종 IR**: 할당 저장 로드과정이 모두 소멸하고 10과 20을 입력받는 `MulI(곱셈)` 노드만이 남아 레지스터 연산으로 직결된다.

<br>

## C2 기계어 vs GraalVM AOT 기계어 (런타임 가정과 역최적화 관점)

C2(JIT)와 GraalVM Native Image AOT가 생성하는 기계어는 겉보기엔 네이티브 바이너리지만

**런타임 환경에 대한 가정과 가정이 틀렸을때의 복구 deoptimization**능력에서 근본적인 차이가 존재한다.

### C2 JIT Compiler (Dynamic)

**Runtime 가정 (Optimistic Assumptions)**: C2는 1번에서 언급한 `MethodData`를 맹신하며 매우 공격적인 가정을 한다, 현재 로드된 클래스중에 `InterfaceA`를 구현한건 `ClassB` 뿐이다. (CHA, Class Hierachy Analysis)라고 가정하고 가상 메서드 호출(virtual call)을 인라인화해버린 빠르고 얇은 기계어를 생성한다.

**역최적화 Depotimization**: 런타임에 새로운 클래스가 동적으로 로드되어 앞선 가정이 깨지면 어떻게 될까? C2가 만든 기계어 곳곳에는 가정을 검증하는 `Guard` 명령어와 UncommonTrap이 심어져있다. 가정이 틀린순간 기계어 실행을 즉시 중단하고 레지스터 상태를 인터프리터의 스택 프레임으로 복원(deoptimization)하여 안전한 인터프리터 모드로 실행 흐름을 던져버린다.

### GraalVM AOT (Native Image)

**Runtime 가정 (Closed-World Assumption)**: AOT 컴파일은 빌드 시점에 존재하는 모든 클래스와 메서드는 이미 결정되었다는 닫힌 세계를 가정한다. closed-word. 런타임 프로파일링 데이터가 없기 때문에 PGO를 사용하지 않는 기본 상태 기준, 특정 분기문이 안쓰일것이거나 특정 타입만 들어올것이라는 **동적이고 도박적인 가정을 할 수 없다.**

> PGO: Profile-Guided Optimization 프로파일 기반 최적화는 프로그램이 실제 실행될 때 수집된 데이터(프로파일)를 바탕으로 컴파일러가 기계어를 더 빠르고 효율적으로 생성하도록 돕는 최적화 기법이다.

**역최적화 (No Interpreter Fallback)**: 네이티브 이미지는 JVM 인터프리터나 C1/C2 컴파일러를 바이너리에 포함시키지 않는다. 메모리를 줄이기 위해서다. 즉 **역최적화를 통해 되돌아갈 안정망이 없다는 것** 따라서 AOT가 생성한 기계어는 모든 발생 가능한 분기 처리와 타입 체크 로직을 포함해야하며, C2처럼 극단적으로 가벼운 인라인 코드를 만들기는 어렵다. 단 PGO-Profile Guided Optimization을 적용하면 미리 수집된 프로파일을 바탕으로 C2와 유사한 공격적 기계어를 생성할 수 있지만, 이 경우에도 인터프리터로의 폴백이 아닌 덜 최적화된 컴파일 코드로의 폴백 방식을 취한다.

이러한 차이점들로 인해 단기 실행이나 메모리 제약이 심한 환경에서는 AOT가 유리하지만, 오랫동안 실행되며 런타임 데이터가 쌓이는 환경 peak performace에는 c2 jit가 여전히 강력한 이유가 된다.


