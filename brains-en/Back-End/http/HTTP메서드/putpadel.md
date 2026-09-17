# PUT, PATCH , DELETE

### PUT
- Replaces a resource
  - If the resource exists, it replaces it.
  - If the resource does not exist, it creates it.
  - Simply put, it overwrites it.
- Important! The client identifies the resource.
  - The client knows the resource's location and specifies the URI.
  - Difference from POST

```
PUT /members/100 HTTP/1.1
Content-Type: application/json

{
	"username": "hello",
	"age": 20
}
```
Case 1: Resource exists
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

Case 2: Resource does not exist
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
No such resource  
  
=
```
{
	"username": "old",
	"age": 50
}
```
Caution - Completely replaces the resource
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

The existing resource is deleted and replaced, so the username field is removed.

<br>

### PATCH
- Partially modifies a resource

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
Only age is changed to 50.

<br>

### DELETE
- Removes a resource

```
DELETE /members/100 HTTP/1.1
Host: localhost:8080
```
