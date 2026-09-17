# 웹 브라우저 요청 흐름
https://www.google.com:443/search?q=hello&hl=ko  
DNS조회 , HTTPS PORT 생략, 433  
웹 브라우저에서 HTTP 요청 메시지를 생성  
```
HTTP 요청 메세지  
GET/search?q=hello&hl=ko HTTP/1.1
Host: www.google.com
```
1. 웹 브라우저가 http 메시지 생성
2. SOCKET 라이브러리 통해 전달
  - A:TCP/IP 연결(IP , PORT)
  - B: 데이터 전달
3. **TCP/IP 패킷 생성** , HTTP 메시지 데이터 포함
4. LAN카드로 인터넷을 통해 HTTP 메시지를 서버에 전송

구글 서버에서는 TCP/IP 패킷은 까서 버리고 HTTP  메시지만 해석을 한다.  

```
HTTP 응답 메시지

HTTP/1.1 200 OK
Content-Type : text/html;charset=UTF-8
Content-Length: 3423

<html>
  <body> ... <body>
</html>
```
클라이언트의 웹 브라우저가 http 응답 메시지를 렌더링 해서 결과를 볼 수 있다.