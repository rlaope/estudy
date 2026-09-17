# 코틀린 테스트 프레임워크 mockk 사용법

## mockk
mockk은 코틀린 스타일로 테스트 코드를 작성할 수 있도록 도와주는 라이브러리다.
  
기존에 자바에서 사용하던 mockkito를 대체한다고 보면 된다.
  
mockk을 사용하기 위해선 아래처럼 mockk에 대한 의존성을 주입해야한다.

```kts
testImplementation("io.mockk:mockk:${VERSION}")
```

### Mock 객체 생성하기

```kt
val domainRepository = mockk<DomainRepository>()
```

### Argument Matching 사용하기

특정 객체에 expected answer를 부여할 때 파라미터를 지정ㅇ해주어야 한다. matcher는 다음과 같은 느낌으로 동작한다.

```kt
// 들어오는 파라미터가 1L 일때만 cycle을 반환한다.
every { domainRepository.findById(1L) } returns Optional.of(cycle)

// 어떤 파라미터가 들어오더라도 cycle 반환한다.
every { domainRepository.findById(any()) } returns Optionals.of(cycle)

// 3 이하의 파라미터가 들어올때만 cycle을 반환한다.
every { domainRepository.findById(less(3)) } returns Optionals.of(cycle)
```

### Expected Answer
- returns: 특정 메서드가 특정한 값을 반환하도록 함
- returnsMany: 여러번 호출될 때 마다 순차적으로 다음 요소를 반환하도록 한다.

```kt
every { domainRepository.findById(1L) } returnsMany listOf(Optional.of(domain), Optional.of(domain))
```

`returnsMany` 대신에 `andThen`을 사용할 수도 있다.

```kt
every { mock1.call(5) } returns 1 andThen 2 andThen 3
```

throws는 Exception을 내게 해주며, just Runs는 아무것도 하지 않음을 의미한다. (Mock 객체를 생성할때 파라미터에 따라 설정되지 않은 메서드가 호출되면 런타임 에러가 날 수 있는데 이럴때 just Runs를 쓸 수도 있다.)


```kt
every { mock1.call(5) } throws RuntimeException("error happend")

every { mock1.callReturningUnit(5) } just Runs
```

`answers` 는 정답을 반환하는 커스텀 람다 함수를 작성할 수 있게 한다.

```kt
every { mock1.call(5) } answers { arg<Int>(0) + 5 }
```

### 검증하기

mocking은 행위를 검증하기 위해 사용되는 기술인만큼, 행위 검증을 위한 기능들이 다양하게 존재한다.
  
대표적으로 메서드의 호출 여부를 검증할 수 있다.

```kt
// mock1.call(5) 가 한 번 이상 호출되었는지 검증
verify { mock1.call(5) }

// mock1.call(5)가 5번 이상 7번 이하로 호출되었는지 검증
verify(atLeast = 5, atMost = 7){
    mock1.call(5)
}

// mock1.call(5)가 정확히 한 번 호출되었는지 검증
verify(exactly = 5){
    mock1.call(5)
}

// mock1과의 상호작용이 전혀 없었는지를 검증
vverify{
    mock1 wasNot Called
}
```

또한 순서를 검증할 수도 있다.

```kt
// mock1.call(1) -> mock1.call(2) -> mock1.call(3) 순서로 정확히 호출되었고 그 사이 다른 호출이 없었음을 검증

verifySequence{
    mock1.call(1)
    mock1.call(2)
    mock1.call(3)
}

// mock1.call(1)이 mock.call(3) 보다 먼저 호출되었음을 검증

verifyOrder{
    mock1.call(1)
    mock1.call(3)
}
```