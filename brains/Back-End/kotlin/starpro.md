# Kotlin Generics 스타 프로젝션, reified 자료형

## 스타 프로젝션
스타 프로젝션 `<*>`은 어떤 자료형이라도 들어올 수 있으나 구체적으로 자료형이 결정되고 난 후엔ㄴ 그 자료형과 하위 자료형의 요소만 담을 수 있도록 제한할 수 잇습니다.
  
in으로 정의되어 있는 타입매개변수를 * 스타 프로젝션으로 받으면 in Nothing으로 간주하고,  
  
out으로 정의되어 있는 타입매개변수를 * 스타 프로젝션으로 받으면 outAny?인 것으로 간주합니다.
  
따라서 * 스타 프로젝션을 사용할 때 그 위치에 따라 메서드 호출이 제한될 수 있습니다.

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

## reified 자료형

제네릭은 컴파일 후 런타임에는 삭제되기 때문에 제네릭 타입에 접근할 수 없다.
  
그래서 `c: Class<T>`처럼 제네릭 타입을 함수의 매개변수로 전달해야 타입에 접근할 수 있다.
  
하지만 `reified`로 타입 매개변수 T를 지정하면 런타임에서도 접근할 수 있게되어 `c: Class<T>`와 같은 매개변수를 넘겨주지 않아도 된다.
  
하지만 refieid 자료형은 인라인 함수에서만 사용할 수 있다.

```kt
inline fun <reified T> fuc() {
    print(T::class)
}
```