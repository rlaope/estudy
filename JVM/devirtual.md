# JVM이 구현하는 다형성 탈가상화 Devirtualization

JIT 컴파일러나 AOT 컴파일러가 성능 향상과 JVM이 다형성을 구현하기위해 발생하는 런타임 오버헤드(vtable) 조회에 대해서 오버헤드를 어떻게 제거하는지 보여주는 탈가상화 기술을 알아보겠다.

이 과정을 명확하게 이해하기 위해 앞서 자바 가상 머신 JVM이 인터페이스나 상위 클래스 타입으로 선언된 객체의 메서드를 호출할 때 내부 메모리에서 어떤 일이 일어나는지 확인해야한다.

### vtable의 물리적인 위치

가장 널리 퍼진 오해는 객체마다 vtable을 가지고 있다인데, 이는 사실이 아니고 객체마다 vtable을 가지면 메모리 낭비가 극심해진다.

**위치는 vtable은 자바 힙영역이 아닌 네이티브 메모리 영역인 metaspace에 위치한다**

Cpp 레벨에서 클래스의 메타데이터를 표현하는 `InstanceKlass` 구조체의 메모리 끝부분에 가변길이 배열 형태로 클래스당 딱 하나식 존재한다.

**객체와의 연결은**자바 힙에 생성된 객체 oop Ordinary Object Pointer는 객체 헤더에 8바이트의 마크워드와 4바이트또는 8바이트의 Klass Pointe를 가진다. 이 Klass Pointer가 Metaspace에 있는 자신의 InstanceKlass를 가르키며 이를 통해 vtable을 접근한다.

#### vtable 내부 구조와 생성 원리

vtable은 본질적으로 **메서드의 실행 메모리 주소를 담고잇는 1차원 배열 array of pointers**이다.

cpp 소스코드에서는 `vtableEntry`배열로 관리된다.

클래스가 로드될 때 JVM은 다음과 같은 엄격한 규칙으로 vtable 배열을 그대로 복사해온다.

1. **상속 복사**: 자식 클래스는 부모 클래스의 vtable 배열을 그대로 복사
2. **고정된 인덱스 보장**: 부모 클래스에 있는 메서드는 자식 클래스의 vtable에서도 항상 동일한 인덱스 vtable_index를 가진다. 예를 들어 `Object::hashCode()`가 vtable의 3번 인덱스라면, 이를 상속받는 String, Dog 클래스의 3번 인덱스에도 hashCode()가 있는것이다.
3. **오버라이딩**: 자식 클래스가 부모의 메서드를 오버라이딩하면 해당 인덱스의 포인터를 부모의 메서드 주소에서 **자식의 새로운 메서드 주소로 덮어쓴다.**
4. **확장**: 자식 클래스에서 완전히 새로운 메서드를 추가하면 vtable 배열 맨 끝에 새로운 인덱스를 부여받아 추가된다.

### `invokevirtual` 명령어 흐름

바이트코드 명령어인 `invokevirtual`이 실행될 때 cpu, jvm 내부에서는 다형성을 해결하기 위해 다음과 같은 포인터 추적 poointer chasing 연산이 발생한다.

예를들어 `animal.getLegCount()` animal은 컴파일 타임에는 Animal, 런타임에는 Dog 객체가 호출되는 과정이다.

1. **상수 풀 해석 Resolution**: JIT or Interpreter는 Animal::getLogCount() 메서드가 Animal 클래스의 vtable에서 몇 번 인덱스 예: Index 5를 사용하는지 컴파일 시점에 이미 일어난다. 이 인덱스는 자식인 Dog에도 변하지 않는다
2. **객체 접근**: 수신자 객체인 animal(oop)의 힙메모리 주소로 이동한다
3. **Klass 읽기**: 객체 헤더에서 Klass Pointer를 읽어 메타스페이스의 Dog 클래스 InstanceKlass 주소를 얻는다.
4. **vtable 접근**: InstanceKlass 메모리 주소에 vtable 시작 오프셋을 더하여 Dog의 vtable 배열 메모리로 이동한다.
5. **인덱스 조회**: 컴파일 시점에 알아낸 인덱스 5번(vtable[5])에 들어있는 값을 읽는다. Dog가 메서드 오버라이딩했으므로 여기에는 Animal이아닌 Dog::getLegCount()의 메타데이터 ()`Methid*`구조체)가 들어있다.
6. **코드 실행 Jump**: 해당 Method 구조체 안에 기록된 `_verified_entry_point` 실제 기계어가 컴파일 되어있는 코드캐시의 메모리로 cpu의 인스트럭션 포인터를 점프시킨다.

### 가상 메서드 호출과 vtable 조회 과정(탈가상화 이전)

자바의 모든 인스턴스 메서드는 기본적으로 **가상 메서드 virtual method** 이다.

즉, 컴파일 시점 `javac`에는 어떤 코드가 실행될지 정확히 알 수 없으며, 프로그램이 실행되는 런타임에 실제 메모리에 할당된 객체 타입을 확인할 메서드를 결정한다.

이를 동적 디스패치 **Dynamic Dispatch**라고 한다.

JVM 내부(C++ 런타임 환경)에선 이 과정을 처리하기 위해 **가상 메서드 테이블 virtual method table. vtable**을 사용한다.

메서드 호출 시 다음과 같은 복잡한 메모리 참조 pointer dereferencing 과정이 발생한다.

1. **객체 헤더 참조**: 호출된 객체(oop)의 메모리 주소로 이동하여 객체 헤더 mark word + klass pointer를 읽는다.
2. **Klass 메타데이터 참조**: 객체 헤더의 Klass Pointer를 찾아가 힙 밖 메타스페이스에 위치한 해당 클래스의 메타데이터 `InterfaceKlass` 구조체를 접근한다
3. **vtable 조회**: `InstanceKlass` 내부에 존재하는 `vtable` 배열에 접근한다
4. **메서드 주소 획득**: 호출하려는 메서드에 할당된 고정 인덱스를 사용하여, 실제 실행해야할 컴파일된 기계어 코드(또는 인터프리터 진입점)의 메모리 주소를 읽어온다.
5. **분기(Jump)**: 알아낸 메모리 주소로 cpu의 실행 흐름 instruction pointer를 이동시켜서 메서드를 실행한다.

이 과정은 매 호출마다 여러 번의 메모리 읽기를 유발하여, cpu의 명령어 파이프라인 효율을 떨어뜨리고 최적화(인스턴스화 등)를 방해하는 주요 원인이 된다.

### 탈가상화 Devirtualization 적용 과정

탈가상화는 컴파일러가 코드를 분석하여 위에서 설명한 복잡한 5단계 vtable 조회 과정을 **단 1단계의 직접 호출 direct call**로 단순화 하는 최적화 기법이다.

컴파일러(c2 or PGO가 적용된 GraalVM)는 런타임 프로파일링 데이터나 클래스 계층 구조 분석(CHA, Class Hierachy Analysis)를 통해 다음과 같이 판단한다.
- **이 호출지점 call site에서는 항상 특정 클래스의(ex Dog)의 인스턴스만 사용하는구나.**

이 사실이 증명되면 컴파일러는 객체의 헤더를 읽고 vtable을 뒤지는 기계어를 생성하는 대신 바로 **Dog 클래스의 메서드 주소로 점프 static call하는 기계어를 생성한다.** vtable 조회 자체가 생략되는 것이다.

### Java 코드 및 컴파일러 변환 예시

이해를 돕기 위해서 간단한 자바 코드 예시와 컴파일러 내부의 처리 변화를 설명하겠다.

```java
interface Animal {
    int getLegCount();
}

class Dog implements Animal {
    @Override
    public int getLegCount() {
        return 4;
    }
}

public class Main {
    public int calculateLegs(Animal animal) {
        // 인터페이스 타입으로 메서드 호출 (가상 메서드 호출 지점)
        return animal.getLegCount() * 2;
    }

    public static void main(String[] args) {
        Main m = new Main();
        Animal myDog = new Dog();
        for (int i = 0; i < 10000; i++) {
            m.calculateLegs(myDog);
        }
    }
}
```

**최초 실행시 인터프리터 또는 C1 컴파일 상태에서** 위의 코드에서 `calculateLegs` 메서드 내부의 `animal.getLongCount()`는 가상 메서드 호출이다. 매번 루프를 돌 때마다 jvm은 animal 변수가 가리키는 실제 메모리로 가서 객체 헤더를 읽고 Dog 클래스의 메타데이터에 있는 vtable을 조회하여 `getLegCount`의 주소를 찾아 실행한다

**프로파일러 데이터가 축적이 된다면**? 루프가 수천번 도는동안 컴파일러는 프로파일링 데이터 `MethodData` 구조체를 수집한다. 수집결과 `calculateLegs` 메서드에 전달된 `animal` 객체는 100% 확률로 `Dog` 타입임이 기록된다 이를 Monomorphic Call이라고 한다.

**C2 컴파일러가 탈 가상화 Devirtualization 적용 (IR 변환)**하게 되는데 C2가 `calculateLegs`를 최적화할 때 vtable 조회 로직을 제거하고 코드를 다음과 같이 내부적으로 변환한다. 개념적인 C2 IR 변환 상태는

```java
// 컴파일러가 내부적으로 최적화한 개념적 코드
public int calculateLegs(Animal animal) {
    // 1. 타입 검사 (가드 삽입 - Guard)
    if (animal.getClass() != Dog.class) {
        deoptimize(); // 만약 Dog가 아니면 C2 최적화를 취소하고 인터프리터로 돌아감
    }

    // 2. 탈가상화 (vtable 조회를 생략하고 Dog의 메서드 직접 호출)
    // return Dog::getLegCount(animal) * 2; 

    // 3. (추가 최적화) 인라인화 적용
    // getLegCount()의 구현체인 'return 4;'를 호출 지점에 직접 삽입
    return 4 * 2; 
}
```

결과적으로 vtable을 뒤지는 무거운 과정이 생략되고 최종적으로 생성된 기계어 타입 검사 후 상수 8을 반환하는 매우 단순하고 빠른 코드로 탈바꿈된다.

### JIT 컴파일러 입장에서본 vtable 한계점

위의 단계들을 보면 알수있듯 `invokevirtual`을 처리하기 위해 cpu는 메모리를 최소 3~4번 건너뛰어야한다.

포인터를 계속 따라가야하므로 cpu의 L1/L2 캐시 미스 확률이 높아진다.

가장 치명적인 점은 메모리를 뒤져봐야 실행할 타겟 코드를 알 수 있기 때문에 인라인화가 원천적으로 차단된다는 것이다.

> 인라인화는 어떤 코드가 실행될지 컴파일 시점에 100% 확신할 수 있어야만 가능함.

이러한 하드웨어 레벨의 비효율성을 제거하기위해 역가상화가 vtable 조회를 생략하고 직접 코드를 꽂아넣는 극단적인 최적화를 수행하는 것이다.

