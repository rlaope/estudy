# Kotlin Test Framework mockk Usage

## mockk
mockk is a library that helps you write test code in a Kotlin style.

You can think of it as a replacement for Mockito, which was previously used in Java.

To use mockk, you need to inject the mockk dependency as shown below.

```kts
testImplementation("io.mockk:mockk:${VERSION}")
```

### Creating Mock Objects

```kt
val domainRepository = mockk<DomainRepository>()
```

### Using Argument Matching

When assigning an expected answer to a specific object, you need to specify the parameters. Matchers work in the following way.

```kt
// Returns cycle only when the incoming parameter is 1L.
every { domainRepository.findById(1L) } returns Optional.of(cycle)

// Returns cycle regardless of the incoming parameter.
every { domainRepository.findById(any()) } returns Optionals.of(cycle)

// Returns cycle only when the incoming parameter is less than 3.
every { domainRepository.findById(less(3)) } returns Optionals.of(cycle)
```

### Expected Answer
- returns: Makes a specific method return a specific value.
- returnsMany: Returns the next element sequentially each time it is called multiple times.

```kt
every { domainRepository.findById(1L) } returnsMany listOf(Optional.of(domain), Optional.of(domain))
```

You can also use `andThen` instead of `returnsMany`.

```kt
every { mock1.call(5) } returns 1 andThen 2 andThen 3
```

`throws` makes it throw an `Exception`, and `just Runs` means doing nothing. (When creating a Mock object, if an unset method is called depending on the parameters, a runtime error may occur, in which case `just Runs` can be used.)

```kt
every { mock1.call(5) } throws RuntimeException("error happend")

every { mock1.callReturningUnit(5) } just Runs
```

`answers` allows you to write custom lambda functions that return an answer.

```kt
every { mock1.call(5) } answers { arg<Int>(0) + 5 }
```

### Verification

Mocking is a technique used to verify behavior, so there are various features for behavior verification.

Typically, you can verify whether a method has been called.

```kt
// Verifies that mock1.call(5) has been called at least once.
verify { mock1.call(5) }

// Verifies that mock1.call(5) has been called at least 5 times and at most 7 times.
verify(atLeast = 5, atMost = 7){
    mock1.call(5)
}

// Verifies that mock1.call(5) has been called exactly once.
verify(exactly = 5){
    mock1.call(5)
}

// Verifies that there was no interaction with mock1 at all.
vverify{
    mock1 wasNot Called
}
```

You can also verify the order.

```kt
// Verifies that mock1.call(1) -> mock1.call(2) -> mock1.call(3) were called in that exact sequence, with no other calls in between.

verifySequence{
    mock1.call(1)
    mock1.call(2)
    mock1.call(3)
}

// Verifies that mock1.call(1) was called before mock.call(3).

verifyOrder{
    mock1.call(1)
    mock1.call(3)
}
```
