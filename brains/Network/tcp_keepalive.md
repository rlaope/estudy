# TCP 세션 유지 Keepalive

**TCP Keepalive는 두 종단 간 맺은 세션을 유지하기 위하여 통신이 일어날 때마다 유지중인 세션을 이용하게 한다.**

> 3 way handshake를 진행한 후 그 다음 요청에도 똑같이 3 way handshake를 진행한다면 리소스 소모가 심할 것이다. 그렇기에 keepalive동안 세션을 유지하여 효율성을 증가시키는 것이다.

keepalive 적용, 사용 방식은 서로의 keepalive 확인을 위한 작은 패킷을 주고 받은 후에 타이머는 다시 원래 값으로 돌아가지고 카운트를 진행하는 메커니즘으로 동작한다.

**위 처럼 주기적으로 두 종단 간에 keepalive를 확인하면서 양쪽의 세션이 끊기지 않고 유지된다.**

```bash
$ netstat -napo | grep -i est
(No info could be read for "-p": geteuid()=1000 but you should be root.)
Active Internet connections (servers and established) 
tcp 0 0 172.31.35.50:22 61.**.2*.7*:51079 ESTABLISHED - keepalive (3684.64/0/0) # 현재 타이머가 3684초 가량 남음
```


<br>

### Keepalive 타이머 확인

```bash
# redis-cli  
127.0.0.1:6379> config set tcp-keepalive 100  
OK  

# redis-cli  
127.0.0.1:6379> config get tcp-keepalive  
1) "tcp-keepalive"  
2) "100"  

$ netstat -napo | grep -i 6379 | grep -i est  
tcp 0 0 172.31.37.40:6379 172.31.35.50:51368 ESTABLISHED 5263/redis-server 0 keepalive (100.00/0/0)
```


syn 패킷 전송 약 100초 후에 keepalive 패킷 확인 가능
```bash
tcpdump -i any -A -vvv -nn port 6379 -w tcp_keepalive.pcap
```

tcp keepalive의 패킷은 `68bytes`정도로 매우 작은 크기다. 그렇기 때문에 종단 간의 세션 유지를 위해 많은 리소스가 필요치 않아 가능하면 켜져 있는 상태로 통신하는 것이 좋다.

<br>

### TCP Keepalive Paramters

```bash
$ sysctl -a | grep -i keepalive  
net.ipv4.tcp_keepalive_intvl = 75  
net.ipv4.tcp_keepalive_probes = 9  
net.ipv4.tcp_keepalive_time = 7200  
  
# 본문과 동일하도록 설정 변경  
$ sysctl -w net.ipv4.tcp_keepalive_time="240"  
net.ipv4.tcp_keepalive_time = 240  
$ sysctl -w net.ipv4.tcp_keepalive_intvl="30"  
net.ipv4.tcp_keepalive_intvl = 30  
$ sysctl -w net.ipv4.tcp_keepalive_probes="3"  
net.ipv4.tcp_keepalive_probes = 3
```

- `net.ipv4.tcp_keepalive.time`: **keepalive 소켓의 유지 시간**으로 타이머는 이 시간을 기준으로 동작하며 이 시간이 지나면 keepalive 확인 패킷을 보낸다.
- `net.ipv4.tcp_keepalive.probes`: **keepalive 패킷을 보낼 최대 전송 횟수**로, 네트워크 패킷은 다양한 원인으로 손실될 수 있으며 이에 따른 재전송 메커니즘이 있다. 하지만 그렇다고 무한정 보낼 수는 없기에 해당 파라미터로 최대 재전송 횟수를 정의한다.
- `net.ipv4.tcp_keepalive_intvl`: **keepalive 재전송 패킷(ack)를 보내는 주기**, 처음에 설정한 tcp_keepalive_time이 지난 이후 keepalive 확인 패킷을 보내게 되는데, 이 패킷이 응답이 없으면 몇 초 후에 재전송 패킷을 보낼 것인지 그 값을 설정한다.

두 종단 간의 연결을 끊기 위해서는 FIN 패킷이 필요한데 양쪽 모두 정상적으로 FIN 패킷을 주고 받아서 연결을 끊는 것이 정상적이지만, 시스템을 운영하다보면 다양한 이슈로 인해 FIN을 주고받지 못하고 끊어지는 경우가 종종 생긴다.

예를 들어서 서버가 연결되어 있는 스위치에 장애가 발생하여 두 종단간에 연결이 끊어지면, FIN을 전달할 방법이 없어 계속해서 연결된 것처럼 남아있게 된다. 하지만 TCP Keepalive 옵션을 사용한다면, 일정 시간이 지난 후에 keepalive 확인 패킷을 보내고, 이에 대한 응답이 없다면 커널이 끊어진 세션으로 판단하고 소켓을 정리한다.

<br>

### Keepalive 좀비 커넥션

의도적으로 iptables를 통해 패킷을 드랍시키도록 설정한 후에 mysql을 종료했을 때 클라이언트의 소켓상태는 ESTABLISHED이지만, 정상적으로 통신한다면, mysqld를 종료하였을 때 소켓 상태는 CLOSE_WAIT로 변한다. 명시적으로 close()를 호출하지 못했기 때문에 CLOSE가 아닌 CLOSE_WAIT 상태로 유지된다.

DB 서버에서 설정된 iptables로 인해 클라이언트는 서버로부터 FIN 패킷을 받지 못했기 때문에 DB 서버와의 연결을 끊어졌는지 모르므로 ESTABLISHED 상태로 남아있지만 이러한 좀비 커넥션도 결국엔 Keepalive 옵션을 통해 타이머가 지나 keepalive 소켓에 대한 응답을 받지 못했기 때문에 소켓이 종료된다.


<br>

### 좀비 커넥션

예를들어 MQ 서버와 로드 밸런서 관련 경험에서는 로드 밸런서의 idle timeout으로 설정된 120초로 인해 120초 동안 패킷이 흐르지 않은 세션은 로드 밸런서의 세션 테이블에서 지워지지만, 두 종단에 로드 밸런서의 **세션 테이블이 지워졌음을 알리는 역할을 하지 않음으로 세션 테이블이 지워졌을 때 문제가 발생한다.**

로드밸런서의 세션 테이블 정보(client ip, port, timeout, session id)가 지워진 상태에서 클라이언트는 로드밸런서에게 요청을 전송하고 로드밸런서는 라운드 로빈 방식으로 로드 밸런서 뒷단에 두 서버에게 요청을 분산하여 되게 운이 좋아서 연결했떤 서버에 연결하게 된다면 문제가 없지만, 만약 다른 서버에 연결하게 되어 요청을 이어서 전달한다면, 새로 연결된 서버 입장에서는 연결도 맺지 않은 클라이언트가 데이터를 쓰겠다고 하니 거부하고 클라이언트는 타임아웃을 경험한다. 이러한 사오항들이 반복되면 서버쪽에서 다량의 좀비 커넥션이 남게되는 문제가 발생한다.

이런 문제를 해결하기 위해서 idle timeout에 걸리지 않도록 keepalive 관련 파라미터들을 수정한다. **로드 밸런서의 idle timeout은 120초였다. 어떤 경우에도 120초 안에 두 종단간에 패킷이 흐르게 하기 위하여 `net.ipv4.tcp_keepalive_time`은 60초 `net.ipv4.tcp_keepalive_probes`를 3으로 `net.ipv4.tcp_keepalive_intvl`은 10초로 설정한다.** 이렇게 설정하면 중간에 패킷이 유실까지 포함해도 120초안에 충분히 체크할 수 있기 때문에 서버 측에 좀비 커넥션이 발생하는 이슈를 막을 수 있다.

- DSR(Direct Server Return): 로드 밸런서 환경에서 서버의 응답 패킷이 로드 밸런서를 통하지 않고 직접 클라이언트로 전달되는 구조
- Inline 구조: 서버로의 요청과 서버에서의 응답 패킷이 모두 로드 밸런서를 거치는 구조

TCP Keepalive 설정을 이용하면 FIN 패킷을 받지 못해 정리되지 않고 남아있는 좀비 커넥션을 없앨 수 있다.

<br>

### TCP Keepalive vs HTTP Keepalive

TCP Keepalive는 두 종단 간의 연결을 유지하기 위함이라면, HTTP Keepalive는 최대한 연결을 유지하기 위하는 것이 목적이다. 본문에서 두 가지 테스트를 통해 TCP Keepalive가 설정되어 있어도 HTTP Keepalive가 설정되어 있다면 해당 설정 값에 맞춰서 동작한다는 것을 보여준다.


<br>

