# 내가 제일 좋아하는 Kotlin companion object

Class에서 아래와 같이 정의할 수 있다.

```kt
class Base {
    companion object {
        val type: Int = 0
    }
}
```

java에서 static variable 처럼 `Class.variable`로 접근이 가능하다.

## 1. Primitive type or String

원시타입이거나 String인 경우 de-compile 되었을 때 해당 클래스에 public static final로 선언하게 하고 싶으면 타입 앞에 const val를 붙이게 된다. 만약 val만 사용한다면 아래처럼 된다.
  
### const val 사용시

kotlin
```kt
// 
class Base {
    companion object {
        const val type: Int = 0
    }
}
```

java
```java
class Base {
    public static final int type = 0;
}
```

### val 사용시

kotlin
```kt
class Base {
    companion object {
        val type: Int = 0
    }
}
```

java
```java
class Base {
    private static final int type = 0;

    public static final class Companion {
        public final int getType(){
            return Base.type;
        }
    }
}
```

## non-primitive type
원시타입이나 String이 아닌 레퍼런스 타입이라면 조금 다른 방법을 이용해야한다. const를 사용하면 에러가 발생하기 때문이다.
  
`@JvmField`를 이용해서 public static final로 de-compile되게 할 수 있다.

### val 사용시
kotlin
```kt
class Base {
    companion object {
        val type: User = User()
    }
}
```

java
```java
class Base {
    private static final User type = new User();

    public static final class Companion {
        public final User getType(){
            return Base.type;
        }
    }
}
```

### @JvmField 사용시
kotlin
```kt
class Base {
    companion object {
        @JvmField
        val type: User = User()
    }
}
```

java
```java
class Base {
    public static final User type = new User();
}
```