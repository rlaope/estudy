# JVM이 구현하는 다형성 탈가상화 Devirtualization

JIT 컴파일러나 AOT 컴파일러가 성능 향상과 JVM이 다형성을 구현하기위해 발생하는 런타임 오버헤드(vtable) 조회에 대해서 오버헤드를 어떻게 제거하는지 보여주는 탈가상화 기술을 알아보겠다.

이 과정을 명확하게 이해하기 위해 앞서 자바 가상 머신 JVM이 인터페이스나 상위 클래스 타입으로 선언된 객체의 메서드를 호출할 때 내부 메모리에서 어떤 일이 일어나는지 확인해야한다.

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