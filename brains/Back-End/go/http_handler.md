# Handle, Handler, HandleFunc 이해

golang `http` 패키지에는 Handle, HandleFunc 함수가 있습니다.

```go
func Handler(pattern string, handler Handler)
```

```go
func HandleFunc(pattern string, handler func(ResponseWriter, *Request))
```

`Handle` 함수와 `HandlerFunc` 함수는 pattern에 맞는 handler를 호출합니다.
  
이 두 함수는 동작 방식은 같지만 사용하는 인자가 조금 다릅니다.

## Handle

Handle 함수에는 인자로 Handler가 필요합니다. 
  
Handler는 ServeHTTP를 가지고 있는 interface입니다.

```go
type Handler interface {
    ServeHTTP(ResponseWriter, *Request)
}
```
ServeHTTP 메서드가 구현된 타입만 Handle 함수 두 번째 인자로 사용할 수 있습니다.

```go
type ins struct {}
type upd struct {}
type del struct {}
type sel struct {}

func (i ins) ServeHTTP(w http.ResponseWriter, r *http.Request) {
 // insert
}

func (u upd) ServeHTTP(w http.ResponseWriter, r *http.Request) {
 // update
}

func (d del) ServeHTTP(w http.ResponseWriter, r *http.Request) {
 // delete
}

func (s sel) ServeHTTP(w http.ResponseWriter, r *http.Request) {
 // select
}

func main () {
  var i ins
  var u upd
  var d del
  var s sel

  ...
  http.handle("/insert", i)
  http.handle("/update", u)
  http.handle("/delete", d)
  http.handle("/select", s)
  ...
}
```

이러한 방식은 각 경로에 대한 타입과 ServeHTTP를 작성해야 하므로 경로가 많아질수록 유지보수가 힘들어진다는 문제가 있습니다.
  
HandlerFunc을 사용하면 간단하게 해결할 수 있습니다.

## HandlerFunc

ServeHTTP 메서드를 가진 타입으로 변환이 가능하다면 사용자 정의 함수를 Handle의 두 번째 인자로 넣어줄 수 있습니다.
  
변환하는 방법 HandlerFunc 함수 타입을 사용하면 됩니다.

```go
type HandlerFunc func(ResponseWriter *Request)
```

이 함수 타입 유형과 동일하게 사용자 함수를 정의해야 합니다.

```go
ype database struct {}

func (d database) ins (w http.ResponseWriter, r *http.Request) {
 // insert
}

func (d database) upd (w http.ResponseWriter, r *http.Request) {
 // update
}

func (d database) del (w http.ResponseWriter, r *http.Request) {
 // delete
}

func (d database) sel (w http.ResponseWriter, r *http.Request) {
 // select
}

func main () {
  var d database

  ...
  http.handle("/insert", HandlerFunc(d.ins))
  http.handle("/update", HandlerFunc(d.upd))
  http.handle("/delete", HandlerFunc(d.del))
  http.handle("/select", HandlerFunc(d.sel))
  ...
}
```

HandlerFunc을 사용하니 코드가 간결해지고 가독성 또한 좋아졌습니다.
  
그러면 어떻게 ServeHTTP 메서드가 없는 함수를 HandlerFUnc 타입을 사용하여 Handle 함수의 두 번째 인자로 전달할 수 있었을까요?
  
HandlerFunc 타입은 사용자 정의 함수를 HTTP 핸들러로 사용할 수 있게 해주는 어댑터 역할을 합니다.
  
어떻게 어댑터 역할을 하는지는 아래 HandlerFunc 타입에서 사용하는 ServeHTTP 함수를 보면 알 수 있습니다.

```go
func (f HandlerFunc) ServeHTTP(w ResponseWriter, r *Request) {
    f(w, r)
}
```

위 코드를 보면 ServeHTTP 함수와 HandlerFunc 타입과 연결되어 있음을 확인할 수 있고 여기서 f는 사용자 정의 함수라는 것을 알 수 있습니다.
  
그렇기 때문에 ServeHTTP 함수를 호출하지만 내부에서 HandlerFunc 타입으로 변경된 사용자 정의 함수 f가 호출됩니다.
  
사용자 정의 함수를 HandlerFunc으로 변환하면 내부적으로 사용자 정의 함수를 호출하는 ServeHTTP 메서드를 얻을 수 있습니다.

## HandleFunc

실제로 HandleFunc 함수 내부에는 HandlerFunc 타입을 직접 호출하여 사용자 정의 함수 handler를 전달하는 것을 볼 수 있습니다.

```go
func (mux *ServeMux) HandleFunc(pattern string, handler func(ResponseWriter, *Request)){
    mux.Handle(pattern, HandlerFunc(handler))
}
```

HandleFunc 함수 내부에서 HandlerFunc 타입으로 변환하기 때문에 두 번째 인자로 함수를 넣을 수 있습니다.

```go
type database struct {}

func (d database) ins (w http.ResponseWriter, r *http.Request) {
 // insert
}

func (d database) upd (w http.ResponseWriter, r *http.Request) {
 // update
}

func (d database) del (w http.ResponseWriter, r *http.Request) {
 // delete
}

func (d database) sel (w http.ResponseWriter, r *http.Request) {
 // select
}

func main () {
  var d database

  ...
  http.handleFunc("/insert", d.ins)
  http.handleFunc("/update", d.upd)
  http.handleFunc("/delete", d.del)
  http.handleFunc("/select", d.sel)
  ...
}
```