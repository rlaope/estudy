# Kotlin Generics Star Projection, reified Types

## Star Projection
Star projection `<*>` can accept any type, but once the type is concretely determined, it can only hold elements of that type or its subtypes.

If a type parameter defined with `in` is received as a `*` star projection, it is treated as `in Nothing`,

and if a type parameter defined with `out` is received as a `*` star projection, it is treated as `out Any?`.

Therefore, when using `*` star projection, method calls may be restricted depending on its position.

```kt
class InOut<in T, out U>(t: T, u: U){
    val prop: U = u // U는 out 위치

    fun fuc(t: T){ // T는 in 위치
        print(t)
    }
}

fun starFuc(v: InOut<*,*>) {
    v.fuc(1) // 오류, Nothing으로 인자 처리
    print(v.prop)
}
```

## reified Types

Generics are erased at runtime after compilation, so the generic type cannot be accessed.

Therefore, to access the type, it must be passed as a function parameter like `c: Class<T>`.

However, if the type parameter T is specified with `reified`, it becomes accessible at runtime, eliminating the need to pass parameters like `c: Class<T>`.

However, reified types can only be used in inline functions.

```kt
inline fun <reified T> fuc() {
    print(T::class)
}
```
