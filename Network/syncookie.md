# SYN Cookie

SYN Cookie는 SYN Flood(syn요청막갈겨서 부하주는) 공격으로부터 서버를 보호하기 위한 커널이 상태를 저장하지 않고 tcp 연결을 유지하는 기법이다.

tcp는 syn을 받으면 syn-ack를 받을때 식별하기위해 syn queue에 해당 conn을 저장해두고 있는데, 문제는 syn flood공격을 갈기면, 해당 큐에 많은 conn들이 쌓여 부하가 걸릴수있다는것이다.

물론 공격 뿐만 아니라 대용량 트래픽에서 밀릴때도 있지만 어쨌거나 그건 다르게 해결하면 되는 문제고, syn flood 공격은 막아야한다.

1. client의 syn 수신
2. syn queue에 상태 저장
   1. ISN(Initial Sequence Number)
   2. MSS, Window Scale
   3. Timer
3. SYN-ACK 전송
4. ACK 오면 상태 매칭 -> ESTABLISHED

문제는 syn flood인데, 공격자는 syn만 계쏙 보내서 ack를 보내지 않고 syn queue가 가득차게 된다. 이러면 정상 클라이언트도 연결수립에 실패하게됨.

이때 필요한게 syn cookie 핵심 아이디어는 SYN queue에 아무것도 저장안함

대신 서버가 보낼 때 SYN-ACK에 SEQ 번호 안에 이 SYN이 진짜였는지를 검증할 수 있는 정보를 암호화해서 넣는다.

즉 상태 저장 -> 상태를 숫자 하나에 인코딩하는 것.

### 동작 흐름

클라 -> 서버

```
SYN (client_isn)
```

서버는 아무것도 하지 않음

서버 -> 클라

```
SYN-ACK
SEQ = SYN_COOKIE(...)
```

이 seq값 안에 들어가있는 정보는 client ip port server ip port, time slot(time window), secret key(커널 내부) 값들이 들어있고 해시함수로 생성된다.

클라이언트 -> 서버

```
ACK
ACK = SEQ + 1
```

ACK 번호를 보고 서버는 다시 SYN Cookie를 계산한다. 그리고 값이 일치하면

이 클라이언트는 syn 클라구나를 식별한다. 엑세스토큰같은 느낌의 매커니즘하고 비슷하다고 보면된다.

그제서야 연결 상태를 생성하고 accept queue로 보낸다.

| 공격       | 결과                |
| -------- | ----------------- |
| SYN만 보냄  | 상태 저장 안 하므로 영향 없음 |
| ACK 안 보냄 | 서버는 아무 손해 없음      |
| IP 스푸핑   | ACK 못 받음 → 무효     |


### 제약

하지만 syn cookie는 만능이 아닌게 tcp 옵션 손실이라는 제약이 있음

쿠키 안에 담을 수 있는 비트가 제한되어있기 때문에 보통 window scale, sack permmited, 일부 mss 정보가 제한되어있어 성능이 저하될 수있고 대역폭 큰 연결엔 분리함

정상 트래픽에서도 tcp 성능이 하락하고 rtt증가에 혼잡 제어 최적화를 손실당한다.

그래소 syn queue가 가득 찼을때만 syn cookie를 사용하도록 리눅스에는 설정되어 있다.

```
net.ipv4.tcp_syncookies = 1
```

의미: 평소: SYN Queue 사용, Queue overflow 발생 시: SYN Cookie 활성화

2로 설정하면 항상 사용하지만 권장하지 않음

1이 사용같지만 아닌게 킥

