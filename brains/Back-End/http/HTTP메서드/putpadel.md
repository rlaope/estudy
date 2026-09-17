# PUT, PATCH , DELETE

### PUT
- 리소스를 대체
  - 리소스가 없으면 대체
  - 리소스가 없으면 생성
  - 쉽게 이야기해서 덮어버림
- 중요! 클라이언트가 리소스를 식별
  - 클라이언트가 리소스 위치를 알고 URI 지정
  - POST와 차이점

```
PUT /members/100 HTTP/1.1
Content-Type: application/json

{
	"username": "hello",
	"age": 20
}
```
리소스가 있는 경우1
```
PUT /members/100 HTTP/1.1
Content-Type: application/json

{
	"username": "old",
	"age": 50
}
```
->  
/members/100
```
{
	"username": "hello",
	"age": 20
}
```
=
```
{
	"username": "old",
	"age": 50
}
```

리소스가 없는 경우2
```
PUT /members/100 HTTP/1.1
Content-Type: application/json

{
	"username": "old",
	"age": 50
}
```
->  
/members/100  
이런 리소스 없음  
  
=
```
{
	"username": "old",
	"age": 50
}
```
주의 - 리소스를 완전히 대체한다1
```
PUT /members/100 HTTP/1.1
Content-Type: application/json

{
    "age": 50
}
```
->  
/members/100
```
{
	"username": "hello",
    "age": 20
}
```
=
```
{
    "age": 50
}
```

기존 리소스를 삭제하고 대체해서 username 필드가 삭제됨

<br>

### PATCH
- 리소스 부분 변경

```
PATCH /members/100 HTTP/1.1
Content-Type: application/json

{
    "age": 50
}
```
->  
/members/100
```
{
	"username": "young",
	"age": 20
}
```
=
```
{
	"username": "young",
	"age": 50
}
```
age만 50으로 변경

<br>

### DELETE
- 리소스 제거

```
DELETE /members/100 HTTP/1.1
Host: localhost:8080
```
