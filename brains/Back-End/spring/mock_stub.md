# Mock 테스트와 Stub 테스트의 차이

## 사전 개념

### Dummy
Dummy 객체는 전달되지만 사용되지 않고 일반적으로 매개변수 목록을 채우는 목적으로만 사용됨

### Fake
Fake 객체는 실제로 작동하는 구현을 가지고 있지만 일반적으로 프로덕션에 적합하지 않는 몇가지 지름길을 사용 
  
대표적 예시로는 메모리 데이터베이스가 있다.

### Stub
Stub은 테스트 중에 만들어진 호출에 미리 준비된 답변을 제공하며 일반적으로 테스트를 위해 프로그래밍된 것 외에는 전혀 응답하지 않는다.

### Spy
Spy는 어떻게 호출받았는지에 따라 일부 정보를 기록하는 Stub이다.

### Mock
Mock은 예상되는 기대값으로 미리 프로그래밍 객체이다.

## 테스트 대역(Test Double)

위의 개념에 대해 이해하기전 우선 테스트 대역이라는 개념을 이해해야한다.

![](https://user-images.githubusercontent.com/42582516/155876237-36a220bd-8e0f-4f1c-b0be-6a4a012fb3d2.png)

테스트 대역이란 테스트하려는 객체가 다른 객체들이 여러 관계가 엮여있어 사용하기 힘들 때, 대체할 수 있는 객체를 의미한다.  
  
테스트 대역은 Dummey, Stub, Spy, Mock, Fake로 나누어진다.

## Mock vs Stub
일반적으로 많이 사용하는 두 개념에 대해서 정리를 하면 다음과 같다.
  
> Test의 원칙에 따르면 하나의 테스트에는 여러 개의 스텁이 있을 수 있지만 일반적으로 모의는 하나만 있다.

## Stub
인스턴스화하여 구현한 가짜 객체(Dummy, 기능 구현 x)를 이용해 실제로 동작하는것 처럼 보이게 만드는 객체
  
해당 인터페이스나 클래스를 최소한으로 구현한다.
  
테스트에서 호출된 요청에 대해 미리 준비해둔 답변을 응답한다.
  
테스트시에 프로그래밍된 것 외에는 응답하지 않는다.
  
협력 객체의 특정 부분이 테스트가 어려운 경우, stub을 사용하여 수월하게 테스트할 수 있다.

### Stub's Lifecycle
- Setup, 테스트 준비  
- Exercise, 테스트  
- Verify state, 상태 검증  
- Teardown, 리소스 정리

<br>

## Mock
호출에 대한 기대를 명세하고, 내용에 따라 동작하도록 프로그래밍된 객체
  
테스트 작성을 위한 환경 구축이 어려울 때, 테스트하고자 하는 코드와 엮인 객체들을 대신하여 만들어진 객체다.
  
행위 검증을 진행한다.

### Mock's Lifecycle

- Setup data, 데이터 준비
- Setup expectations, 예상되는 결과 준비
- Exercise, 테스트
- Verify expectations, 예상 검증
- Verify state, 상태 검증
- Teardown, 리소스 정리

## Stub과 Mock의 차이
stub을 포함한 다른 대역들은 `상태 검증(state verification)`을 사용하고 Mock 오브젝트는 `행위 검증(behavior verification)`을 사용한다
  
> 상태 검증: 메서드가 수행된 후, 객체의 상태를 확인해 올바르게 동작했는지 확인하는 검증법  
> 행위 검증: 메소드의 리턴 값으로 판단할 수 없는 경우, 특정 동작을 수행하는지 확인하는 검증법
  
검증의 대상이 다르다는 것이 중요한 체크요소다.

### 상태 검증 예시
```java
StateClass stateClass = new StateClass();
stateClass.doSomething()

assertThat(stateClass.getStatus()).isEqualTo(true);
```

### 행위 검증 예시
```java
BehaviorClass behaviorClass = new BehaviorClass();

verify(behaviorClass).doBehavior();
```

## 조금 더 상세한 예시

### Stub
사용하기 쉬우며 추가 종속성이 없다.

```java
public class SimpleService implements Service {

    private Collaborator collaborator;

    public void setCollaborator(Collaborator collaborator){
        this.collaborator = collaborator;
    }

    // part of Service interface

    public boolean isActive(){
        return collaborator.isActive()
    }
}
```

```java
public void testActiveWhenCollaboratorIsActive() throws Exception {

    service.setController(new Collaborator(){
        public boolean isActive(){
            return true;
        }
    });

    assertTrue(service.isActive());
}
```

### Mock 
```java
Collaborator collaborator = EasyMock.createMock(Collaborator.class);
EasyMock.expect(collaborator.isActive()).andReturn(true);
EasyMock.replay(collaborator);

service.setCollaborator(collaborator);
assertTrue(service.isActive());

EasyMock.verify(collaborator);
```

## 언제 Stub과 Mock을 사용하는가?

기본적으로 적합하다고 판단될때(당연)
  
행위 검증(Mock)의 경우 특정 메서드의 호출 등을 검증하기 때문에 구현에 의존적이다. 
  
상태 검증(Stub)의 경우 상태를 노출하는 메서드가 많이 추가될 수 있다.
  
많은 경우 상태 검증이 좋은 경우가 많다.
  
그러나 상태 검증이 어려운 경우가 있어서 이때는 행위 검증 혹은 전체 테스트를 진행하는 것도 좋은 방법이 된다.