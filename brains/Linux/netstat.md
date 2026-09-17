# netstat

![](https://miro.medium.com/v2/resize:fit:1400/format:webp/1*hsoFihOUE1X6wGDDjPhN3g.png)

서버에 네트워크 연결 정보를 확인할 수 있다.

tcp 3 way handshake, tcp 4 way handshake 진행 과정의 상태가 표현되기에 이를 매우 잘 이해해야한다.

LISTEN, TIME_WAIT, ESTABLISHED 상태를 제일 많이 마주하게 되며 상태별로 다음과 같은 해석이 가능하다.

- LISTEN: 프로세스가 소켓을 통해 요청을 듣고 있는 상태
- ESTABLISHED: 커넥션이 맺어진 상태
- TIME_WAIT: 커넥션이 종료되고 연결을 먼저 끊은쪽(active closer)에서 소켓을 정리하기 전 잠시 대기 하는 상태다
	- 이는 fin 과정 4 way hand shake에서 active closer가 마지막에 ack 패킷을 보내고 나서 패킷이 유실될 경우 다시 패킷을 보내 graceful하게 연결을 끊기 위함이다. (마지막 ack가 유실되면 passive closer는 ack를 못받았으므로 앞서 보낸 Fin을 다시 보내고 active closer는 time wait 상태이므로 소켓을 정리하기전에 fin을 다시 받을 수 있다. 이후 active closer는 ack를 보내 연결을 graceful하게 종료가 가능하다.) 

keepalive-timeout: nginx에 설정값중 하나로 매번 tcp 3 way handshake를 하게 되면 네트워크 성능 저하가 발생하기 때문에 연결을 유지하도록 하는 설정이다.

http/1.0에서는 명시적인 connection: keep-alive 헤더를 추가하는 기법으로 구현되어 있지만 1.1 부터 스펙 자체에 persistence connections 개념이 도입되어 별도 헤더를 추가하지 않아도 된다.

**CLOSE_WAIT**: fin 패킷을 수신하고 ack 요청을 만들기 전 상태로 이 상태가 계속 유지된다면, application 응답을 만들지 못하는 상황이므로 원인 분석 후 조치가 필요하다.

정리하면 netstat 명령을 통해 네트워크 연결 상태를 점검할 수 있다.

커넥션 종단간 ip port 정보등을 확인할 수 있으며 LISTEN, ESTABLISHED, TIME_WAIT등 소켓 상태 확인이 가능하다.

CLOSE_WAIT 상태는 application이 소켓을 정리하지 못하고 응답을 만들어내지 못하는 경우이기 때문에 원인을 확인하고 조치가 필요한 상황으로 해석해야한다.

