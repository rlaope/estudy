# HTTP 헤더 개요

### HTTP 헤더
- header-field = field-name ":" OWS field-value OWS(OWS: 띄어쓰기 허용)
- field-name은 대소문자 구분 없음

```
GET /search?q=hello&hl=ko HTTP/1.1
Host: www.google.com
```
```
HTTP/1.1 200 OK
Content-Type: text/html;charset=UTF-8
Content-Length: 3423

<html>
 <body>...</body>
</html>
```

### 용도
- HTTP 전송에 필요한 모든 부가정보
- 예) 메시지 바디의 내용, 메시지 바디의 크기, 압충, 인증, 요청 클라이언트, 서버 정보, 캐시 관리 정보...
- 표준 헤더가 너무 많음
- 필요시 임의의 헤더 추가 가능
  - helloworld: hihi

### 분류 - RFC2616(과거)

- 헤더 분류
  - General 헤더 : 메시지 전체에 적용되는 정보, 예) Connection: close
  - Request 헤더: 요청 정보 예) User-Agent : Mozilla/5.0 (Macintosh;..)
  - Response 헤더: 응답 정보 예) Server: Apache
  - Entity 헤더 : 엔티티 바디 정보, 예 ) Content-Type: text/html, Content-Length: 3423


<br>

### HTTP BODY
**message body - RFC2616(과거)**
- 메시지 본문(message body)은 엔티티 본문(entity body)을 전달하는데 사용
- 엔티티 본문은 요청이나 응답에서 전달할 실제 데이터
- 엔티티 헤더는 엔티티 본문의 데이터를 해석할 수 있는 정보 제공
  - 데이터 유형(html, json), 데이터, 길이 압축 정보 등등

### HTTP 표준
1999년 FRC -> 폐기됨
2014년 RFC7230 ~ 7235 emdwkd

### RFC723x 변화
- 엔티티 -> 표현(Representation)
- Representation = representation Metadata + Representation Data
- 표현 = 표현 메타데이터 + 표현 데이터

### message body - RFC7230
- 메시지 본문을 통해 표현 데이터 전달
- 페이지 본문 = 페이로드
- 표현은 요청이나 응답에서 전달할 실제 데이터
- 표현 헤더는 표현 데이터를 해석할 수 있는 정보 제공
  - 데이터 유형(html, json), 데이터 길이, 압충 정보 등등
- 참고: 표현 헤더는 표현 메타데이터와 페이로드 메시지를 구분해야 하지만, 여기서는 표현헤더라고 말함

<br>

## 표현
- Content-Type : 표현 데이터의 형식
- Content-Encoding : 표현 데이터의 압축 방식
- Content-Language : 표현 데이터의 자연 언어
- Content-Length : 표현 데이터의 길이
- 표현 헤더는 전송, 응답 둘다 사용

### Content-Type
**표현 데이터의 형식 설명**
- 미디어 타입, 문자 인코딩
- 예) text/html; charset=utf-8 , application/json , image/png


### Content-Encoding
**표현 데이터 인코딩**
- 표현 데이터를 압축하기 위해 사용
- 데이터를 전달하는 곳에서 압축 후 인코딩 헤더 추가
- 데이터를 읽는 쪽에서 인코딩 헤더의 정보로 압축 해제
- 예)
  - gzip(압축)
  - deflate
  - identity(원본)

### Content-Language
**표현 데이터의 자연 언어**
- 표현 데이터의 자연 언어를 표현
- 예) 
  - ko
  - en
  - en-US

### Content-Length
**표현 데이터의 길이**
- 바이트 단위
- Transfer-Encoding(전송 코딩)을 사용하면 Content-Length를 사용하면 안됨


