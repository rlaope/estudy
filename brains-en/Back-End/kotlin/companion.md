# My Favorite Kotlin companion object

It can be defined in a class as shown below.

```kt
class Base {
    companion object {
        val type: Int = 0
    }
}
```

It can be accessed like a static variable in Java, using `Class.variable`.

## 1. Primitive type or String

If you want a primitive type or String to be declared as `public static final` in the decompiled class, you prepend `const val` to the type. If you only use `val`, it will be as follows.

### When using const val

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

### When using val

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

## Non-primitive type
For reference types that are not primitive types or Strings, a slightly different approach is needed, as using `const` will result in an error.

You can use `@JvmField` to make it decompile as `public static final`.

### When using val
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

### When using @JvmField
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
