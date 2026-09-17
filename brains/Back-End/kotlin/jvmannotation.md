# @JvmField && @JvmStatic 어노테이션

## @JvmField

`@JvmField`는 getter, setter를 생성하지 말라는 의미다.  
  
다음 코틀린 코드에서 프로퍼티 var count는 getter/setter를 생성한다.
```kt
class A{
    var count = 0
}
```

`@JvmField` 어노테이션을 붙이게 되면 자바로 변환했을 때 getter, setter가 생기지 않게 된다.
```kt
class A {
   @JvmField
   var count = 0
}
```

## @JvmStatic

함수와 프로퍼티에 static하게 접근할 수 있도록 추가적인 메서드 getter, setter를 생성한다.  
  
다음 A 클래스에서 count라는 변수를 companion object에 선엄함으로써, 전역변수를 만들었다.
```kt
class A {
    companion object {
        var count : Int = 0
    }
}
```

```java
public final class A {
   private static int count;
   public static final class Companion {
      public final int getCount() {
         return A.count;
      }
      public final void setCount(int var1) {
         A.count = var1;
      }
   }
}
```

자바로 변환하면 A 클래스에 count는 선언되어 있지만 Getter, Setter는 A.Companion 클래스에 등록된 것을 볼 수 있다.
  
자바에서 Getter, Setter 함수에 접근하려면 다음처럼 Companion을 꼭 써줘야한다.
```kotlin
A.Companion.getCount();
A.Companion.setCount(100);
```
static과 companion object가 다르다고 하는 이유가 바로 이것이다. Kotlin만 사용할때는 다른 것이 없지만. java, kotlin을 같이 사용하는 경우에는 이 부분에서 차이가 난다.  
  
companion object를 static 처럼 사용하려면 `@JvmStatic`을 사용해야한다.

```kt
class A {
    companion object {
        @JvmStatic var count : Int = 0
    }
}
```

자바로 변환해보면 A 클래스에 count가 선언되어 있고, A 클래스와 A.Companion 클래스에 getter,setter함수가 모두 생성된 것을 볼 수 있다.

```java
public final class A {
   private static int count;
   public static final int getCount() {
      return count;
   }

   public static final void setCount(int var0) {
      count = var0;
   }

   public static final class Companion {
      public final int getCount() {
         return A.count;
      }
      public final void setcount(int var1) {
         A.count = var1;
      }
   }
}
```

자바에서도 위 코드를 접근하려면 A.Companion도 가능하지만 A.getCount와 같이 바로 접근도 가능하다.
```java
A.getCount();
A.setCount(10);
A.Companion.getCount();
A.Companion.setCount(10);
```

@JvmStatic을 사용하면 클래스도 마찬가지로 Companion 키워드 없이 바로 접근이 가능하다.  
  
정리하면 @JvmStatic은 Companion에 등록된 변수를 자바의 static 처럼 선언하기 위한 어노테이션이다.