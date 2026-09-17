# Golang의 포인터와 역참조

C랑 비슷하지만 다르다.
  
C언어에서는 배열 이름 자체가 배열의 첫번째 인덱스 요소 주소값인데 Go언어에는 그런 것이 없다. 주소값은 어떤 변수 앞에 &를 붙이는 것만 기억하면 된다.
  
C언어에서는 *(배열이름 _ 인덱스)는 배열이름[인덱스]와 같은 기능을 했는데 Go언어에는 그런 것이 없다. 직접 참조를 원하면 포인터 변수 앞에 *를 붙이는 것만 기억하면 된다.
  
함수를 호출할 때 주소값 전달을 위해 "함수이름(&변수이름)"을 입력하고 매개변수이름을 입력할 때는 값을 직접 참조하기 위해 *를 매개변수형 앞에 붙인다. 그리고 함수 안에서 매개변수앞에 모두 *를 붙여야한다.
  
Go만 쓴다면 포인터 / 역참조에 대한 개념은 다음과 같이 이해할 수 있다.
- 값은 메모리에 저장되며 변수는 일종의 alias다.
- 메모리 주소를 값으로 가진 변수를 포인터라고 부른다.
- 포인터가 가리키는 값을 가져오는 건 역참조라고 한다.
- 메모리 주소를 직접 대입하거나 포인터 연산을 허용하지 않는다.

## 기본적인 포인터 사용법

&를 통해 메모리값을 출력할 수 있다.

```go
func main() {
    a := 2
    b := a // 값 복사 원시 타입
    a = 10
    fmt.Println(&a, &b) // &를 붙여 주소 값이 나온다. 서로 다른 메모리 주소
}
```

메모리 값을 변수에 할당하면 그게 포인터다. 아래 코드에서는 b는 포인터다.
  
*를 통해 역참조를 할 수 있고 더 나아가 해당 메모리에 특정 값을 다시 넣을 수도 있다.

```go
func main() {
    a := 5
    b := &a
    fmt.Println(b)
    fmt.Println(*b) // 역참조
}
```

:= 축약형이 아닌 *자료형 꼴을 이용해 포인터 변수를 만들 수도 있다.

```go
func main() {
    var a *int // 포인터 변수. a의 역참조 값은 int다.
    b := 3
    a = &b // 주소를 할당해주어야함

    fmt.Println(a) // 주소
    fmt.Println(*a) // 역참조
}
```

C를 사용해왔다면 아래 코드가 더욱 익숙할 것이다. 아래의 경우 *int로 numPtr이라는 포인터를 만들었다.

```go
func main() {
    var numPtr *int = new(int)
    *numPtr = 10
    fmt.Println(numPtr) // 주소값
    fmt.Println(*numPtr) // 10
}
```

== 연산자를 통해 포인터가 같은 메모리 주소를 가리키는지도 확인할 수 있다.

```go
func main() {
    var a int = 10
	var b int = 20

	var p1 *int = &a
	var p2 *int = &b
	var p3 *int = &a

	fmt.Printf("%v\n", p1 == p2) // false
	fmt.Printf("%v", p1 == p3) // true
}
```

이제 아래 코드를 이해할 수 있을 것이다.
  
메모리 주소(&), 역참조(*)만 알면 Go에서 low level의 기본은 완료한 것이다.

```go
package main

import "fmt"

func main() {
	a := 2
	b := &a            // b는 a의 포인터다
	fmt.Println(&a, b) // 당연히 같은 메모리 주소

	*b = 5         // 해당 메모리의 값을 5로 변경
	fmt.Println(a) // 5 출력
}
```

## 포인터 스코프 바깥의 변수 접근

아래 코드에서 main 함수의 스코프에 있는 프린트에는 0이 찍힙니다.
  
40이 찍히게 만들고 싶은데 이를 포인터를 활용해보겠습니다.

```go
func main() {
	x := 0
	foo(x)
	fmt.Println(x) // 0 왜? foo 함수의 스코프 외부이므로
}

func foo(x int) {
	fmt.Println(x)
	x = 40
	fmt.Println(x)
}
```
아래와 같이 짠다면 x의 메모리 주소값 조작을 통해 스코프와 상관없이 40을 띄울 수 있다.

```go
func main() {
	x := 0
	foo(&x)
	fmt.Println(x) // 40
}

func foo(x *int) {
	fmt.Println(*x)
	*x = 40
	fmt.Println(*x)
}

```
