# 3xx - 리다이렉션

### 3xx (Redirection)
요청을 완료하기 위해 유저 에이전트의 추가 조치 필요
- 300 Multiple choices
- 301 Moved Permanently
- 302 Found
- 303 See Other
- 304 Not Modified
- 307 Temporary Redirect
- 308 Permanent Redirect

### 리다이렉션 이해
- 웹 브라우저는 3xx 응답의 결과에 Location 헤더가 있으면, Location위초로 자동 이동(리다이렉트)

자동 리다이렉트 흐름
1. 요청 Client -> Server
```
GET /event HTTP/1.1
Host: localhost:8080
```
2. 응답 -> Server -> Client

```
HTTP/1.1 301 Moved Permanently
Location: /new-event
```

3. 자동 리다이렉트 Client -> Client
4. 요청 Client -> Server
```
GET /new-event HTTP/1.1
Host: localhost:8080
```
5. 응답 -> Client
```
HTTP/1.1 200 OK
```

<br>

### 종류
- 영구 리다이렉션 - 특정 리소스의 URI가 영구적으로 이동
  - 예) /members -> /users
  - /event -> /new-event
- 일시 리다이렉션 - 일시적인 변경
  - 주문 완료 후 주문 내역 화면으로 이동
  - PRG : Post/Redirect/Get
- 특수 리다이렉션
  - 결과 대신 캐시를 사용

### 영구 리다이렉션

#### 301 , 308
- 리소스의 URI가 영구적으로 이동
- 원래의 URL을 사용x, 검색 엔진 등에서도 변경 인지
- 301 Moved Permanently
  - 리다이렉트시 요청 메서드가 GET으로 변하고, 본문이 제거될 수 있음(MAY)
- 308 Permanent Redirect
  - 301과 기능은 같음
  - 리다이렉트시 요청 메서드와 본문 유지(처음 POST를 보내면 리다이렉트 유지)

### 영구 리다이렉션 - 301

1. 요청 Client -> Server, POST 사용, 메시지 존재

```
POST /event HTTP/1.1
Host: localhost:8080

name=hello&ange=20
```
2. 응답 Server -> Client

```
HTTP/1.1 301 Moved Permanently
Location: /new-event
```

3. 자동 리다이렉트 Client -> Client
4. 요청 Client -> Server, GET으로 변경, 메시지 제거

```
GET /new-event HTTP/1.1
Host: localhost:8080
```

5. 응답 Server -> Client
  
```
HTTP/1.1 200 OK
```

<br>

### 영구 리다이렉션 - 308

1. 요청 Client -> Server, POST 사용, 메시지 존재
```
POST /event HTTP/1.1
Host: localhost:8080

name=hello&age=20
```
2. 응답 Server -> Client

```
HTTP/1.1 308 Permanent Redirect
Location: /new-eve
```
3. 자동 리다이렉트 Client -> Client
4. 요청 Client -> Server

```
POST /new-event HTTP/1.1
Host: localhost:8080

name=hello&age=20
```
5. 응답 Server -> Client

```
HTTP/1.1 200 OK
```

<br>

### 일시적인 리다이렉션 302, 307, 303

- 리소스의 URI가 일시적으로 변경
- 따라서 검색 엔진 등에서 URL을 변경하면 안됨
- 302 Found
  - 리다이렉트시 요청 메서드가 GET으로 변하고, 본문이 제거될 수 있음(MAY)
- 307 Temporary Redirect
  - 302와 기능은 같음
  - 리다이렉트시 요청 메서드와 본문 유지(요청 메서드를 변경하면 안된다. MUST NOt)
- 303 See Other
  - 302와 기능은 같음
  - 리다이렉트시 요청 메서드가 GET으로 변경

### PRG Post/Redirect/Get
**일시적인 리다이렉션 - 예시**
- POST로 주문 후 웹 브라우저를 새로 고침하면?
- 새로고침은 다시 요청
- 중복 주문이 될 수 있다.

### PRG 사용전
1. 요청

```
POST /order HTTP/1.1
Host: localhost:8080

itemId=mouse&count=1
```
2. 주문 데이터 저장 mouse 1개
3. 응답

```
HTTP/1.1 200 OK

<html>주문완료</html>
```
4. 결과 화면에서 새로고침
5. 요청

```
POST /order HTTP/1.1
Host: localhost:8080

itemId=mouse&count=1
```

6. 주문 데이터 저장 mouse 1개
7. 응답

```
HTTP/1.1 200 OK

<html>주문완료</html>
```

### 일시적인 리다이렉션 - 예시
- POST로 주문후에 새로 고침으로 인한 중복 주문 방지
- POST로 주문후에 주문 결과 화면을 GET 메서드로 리다이렉트
- 새로고침해도 결과 화면을 GET으로 조회
- 중복 주문 대신에 결과 화면만 GET으로 다시 요청

1. 요청
```
POST /order HTTP/1.1
Host: localhost:8080

itemId=mouse&count=1
```

2. 주문 데이터 저장 mouse 1개
3. 응답

```
HTTP/1.1 302 Found
Location: /order-result/19
```

4. 결과 화면에서 새로고침
5. 요청

```
GET /order-result/19 HTTP/1.1
Host: localhost:8080
```

6. 주문데이터 조회 19번 주문
7. 응답

```
HTTP/1.1 200 OK	

<html>주문완료</html>
```
8. 결과 화면에서 새로고침
   - GET /order-result/19
   - 결과 화면만 다시 요청(5번으로 이동)

- PRG 이후 리다이렉트
  - URL이 이미 POST -> GET으로 리다이렉트 됨
  - 새로 고침 해도 GET으로 결과 화면만 조회

<br>

### 그래서 뭘 써야 하나요?

**302, 307, 303**

- 잠깐 정리
  - 302 Found -> GET으로 변할 수 있음
  - 307 Temporary Redirect -> 메서드가 변하면 안됨
  - 303 See Other -> 메서드가 GET으로 변경
- 역사
  - 처음 302 스펙의 의도는 HTTP 메서드를 유지하는 것
  - 그런데 웹 브라우저들이 대부분 GET으로 바꾸어 버림(일부는 다르게 동작)
  - 그래서 모호한 302를 대신하는 명확한 307, 303이 등장함(301 대용으로도 308도 등장)
- 현실
  - 307, 303을 권장하지만 현실적으로 이미 많은 애플리케이션 라이브러리들이 302를 기본값으로 사용
  - 자동 리다이렉션시에 GET으로 변해도 되면 그냥 302를 사용해도 큰 문제는 없음

<br>

### 기타 리다이렉션
- 300 Multiple Choice: 안쓴다.
- 304 Not Modified
  - 캐시를 목적으로 사용
  - 클라이언트에게 리소스가 수정되지 않았음을 알려준다. 따라서 클라이언트는 로컬 PC에 저장된 캐시로 재사용한다.(캐시로 리다이렉트 한다.)
  - 304 응답은 응답에 미시지 바디를 포함하면 안된다.(로컬 캐시를 사용해야 하므로)
  - 조건부 GET , HEAD 요청시 사용