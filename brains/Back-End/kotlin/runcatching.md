# Kotlin runCatching과 Result 타입

![](https://miro.medium.com/v2/resize:fit:1400/format:webp/1*wszuc8Bc4TvIutd_EljxZw.png)

runCatching은 코틀린 1.3버전부터 도입된 캡슐화 블록입니다.

runCatching 블록 안에서 성공/실패 여부가 캡슐화된 Result<T> 형태로 리턴합니다.

runCatching을 이용하면 코루틴 블록등을 RxJava에서 사용했던 것과 같이 유연한 이벤트 스트림으로 처리할 수 있습니다.

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

### 프로퍼티
Result<T>타입은 isSuccess와 isFailure를 프로퍼티로 갖습니다.

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

또한 테스트코드 작성시 가독성이 더 좋게 작성할 수 있습니다.

## Result 타입에서 값 가져오기

### getOrThrow()

```kt
colorName.getOrThrow()
// runBlocking 문에서 에러가 발생한 경우 해당 에러를 리턴합니다.
```

### getOrDefault

```kt
colorName.getOrDefault(defaultValue = "미상")
// runBlocking문에서 에러가 발생한 경우 defaultValue 파라미터를 리턴한다.
```

### getOrNull

```kt
colorName.getOrNull()
// runBlock문에서 에러가 발생한 경우 null을 리턴한다.
```

### getOrElse

```kt
colorName.getOrElse {
    e: Throwable ->
}
// runBlocking문에서 에러가 발생한경우 exception을 인자로받아
// 블록안의 값을 리턴합니다.캡슐화된 타입값과 같아야하기때문에 다른타입을
// 리턴하고싶다면 아래 mapCatching을 사용해야합니다.
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

위 코드에서는 firstUserAge와 secondUserAge 모두 getOrNull() 인스턴스 호출시 123값을 리턴합니다.

어떻게 보면 동일해 보이지만 블록 안의 에러가 발생한다면 다르게 처리됩니다.

### map
map은 블록 안의 에러가 발생할 경우 바깥으로 에러를 보냅니다.

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

mapCatching은 블록 안의 에러를 내부에서 처리하여 onFailure로 받을 수 있습니다.

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

map이 runCatching이 성공할 경우 호출되었다면 recover은 실패했을 경우 호출됩니다.

만약 runCatching문에서 에러가 발생할 경우 recover, recoverCatching 문이 호출된 후 리턴 값을 onSuccess로 전달합니다. 

하지만 map과 마찬가지로 블록 내에서 에러가 발생했을 경우 다르게 처리합니다.

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

