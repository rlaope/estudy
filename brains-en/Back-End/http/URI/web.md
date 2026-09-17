# Web Browser Request Flow
https://www.google.com:443/search?q=hello&hl=ko
DNS lookup, HTTPS PORT omitted, 433
Web browser generates an HTTP request message
```
HTTP 요청 메세지  
GET/search?q=hello&hl=ko HTTP/1.1
Host: www.google.com
```
1. Web browser generates an HTTP message
2. Transmitted via SOCKET library
  - A: TCP/IP connection (IP, PORT)
  - B: Data transmission
3. **TCP/IP packet generated**, includes HTTP message data
4. HTTP message sent to the server via the internet using the LAN card

The Google server discards the TCP/IP packet and interprets only the HTTP message.

```
HTTP 응답 메시지

HTTP/1.1 200 OK
Content-Type : text/html;charset=UTF-8
Content-Length: 3423

<html>
  <body> ... <body>
</html>
```
The client's web browser renders the HTTP response message to display the result.
