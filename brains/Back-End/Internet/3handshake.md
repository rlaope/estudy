# 3-way / 4-way Handshake

## 3-way Handshake

TCP는 장치들 사이에 논리적인 접속을 성립하기 위하여 3-way handshake를 쇼앙한다.

**TCP 3 Way Handshake는 TCP/IP 프로토콜을 이용해서 통신을 하는 응용 프로그램이 데이터를 전송하기 전에 먼저 정확한 전송을 보장하기 위해 상대방의 컴퓨터와 사전에 세션을 수립하는 과정을 의미한다.**

- Client -> Server: TCP SYN  
- Server -> Client: TCP SYN, ACK
- Client -> Server: TCP ACK


> SYN: synchronized sequence numbers  
> ACK: Acknowlegment

이러한 절차는 TCP 접속을 성공적으로 성립하기 위하여 반드시 필요하다.

<br>

## TCP 3-way Handshaking의 역할

양쪽 모두 데이터 전송을 할 준비가 되었다는 것을 보장하고, 실제로 데이터를 전달이 시작하기 전에 한쪽이 다른 쪽에게 준비가 되었다는 것을 알려주는 역할이다.

양쪽 모두 상대편에 대한 초기 순차일련번호를 얻을 수 있도록 한다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbNwPCT%2FbtqD0hCftBa%2F4fUpGdt1ddNBtk9RGmfKw0%2Fimg.png)

### 과정

1. 클라이언트는 서버에 접속을 요청하는 SYN 패킷을 보낸다. 이때 클라이언트는 SYN을 보내고 SYN/ACK 응답을 기다리는 SYN_SENT 상태, 서버는 Wait for Client 상태다.

2. 서버는 SYN 요청을 받고 클라이언트에게 요청을 수락한다는 ACK와 SYN flag가 설정된 패킷을 발송하고 클라이언트가 다시 ACK로 응답하기를 기다린다. 이때 서버는 SYN_RECEIVED 상태가 된다.

3. 클라이언트는 서버에게 ACK를 보내고 이후로부터 연결이 이루어지고 데이터가 오간다. 이때의 서버 상태가 Established 즉, 성립이다.

<br>

## 4-way Handshake

3-way handshake는 TCP의 연결을 초기화 할 때 사용한다면.

4-way handshake는 **세션을 종료하기 위해 수행되는 절차다.**

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FqUXSw%2FbtqDWsFNWJw%2FhVdKIneSYb7UK3wc0pj6Z0%2Fimg.png)

### 과정

1. 클라이언트가 연결을 종료하겠다는 FIN 플래그를 전송한다. 이때 클라이언트는 FIN-WAIT 상태가 된다.

2. 서버는 FIN플래그를 받고, 일단 확인메시지 ACK를 보내고 자신의 통신이 끝날때까지 기다리는데 이 상태가 서버의 CLOSE_WAIT 상태다.

3. 연결을 종료할 준비가 되면, 연결해지를 위한 준비가 되었음을 알리기 위해 클라이언트에 FIN 플래그를 전송한다. 이때 서버의 상태는 LAST_ACK이다.

4. 클라이언트는 해지 준비가 되었다는 ACK를 확인했다는 메시지를 보낸다. 클라이언트의 상태가 FIN-WAIT -> TIME-WAIT으로 변경된다.

<br>

여기서 만약, 서버에서 FIN을 전송하기 전에 전송한 패킷이 라우팅 지연이나 패킷 유실로 인한 재전송 등으로 인해 FIN 패킷보다 늦게 도착한다면.. 어떻게 될까?

클라이언트에서 세션을 종료시킨 후 뒤늦게 도착하는 패킷이 있다면 패킷은 Drop되고 데이터는 유실된다.

클라이언트는 이러한 현상에 대비해 서버로부터 FIN을 수신하더라도 일정시간(default:240s) 동안 세션을 남겨놓고 잉여 패킷을 기다리는 과정을 거치게 되는데 이 과정을 TIME_WAIT이라고 한다.

일정시간이 지나면, 세션을 만료하고 연결을 종료시키며 CLOSE 상태로 변화한다.