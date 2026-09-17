# Kotlin runCatching and Result Type

![](https://miro.medium.com/v2/resize:fit:1400/format:webp/1*wszuc8Bc4TvIutd_EljxZw.png)

runCatching is an encapsulation block introduced in Kotlin 1.3.

Inside a runCatching block, the success/failure status is returned as an encapsulated Result<T>.

Using runCatching, you can process coroutine blocks and similar constructs as flexible event streams, much like in RxJava.

## runCatching

```kt
val colorName: Result = runCatching {
    when(color) {
        Color.BLUE -> "파란색"
        Color.RED -> "빨간색"
        Color.YELLOW -> "노란색"
        Color.BLUE -> throw Error("처음 들어보는 색")
    }
}.onSuccess {
    it: String ->
    // 성공시만 실행
}.onFailure {
    it: Throwable ->
    // 실패시에만 실행 try catch의 catch와 유사
}
```

### Properties
The Result<T> type has isSuccess and isFailure as properties.

```kt
if(colorName.isSuccess){
    // 성공시 호출
}

if(colorName.isFailure){
    // 실패시 호출
}
```
```kt
feature("Repository Test") {
    val repository = MockRepository()
    val dummyUser = SampleHelper.dummyUser()

    scenario("should be able to insert a user In Repo") {
        val result = runCatching {
            runBlocking { 
                repository.insert(dummyUser)
            }
            result.isSuccess shouldBe true
        }
    }
}
```

It also allows for writing more readable test code.

## Getting Values from Result Type

### getOrThrow()

```kt
colorName.getOrThrow()
// If an error occurs in the runBlocking statement, it returns that error.
```

### getOrDefault

```kt
colorName.getOrDefault(defaultValue = "미상")
// If an error occurs in the runBlocking statement, it returns the defaultValue parameter.
```

### getOrNull

```kt
colorName.getOrNull()
// If an error occurs in the runBlock statement, it returns null.
```

### getOrElse

```kt
colorName.getOrElse {
    e: Throwable ->
}
// If an error occurs in the runBlocking statement, it takes the exception as an argument
// and returns the value inside the block. Since it must be the same as the encapsulated type value,
// if you want to return a different type, you should use mapCatching below.
```

## map, mapCatching

```kt
val firstUserAge: Result<Int> = runCatching {
    "123"
}.map {
    it: String -> it.toInt()
}

val secondUserAge: Result<Int> = runCatching {
    "123"
}.mapCatching {
    it: String -> it.toInt()
}
```

In the code above, both firstUserAge and secondUserAge return the value 123 when the getOrNull() instance is called.

While they might seem identical, they handle errors differently if an error occurs within the block.

### map
If an error occurs within the block, map propagates the error outwards.

```kt
try {
    runCatching {
        database.getUser(id)
    }.map {
        user: User ->
        // 강제로 에러 발생
        throw Error("유저 정보를 가져올 수 없습니다.")
    }.onSuccess {
        // map 블록에서 에러 발생시 실행되지 않습니다.
    }.onFailure {
        // map 블록에서 에러 발생시 실행되지 않습니다.
    }
} catch (e: Exception) {
    // map 블록에서 발생한 에러 인자로 받아 호출됩니다.
}
```

### mapCatching

mapCatching handles errors within the block internally, allowing them to be received by onFailure.

```kt
runCatching {
    database.getUser(id)
}.mapCatching {
    user: User? ->
    // 강제로 예외 발생
    throw Error("유저 정보를 가져올 수 없음")
}.onSuccess {
    // mapCatching 블록에서 에러 발생시 실행되지 않습니다.
}.onFailure {
    // mapCatching 블록에서 에러 발생시 호출됩니다.
}
```

## recover, recoverCatching

While map is called when runCatching succeeds, recover is called when it fails.

If an error occurs in the runCatching statement, recover or recoverCatching is called, and then the return value is passed to onSuccess.

However, similar to map, they handle errors differently if an error occurs within their blocks.

```kt
try {
    runCatching {
       throw Error("runBlock문 에러발생")
    }.recover { it: String ->
        throw Error("recover문 에러발생")
    }.onFailure {
        //에러가 전달되지않습니다.
    }
} catch (e: Exception) {
    //recover에서 발생한 에러가 받아집니다.
}
```

### runCatching

```
runCatching {
    throw Error("runBlock문 에러발생")
}.recoverCatching { it: String ->
    throw Error("recover문 에러발생")
}.onFailure {
    //이곳에서 에러를 받습니다.
}
```

![](https://miro.medium.com/v2/resize:fit:1400/format:webp/1*YqtLTx7NUc-x5bvyzH525w.png)
