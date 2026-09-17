# Go 데이터베이스 Sqlite3 사용하기

Go 에는 database/sql 패키지를 지원한다.
  
그래서 다른 DB들도 이 함수로 커버가 가능하다.
  
간단하게 Sqlite를 연동해 사용하는 프로그램을 작성해보겠다.

## 파일 분할 database.go

데이터베이스의 원활한 사용을 위해 database.go 파일을 만들어준다. 그리고, 아래 코드를 추가한다.

```go
package main

import(
    "database/sql"
    "errors"
    "log"

    _ "github.com/mattn/go-sqlite3"
)
```

Sqlite 데이터베이스 파일을 열고, 테이블을 생성하는 함수를 작성하겠습니다.

```go
func InitDB(file string) (*sql.DB, error) {
    db, err := sql.Open("sqlite3", file)

    if err != nil {
        return nil, err
    }

    createTableQuery := `
    create table IF NOT EXISTS useraccount (
        id integer PRIMARY KEY autoincrement,
        userId text,
        password text,
        UNIQUE (id, userId)
    )
    `
    _, e := db.Exec(db, createTableQuery)

    if e != nil {
        return nil, e
    }

    return db, nil

}
```

데이터베이스를 열려면 sql.Open 함수를 사용하여 데이터베이스 종류를 적은 뒤, 파일 이름을 적으려면 열려지고 반환은 데이터베이스 포인터가 반환된다.
  
그리고 db.Exec 함수를 통해 쿼리를 적용할 수 있다.
  
다음은, 테이블에 데이터를 삽입하는 함수를 만들어보겠습니다.
  
프리페어를 통해 변수들을 삽입 쿼리에 적용하는데 기존의 스트링으로 추가하면 제대로 작동이 안되니 Prepare 함수를 잘 활용해야겠습니다.

```go
func AddUser(db *sql.DB, id string, password string) error {
    tx, _ := db.Begin()
    stmt, _ := tx.Prepare("insert into useraccount (userId, password) values (?,?)")

    _, err := stmt.Exec(id, password)

    if err != nil {
        log.Println(err.Error())
        return err
    }

    tx.Commit()
    return nil
}
```

이렇게 쿼리 작업중에 생기는 에러는 패닉되지 않기 때문에 잘 처리 해주는것이 중요합니다.
  
디버깅때는 에러 로그를 남겨서 빠르게 찾도록 합시다.
  
Select 쿼리를 사용해 값을 받아오도록 하겠습니다.
  
쿼리 결과의 Scan으로 값을 복사합니다.

```go
func GetUser(db *sql.DB, userId string) (User, error) {
    var user User
    rows := db.QueryRow("select * from useraccount where userId = $1", userId)
    err := rows.Scan(&user.id, &user.UserId, &user.Password)

    if err != nil {
        return User{}, err
    }

    return user, nil
}
```

QueryRow 함수를 사용하면 검색되는 한개의 열만 가져오고, 존재하지 않을시에 에러를 리턴함으로써 간편하게 사용할 수 있습니다.
  
main.go에서 아래와 같이 사용해주시면 될 것 같습니다.

```go
package main

func main() {
    db, err := InitDB(".hello.db")
    if err != nil {
        log.Fatal(err)
    }
    
}
```