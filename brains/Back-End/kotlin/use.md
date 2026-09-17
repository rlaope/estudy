# .close()가 필요한 클래스를 사용할 때, use로 효율적으로 사용하기

더 이상 필요하지 않을 때 `.close()` 메서드를 사용해 명시적으로 닫아야하는 클래스들이 있다.

예를 들면
- InputStream, OutputStream
- java.sql.Connection
- java.io.Reader
- java.new.Socket
- java.util.Scanner

이러한 리소스들은 `AutoCloseable`을 상속받는 `Closeable` 인터페이스를 구현하고 있다.

이러한 모든 리소스는 최종적으로 리소스에 대한 레퍼런스가 없어질 때, 가비지 컬렉터가 처리한다.

하지만 이러한 방식은 굉장히 느리며 쉽게 처리되지 않기때문에 사용하는 클라이언트가 명시적으로 .close() 작업을 처리하는 것이다.

전통적으로 try catch finally에 이러한 방식을 사용한다.

그러나 이런식으로 작성하게되면 복잡할 뿐만 아니라 catch와 finally에서 예외가 발생하면 예외가 따로 처리되지 않아 블록 내부에서 **오류가 발생하면 둘 중 하나만 예외가 전파되는 문제**가 있다.

물론 둘 다 전파될 수 있도록 구현할 수 있지만 정말 복잡해진다.

```kt
val reader = BufferdReader(FileReader(path))
try {
    return reader.lineSequence().sumBy { it.length }
} finally {
    reader.close()
}
```

그러나 use 함수를 사용해 앞의 코드를 적절하게 변경하면 다음과 같다. 이러한 코드는 모든 Closealbe 객체에 이용될 수 있다.

```kt
reader.use {
    return reader.lineSequence().sumBy { it.length }
}
```

> 추가로 파일을 리소스로 사용하는 경우가 많고, 파일을 한 줄씩 읽어들이는 경우에 제공하는 useLines도 제공한다.


마치 자바에서 `try catch resources`를 사용했던 것과 같다.


### 결론

use를 사용하면 AutoCloseable/Closeable을 구현한 객체를 쉽고 아전하게 처리할 수 있다.

또한 파일을 처리할 때는 파일을 한 줄씩 읽어들이는 useLines를 사용하는 것을 추천한다.