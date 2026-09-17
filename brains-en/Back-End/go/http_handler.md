# Understanding Handle, Handler, and HandleFunc

The `http` package in golang provides `Handle` and `HandleFunc` functions.

```go
func Handler(pattern string, handler Handler)
```

```go
func HandleFunc(pattern string, handler func(ResponseWriter, *Request))
```

Both the `Handle` function and the `HandleFunc` function call the appropriate handler for a given pattern.
  
While these two functions operate similarly, they differ slightly in the arguments they take.

## Handle

The `Handle` function requires a `Handler` as an argument.
  
A `Handler` is an interface that has `ServeHTTP`.

```go
type Handler interface {
    ServeHTTP(ResponseWriter, *Request)
}
```
Only types that implement the `ServeHTTP` method can be used as the second argument to the `Handle` function.

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

This approach has the drawback that as the number of paths increases, maintenance becomes difficult because you have to write a type and `ServeHTTP` for each path.
  
This can be easily resolved by using `HandlerFunc`.

## HandlerFunc

If a custom function can be converted to a type that has the `ServeHTTP` method, it can be passed as the second argument to `Handle`.
  
The way to convert it is by using the `HandlerFunc` function type.

```go
type HandlerFunc func(ResponseWriter *Request)
```

User-defined functions must be defined to match this function type signature.

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

Using `HandlerFunc` makes the code more concise and improves readability.
  
So, how was it possible to pass a function without a `ServeHTTP` method as the second argument to the `Handle` function using the `HandlerFunc` type?
  
The `HandlerFunc` type acts as an adapter, allowing user-defined functions to be used as HTTP handlers.
  
You can understand how it acts as an adapter by looking at the `ServeHTTP` function used within the `HandlerFunc` type below.

```go
func (f HandlerFunc) ServeHTTP(w ResponseWriter, r *Request) {
    f(w, r)
}
```

Looking at the code above, you can see that the `ServeHTTP` function is connected to the `HandlerFunc` type, and `f` here refers to the user-defined function.
  
Therefore, although the `ServeHTTP` function is called, the user-defined function `f`, which has been converted to the `HandlerFunc` type, is invoked internally.
  
By converting a user-defined function to `HandlerFunc`, you effectively get a `ServeHTTP` method that internally calls your user-defined function.

## HandleFunc

In fact, inside the `HandleFunc` function, you can see that it directly calls the `HandlerFunc` type to pass the user-defined `handler` function.

```go
func (mux *ServeMux) HandleFunc(pattern string, handler func(ResponseWriter, *Request)){
    mux.Handle(pattern, HandlerFunc(handler))
}
```

Because the `HandleFunc` function internally converts to the `HandlerFunc` type, you can pass a function as the second argument.

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
