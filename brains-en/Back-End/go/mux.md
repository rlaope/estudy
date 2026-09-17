# REST API Server using gorilla/mux

### main

In the code below, the `rest` package is imported and the `ServeAPI` method is called.

```go
package main
import (
    "fmt"
    "github.com/esperer/golang_mq/rest"
)

func main() {
    rest.ServeAPI("127.0.0.1:8888")
    fmt.Println("REST API SERVER START")
}
```

Looking at the called `ServeAPI` method, it can be seen that it performs the task of mapping routes.

```go
package rest

import (
    "fmt"
    "net/http"
    "time"
    "github/gorilla/mux"
)

func ServeAPI(listneAddr string) {
    r := mux.NewRouter()
    r.Methods("get").Path("/").Handler(&IndexHandler{})
    r.Methods("get").Path("/event/{eventId}/booking").Handler(&CreateBookingHandler{})

    src := http.Server{
        Handler: r,
        Addr: listenAddr,
        WriteTimeout: 2 * time.Second,
        ReadTimeout: 1 * time.Second
    }

    err := srv.ListenAndServe()

    if err != nil {
        fmt.Println("err : ", err)
    }
}

```

Here, `http.Handler` is required as an argument for `.Handler()`.

![](https://velog.velcdn.com/images/divan/post/c46fb686-2e34-4613-861e-cb7d5167de3a/image.png)

Let's check how `http.Handler` is defined. `http.Handler` is an interface that includes the `ServeHTTP` method. Therefore, in `r.Methods("get").Path("/").Handler(&IndexHandler{})`, `IndexHandler` must implement the `ServeHTTP` method.

![](https://velog.velcdn.com/images/divan/post/9f001114-de28-45ac-970a-7dd7575cfa66/image.png)

Looking at the code below, it can be seen that the `IndexHandler` struct includes `ServeHTTP`.

```go
package rest
import (
	"net/http"
)
type IndexHandler struct {
	 
}
func (h *IndexHandler) ServeHTTP(w http.ResponseWriter, r *http.Request){
	w.Header().Set("Content-Type", "application/json")
    w.WriteHeader(http.StatusOK)
    w.Write( []byte("Server is running "))
}
```
