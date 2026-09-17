# Negotiation, Transfer Methods

### Client's Preferred Representation Request
- Accept: Delivers the client's preferred media type
- Accept-Charset: Client's preferred character encoding
- Accept-Encoding: Client's preferred compression encoding
- Accept-Language: Client's preferred natural language
- Negotiation headers are only used in requests

When a client using a Korean browser makes a request to a server that supports multiple languages (1. default English, 2. Korean), if Accept-Language is not applied, it receives a response in default English. However, if it is applied, it receives a response in Korean. If Korean is not supported, however, a priority can be specified.
  
**Negotiation and Priority 1**

Quality Values(q)

```
GET /event
Accept-language: ko-KR,ko;q=0.9,en-US;q=0.8,en;q=0.7
```

- Uses Quality Value (q)
- 0~1, higher value means higher priority
- If omitted, it's 1
- Accept-Language: ko-KR,ko;q=09,en-US;q=0.8,en;q=0.7
  - 1 ko-KR;q=1 (omitted if q=1)
  - 2 ko;q = 0.9
  - 3 en-US;q=0.8
  - 4 en;q=0.7

**Negotiation and Priority 2**
```
GET /event
Accept: text/*, text/plain, text/plain;format=flowed, */*
```

More specific items take precedence.
- Accept: text/, text/plain, text/plain;format=flowed, /*
  1. text/plain;format=flowed
  2. text/plain
  3. text/*
  4. /

**Negotiation and Priority 3**

Quality Values(q)
- Matches media types based on specificity.
- Accept: text/;q=0.3, text/html;q=0.7, text/html;level=1, text/html;level=2;q=0.4, /*;q=0.5

<br>

## Transfer Methods

- Transfer-Encoding
- Range, Content-Range

### Explanation of Transfer Methods
- Simple Transfer: Content-Length

```
HTTP/1.1 200 OK
Content-Type: text/html;charset=UTF-8
Content-Length: 3423
```

- Compressed Transfer: Content-Encoding

```
HTTP/1.1 200 OK
Content-Type: text/html;charset=UTF-8
Content-Encoding: gzip
Content-Length: 3423
```

- Chunked Transfer: Transfer-Encoding

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

- Range Transfer: Range, Content-Range
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
