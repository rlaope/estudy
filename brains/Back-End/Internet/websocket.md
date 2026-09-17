# 웹소켓(WebSocket), HTTP와의 차이

## 웹소켓 프로토콜
클라이언트와 서버(브라우저와 서버)를 연결하고 **실시간으로 통신이 가능하도록 하는 첨단 통신 프로토콜이다.**
  
웹소켓은 하나의 TCP 접속에 전이중(dupex) 통신 채널을 제공한다.
  
쉽게 말해, 웹소켓은 Socket Connection을 유지한 채로 실시간으로 양방향 통신 혹은 데이터 전송이 가능한 프로토콜이다.
  
오늘날 채팅 어플레케이션, SNS, 구글 Docs, LOL 같은 멀티플레이 게임, 화상회의 등 많은 분야에서 사용되고 있다.

## HTTP와의 차이

기존 HTTP는 단방향 통신이였다. 
  
클라이언트에서 서버로 Request를 보내면 서버는 클라이언트로 Response를 보내는 방식으로 동작했다.
  
또한, HTTP는 기본적으로 무상태(Stateless)이므로 상태를 저장하지 않는다.
  
하지만 웹소켓은 양방향 통신으로 연결이 이루어지면 클라이언트가 요청하지 않아도 데이터가 저절로 서버로부터 올 수 있다.
  
HTTP처럼 별도의 요청을 보내지 않아도 데이터를 수신할 수 있다는 것이다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FBCkpf%2Fbtr4fVn5KF1%2FTh5ZK8vN5wfKMZE4SwIs11%2Fimg.png)

예를 들어, 웹 상에서 구글 Docs를 이용해 여러 사용자가 동시에 한 문서를 편집하고 있다고 하자.
  
구글 Docs를 사용해 본 유저는 알겠지만, 새로고침을 누르지 않아도 실시간으로 다른 사용자가 편집한 부분이 자동으로 적용되는 모습을 풀 수 있다.
  
이는 WebSocket을 이용한 기술이다.
  
또한, 웹소켓은 HTTP와 다르게 상태(Stateful) 프로토콜이다. 즉, 클라이언트와 서버가 한 번 연결되면 같은 연결을 이용해 통신하므로 TCP 커넥션 비용을 아낄 수 있다.

## 웹소켓의 동작

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbAXq5j%2Fbtr4hmk7b8i%2FqYt4Dhq7ThXCxKJpSIKSzK%2Fimg.png)

웹소켓은 HTTP 포트 80, HTTPS포트 443 위에서 동작한다.
  
웹소켓은 TCP연결 처럼 핸드셰이크를 이용해 연결을 맺는다.
  
이때 HTTP 업그레이드 헤더를 사용하여 HTTP 프로토콜에서 웹소켓 프로토콜로 변경한다.
  
즉, 최초 접속시에는 HTTP 프로토콜을 이용해 핸드셰이킹을 한다.
  
이후 연결이 맺어지면 어느 한쪽이 연결을 끊지 않는 이상 영구적인(persistent) 동일한 채널이 맺어지고, HTTP 프로토콜이 웹소켓 프로토콜로 변경된다.
  
이때 데이터를 암호화하기 위해 WSS 프로토콜 등을 이용할 수도 있다.