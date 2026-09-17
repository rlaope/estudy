# 자바에서 Finalize 메서드의 역할은 무엇일까요?

### finalize()

`finalize()` 메서드는 자바의 Object 클래스에 정의된 메서드로, 객체가 가비지 컬렉션에 의해 회수되기 직전에 실행되는 코드를 작성하기 위해 사용됩니다.

```java
복사
protected void finalize() throws Throwable {
    try {
        // 객체 정리 로직
    } finally {
        super.finalize();
    }
}
```

이 메서드는 가비지 컬렉터에 의해 자동으로 호출되며, 해당 객체가 사용되던 리소스를 회수하기 위한 마지막 기회를 제공합니다.

예를 들어, 객체가 네트워크 연결이나 파일 핸들과 같은 시스템 리소스를 사용하는 경우, finalize() 메서드에서 이러한 리소스를 명시적으로 해제하는 코드를 작성할 수 있습니다.

그러나 실제로 이 메서드를 사용하는 것은 권장하지 않습니다. 이는 가비지 컬렉터의 실행 시점을 정확히 예측할 수 없으므로 finalize() 메서드가 호출되는 시점 역시 예측할 수 없기 때문입니다.

이로 인해 리소스 누수가 발생하거나, 성능에 부정적인 영향을 미칠 수 있습니다.

따라서, 대신에 try-finally 블록이나 try with resources 구문을 사용하여 필요한 시점에 직접 리소스를 해체하는 것이 좋습니다.

> Java 9부터는 finalize() 메서드가 deprecated 되었고 java 12에서는 Cleaner, PhantomReference를 사용한 대안이 제시되었습니다.