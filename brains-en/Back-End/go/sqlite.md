# Using Sqlite3 with Go Databases

Go supports the database/sql package.

This allows it to cover other databases as well.

I will now write a simple program that integrates and uses Sqlite.

## File Separation: database.go

To facilitate smooth database usage, create a database.go file. Then, add the following code.

```go
package main

import(
    "database/sql"
    "errors"
    "log"

    _ "github.com/mattn/go-sqlite3"
)
```

I will now write a function to open an Sqlite database file and create a table.

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

To open a database, use the sql.Open function, specify the database type, and then the file name. This will open the database and return a database pointer.

You can then apply queries using the db.Exec function.

Next, let's create a function to insert data into the table.

When applying variables to an insert query via prepared statements, simply adding them as strings won't work correctly, so it's important to make good use of the Prepare function.

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

It's important to handle errors that occur during query operations properly, as they do not cause a panic.

During debugging, leave error logs to quickly identify issues.

Now, let's retrieve values using a Select query.

Copy the values using Scan on the query result.

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

Using the QueryRow function allows you to retrieve only a single row that matches the search, and it returns an error if no such row exists, making it convenient to use.

You can use it in main.go as shown below.

```go
package main

func main() {
    db, err := InitDB(".hello.db")
    if err != nil {
        log.Fatal(err)
    }
    
}
```
