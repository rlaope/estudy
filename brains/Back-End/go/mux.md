# gorilla/mux를 사용한 REST API 서버

### main

아래 코드에서 rest package를 임포트하고 ServeApi 메서드를 호출한다.

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

호출된 Service 메서드를 살펴보면 Route를 매핑시켜주는 작업을 하는 것을 확인이 가능하다.

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

여기서 .Handler()의 인자로는 http.Handler가 요구되어진다.

![](https://velog.velcdn.com/images/divan/post/c46fb686-2e34-4613-861e-cb7d5167de3a/image.png)

그럼 http.Handler를 어떻게 정의를 확인해보자. http.Handler는 ServeHTTP 메서드를 포함한 interface다 그렇기에 `r.Methods("get").Path("/").Handler(&IndexHandler{})` 에서 IndexHandler는 필수로 ServeHTTP 메서드를 작성해야한다.

![](https://velog.velcdn.com/images/divan/post/9f001114-de28-45ac-970a-7dd7575cfa66/image.png)

아래 코드를 보면 IndexHandler 구조체가 ServeHTTP를 포함하는것을 확인 가능하다.

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