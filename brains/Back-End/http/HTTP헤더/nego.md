# 협상, 전송 방법

### 클라이언트가 선호하는 표현 요청
- Accept: 클라이언트가 선호하는 미디어 타입 전달
- Accept-Charset: 클라이언트가 선호하는 문자 인코딩
- Accept-Encoding: 클라이언트가 선호하는 압축 인코딩
- Accept-Language: 클라이언트가 선호하는 자연 언어
- 협상 헤더는 요청시에만 사용

한국어 브라우저를 사용하는 클라이언트가 다중 언어 지원하는 서버(1. 기본 영어 2. 한국어)에 요청을 할때 Accept-Language를 적용하지 않으면 기본 영어로 응답 받지만, 적용하면 한국어로 응답 받는다. 하지만 한국어 지원을 안할 경우에 우선순위를 지정할 수 있다.
  
**협상과 우선순위1**

Quality Values(q)

```
GET /event
Accept-language: ko-KR,ko;q=0.9,en-US;q=0.8,en;q=0.7
```

- Quality Value(q) 값 사용
- 0~1, 클ㅅ수록 높은 우선순위
- 생략하면 1
- Accept-Language: ko-KR,ko;q=09,en-US;q=0.8,en;q=0.7
  - 1 ko-KR;q=1(q=1 이면 생략)
  - 2 ko;q = 0.9
  - 3 en-US;q=0.8
  - 4 en;q=0.7

**협상과 우선순위2**
```
GET /event
Accept: text/*, text/plain, text/plain;format=flowed, */*
```

구체적인 것이 우선한다.
- Accept: text/, text/plain, text/plain;format=flowed, /* 
  1. text/plain;format=flowed
  2. text/plain
  3. text/*
  4. /

**협상과 우선순위3**

Quality Values(q)
- 구체적인 것을 기준으로 미디어 타입을 맞춘다
- Accept: text/;q=0.3, text/html;q=0.7, text/html;level=1, text/html;level=2;q=0.4, /*;q=0.5

<br>

## 전송 방식

- Transfer-Encoding
- Range, Content-Range

### 전송 방식 설명
- 단순 전송: Content-Length

```
HTTP/1.1 200 OK
Content-Type: text/html;charset=UTF-8
Content-Length: 3423
```

- 압축 전송: Content-Encoding

```
HTTP/1.1 200 OK
Content-Type: text/html;charset=UTF-8
Content-Encoding: gzip
Content-Length: 3423
```

- 분할 전송: Transfer-Encoding

```
HTTP/1.1 200 OK
Content-Type: text/plain
Transfer-Encoding: chunked

5
hEllo
5
World
0
\r\n
```

- 범위 전송: Range, Content-Range
```
GET /event
Range: bytes=1001-2000
```

```
HTTP/1.1 200 OK
Content-Type: text/plain
Content-Range: bytes 1001-2000 / 2000

afdadfdsfass
```