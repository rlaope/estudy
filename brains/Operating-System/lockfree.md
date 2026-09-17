# Lock-Free 자료구조 및 STM

CAS는 별도의 모니터락을 잡지않고 원자적인 연산으로 get set을 비교하여 가벼운 동시성 처리를 가능하게 하지만, cpu 자원낭비, 멀티코어 환경에서 캐시폭주, 기아 상태 불공정성및 여러 변수 조작의 한계등이 존재한다.

단일 변수 카운팅을 넘어 여러 개의 포인터가 얽힌 자료구조를 락 없이 구현하는 것은 매우 어렵다. 핵심은 **여러 단계의 수정을 어떻게 하나의 원자적 연산처럼 보이게 할 것인가이다.**

대표적으로 락 프리 자료구조 구현하거나 STM 등으로 여러 변수에대한 연산의 원자성을 보장하는데 하나씩 알아보도록 하자

### Michael-Scott Queue

가강 표준적인 락 프리 큐 알고리즘 이다. `java.util.concurrent.ConcurrentLinkedQueue`의 모태가 되기도 했다.

**복잡성**: head, tail 두 개의 포인터를 관리해야한다. 새로운 노드를 삽입할 때 Tail이 가리키는 노드의 next를 업데이트 하고 그 다음 Tail 자체를 이동시켜야 한다. 이 두 단계 사이에서 다른 스레드가 개입할 수 있기 때문에, CAS를 통해 중간 상태를 감지하고 다른 스레드가 작업을 도와서 마무리하는 Helping 로직이 포함된다.

```cpp
#include <atomic>

template<typename T>
class LockFreeQueue {
private:
    struct Node {
        T data;
        std::atomic<Node*> next;
        Node(T val) : data(val), next(nullptr) {}
    };
    std::atomic<Node*> head;
    std::atomic<Node*> tail;

public:
    void enqueue(T value) {
        Node* newNode = new Node(value);
        while (true) {
            Node* currentTail = tail.load();
            Node* tailNext = currentTail->next.load();
            
            if (currentTail == tail.load()) {
                if (tailNext == nullptr) {
                    // 1. Tail의 next를 새 노드로 변경 시도 (CAS)
                    if (currentTail->next.compare_exchange_weak(tailNext, newNode)) {
                        // 2. 성공 시 Tail 포인터를 새 노드로 이동 (CAS)
                        tail.compare_exchange_strong(currentTail, newNode);
                        return;
                    }
                } else {
                    // 다른 스레드가 노드는 추가하고 Tail은 갱신하지 못한 상태 (Helping)
                    tail.compare_exchange_strong(currentTail, tailNext);
                }
            }
        }
    }
};
```

### Harris's Linked List (락 프리 연결 리스트)

연결 리스트에서 노드를 삭제하는 작업은 까다롭다.

**복잡성**: 노드가 a-b-c 가 있을때 b를 삭제하려면 a의 next를 c로 바꾸어야한다. 하지만 그 찰나에 다른 스레드가 b뒤에 d라는 새 노드를 삽입하려고하면 b의 next를 수정, 데이터가 유실되거나 구조가 깨질 수 있다.

**해결책**: Logical Delete 기법을 사용한다. 포인터의 마지막 1비트를 마킹하여 이 노드는 곧 삭제될 것이다. 를 원자적으로 표시한뒤 나중에 실제 물리적 링크를 변경한다.

```cpp
#include <atomic>
#include <cstdint>

struct Node {
    int value;
    std::atomic<Node*> next;
};

// 포인터의 마지막 비트를 1로 마킹하여 '논리적 삭제' 상태를 나타내는 헬퍼 함수
Node* getMarkedReference(Node* ref) {
    return reinterpret_cast<Node*>(reinterpret_cast<uintptr_t>(ref) | 1);
}

bool logicalDelete(Node* target, Node* targetNext) {
    // 타겟 노드의 next 포인터에 삭제 마킹을 시도 (CAS)
    Node* markedNext = getMarkedReference(targetNext);
    
    // target->next가 targetNext와 같다면 markedNext로 변경 (성공하면 삭제 상태가 됨)
    return target->next.compare_exchange_strong(targetNext, markedNext);
}
```

### Lock-free Skip List

이진 탐색 트리 대신에 락 프리 환경에서 많이 쓰이는 정렬된 자료구조다.

**복잡성**: 여러 층의 링크를 동시에 수저애향 하므로 CAS만으로는 구현이 극도로 복잡하다. 하지만 삽입, 삭제시에 특정 구역에만 영향을 미치므로 확장성이 뛰어나다.

```cpp
#include <atomic>

struct Node {
    int key;
    // 다층(Level) 링크 배열 (단순화를 위해 1층만 표시)
    std::atomic<Node*> next[1]; 
};

bool insertBottomLevel(Node* pred, Node* succ, Node* newNode) {
    newNode->next[0].store(succ);
    
    // 이전 노드(pred)가 여전히 다음 노드(succ)를 가리키고 있다면, 
    // 그 사이에 내 노드(newNode)를 끼워 넣음 (CAS)
    return pred->next[0].compare_exchange_strong(succ, newNode);
    // 실패하면 다른 스레드가 pred 뒤에 무언가를 추가한 것이므로 탐색부터 재시도해야 함
}
```

#### 메모리 재사용 문제 Hazard Pointer, EBR

락프리 자료구조의 진짜 숨은 복잡성은 누군가 참조 중인 메모리를 언제 안전하게 해제할 것인가에 있다.

- **Hazard Pointers**: 스레드가 현재 접근중인 포인터를 위험군으로 공표하여 다른 스레드가 그 메모리를 해제하지 못하게 막는 기법이다.
- **Epoch-based Reclamation (EBR)**: 세대 epoch를 나누어 모든 스레드가 특정 세대가 지나갔음이 확인될 때 한꺼번에 메모리를 수거한다.

## 소프트웨어 트랜잭셔널 메모리 STM

STM(Software Transactional Memory) 개념을 메모리 영역으로 가져온 동시성 제어 모델이다. 락이나 복잡한 CAS로직을 직접 짜는 대신에 이 블록의 코드는 원자적으로 실행되어야 한다 라고 선언하는 방식

### atomic { ... }

개발자는 복잡한 동기화 로직 대신에 특정 코드 블록을 트랜잭션으로 지정한다.

- **Optimistic Concurrency**: 일단 다른 스레드가 방해하지 않을것이라 가정하고 트랜잭션 내부의 연산을 자유롭게 수행한다.
- **Rollback & Retry**: 작업을 마치고 커밋하려는 순간 내가 읽거나 쓴 메모리 주소값이 다른 스레드에 의해 바뀌었는지확인후 충돌이 발생하면 취소하고(rollback) 처음부터 다시 시도한다.

작동 메커니즘은 

1. **Read Set / Write Set**: transaction 내에서 읽은 값과 기록하려는 값을 별도 log에 기록한다 실제 메모리는 건들지 않음
2. **Validation**: 커밋 직전에 read set에 기록된 주소들의 현재 값이 트랜잭션 시작 시점과 동일한지 확인한다.
3. **Commit**: 검증에 성공하면 Write Set의 내용을 실제 물리 메모리에 한꺼번에 반영한다.

#### 장단점

- **장점**: 데드락 걱정이 없으며, 여러개의 메모리 위치를 동시에 수정하는 로직을 매우 쉽게 짤수있다. 구정형 composability이 좋아 작은 트랜잭션을 합쳐 큰 트랜잭션을 만들기가 쉽다
- **단점**: 충돌이 잦은 환경 High Contention 에서는 계속 롤백이 일어나면서 CAS의 스핀락보다 더 심한 성능 저하가 발생할 수 있다. 또한 모든 메모리 접근 로그를 남겨야하므로 읽기/쓰기 비용 오버헤드가 더 크다.

```clojure
;; Clojure 언어를 사용한 STM (가장 대표적이고 우아한 STM 구현체)
;; 계좌 이체(Bank Transfer) 예시

;; refs를 생성 (STM으로 관리되는 메모리 공간)
(def account-A (ref 1000)) ; A 계좌 1000원
(def account-B (ref 0))    ; B 계좌 0원

(defn transfer [from to amount]
  ;; dosync 블록: 이 안의 연산은 모두 하나의 원자적 트랜잭션으로 처리됨 (atomic)
  (dosync
    (if (>= @from amount)
      (do
        ;; alter: 트랜잭션 내에서 값을 변경 시도
        (alter from - amount)
        (alter to + amount)
        "이체 성공")
      "잔액 부족")))

;; 실행: 여러 스레드에서 동시에 transfer를 호출해도,
;; Clojure의 STM 메커니즘이 백그라운드에서 충돌을 감지하고 롤백/재시도를 수행함
(transfer account-A account-B 100)
```

실제로 STM을 구현한 사례는 위처럼 클로저 언어나 하스칼 언어 datomic 데이터베이스등이 존재한다.

데이터베이스랑 다른거아니에요? 할 수 있는데. 그 질문에 대한 답을 해보자면 STM은 데이터베이스 트랜잭션 모델을 프로그램의 메모리 RAM 위로 그대로 가져온것이라 생각하면 편하다.

작동하는 물리적 환경이 메모리인지 디스크인지의 차이고 실제 구현 방식과 목적에서 명확한 차이가 있다.

**데이터베이스**: 변경된 데이터를 디스크에게 안전하게 기록 write-ahead logging fsync 해야하며 커밋 직후에 서버 전원이 뽑혀도 durability를 보장해야하며 충돌시 디스크 io와 같은 네트워크 통신이 얽혀있기 때문에 롤백 비용이 매우 크다 잦은 롤백을 피하기위해 락을 걸고 대기하는 비관방식이나 복잡한 mvcc등을 적극 활용하며 ms밀리초 단위의 싸움이다. 아무리 빨라도 스토리지라서

**STM**: 프로그램이 실행되는동안 휘발성 메모리 상태만 관리하고 커밋이 성공하더라도 컴이 꺼지면 데이터가 사라진다. 즉 acid중 ACI는 보장해도 D 영속성은 포기한모델이며 단순한 메모리 상의 로컬 변수나 포인터를 리셋하면 끝이라 롤백이 저렴하다. 락없이 마음껏 수정해보고 커밋할때충돌이나면 그냥 처음부터 다시 계산하자는 낙관적 방식을 기본적으로 채택하고 처리시간은 나노에서 마이크로초 쌍무이라 cpu캐시와 ram 속도에 맞춰 수많은 스레드가 실시간으로 변수 값을 읽고 쓰는 찰나에 동기화처리를 한다.