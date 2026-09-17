
nmap 명령어는 네트워크 매핑에서 사용되는 도구이며, 가장 인기 있는 윤리적 해킹 도구중 하나다. 

nmap은 주변 네트워크를 찾는 데 사용되며 네트워크 관리자는 항상 네트워크를 매핑해야하므로 nmap을 가장 유용하게 활용할 수 있다.

nmap을 통해 특정 네트워크 장치를 빠르게 찾을 수 있으며, 모든 웹서비스와 DNS 서버를 포함하는 시스템에서 실행중인 서비스를 찾는데 유용하다. 열린 포트가 있는 장치를 찾는 데 도움을 주며 보안을 강화하기 위해 장치를 살펴볼 수도 있다.

<br>

### Nmap의 기능

- OS 감지: OS, OS Version, 기타 세부 정보를 감지하는 nmap os 스캔을 할 수 있다.
- 서비스 감지: Nmap-services-probe-file에 있는 여러 서비스 프로브는 네트워크 서비스 및 해당 애플리케이션에서 응답을 얻는데 사용된다.
- 호스트 검색: 네트워크 호스트가 tcp, udp 프로토콜을 사용해 네트워크의 다른 호스트에 대한 데이터를 수집하는데 사용된다.
- 대상 사양: 대상 사양 기능을 사용해 nmap에서 스캔하려는 대상 ip 주소를 지정할 수 잇다.
- Ipv6 지원: ipv6는 인터넷 프로토콜 버전 6을 의미하며 네트워크 스캔을 위해 nmap에서 사용할 수 있다.
- NSE 기능: NSE는 Nmap Scription Engine에 약자이며 호스트 검색, 네트워크 스캐닝 및 대상 지정에 사용할 수 있는 Nmap기능을 제공한다.
- TLS/SSL 스캐닝: Nmap의 도움으로 TLS 배포 문제를 빠르게 분석할 수 있다.

<br>

### Gateway 주소 얻기

```bash
route -n get default  
route -n get default | grep 'gateway' | awk '{print $2}'
```


<br>

###  nmap 사용 예시

nmap의 다양한 옵션들은 [nmap 공식 문서](https://nmap.org/book/man.html)에서 확인해볼 수 있다.

```bash
nmap -sP xxx.xxx.xxx.xxx  
- 대상 호스트가 살아있음을 알 수 있다.  
  
namp -sP -PT80 xxx.xxx.xxx.xxx  
- 특정 포트(80)을 검색  
  
nmap -sT xxx.xxx.xxx.xxx  
- 지정된 포트가 아니라 대상호스트의 열린 포트를 모두 검색  
- 대상 호스트의 열린 포트를 알 수는 있지만 로그가 남으므로 위험  
  
nmap -sS xxx.xxx.xxx.xxx  
- 스텔스 스캔으로 감시를 피함  
  
nmap -sU localhost  
- UDP port 스캔입니다. 시간이 많이 걸릴 수도 있다.  
  
nmap -sS -O xxx.xxx.xxx.xxx  
- -O 옵션으로 운영체제를 알아볼 수 있다.  
  
nmap -v xxx.xxx.xxx.xxx  
- 좀 더 자세하게 정보를 보여준다.  
  
nmap xxx.xxx.xxx.xxx/xx  
- xxx.xxx.xxx.xxx/xx 네트워크 전체를 스캔한다.  
  
nmap xxx.xxx.xxx.xxx-xxx  
- 연속되어 있는 여러 개의 호스트를 검색  
  
nmap -O xxx.xxx.xxx.xxx  
- 해당 호스트의 포트 스캔과 운영체제를 보여준다.  
  
namp -sR -p 1-40000 xxx.xxx.xxx.xxx  
- 해당 호스트의 1 ~ 40000 포트에서 RPC 포트를 찾아 보여준다.  
  
nmap -sU -PT xxx.xxx.xxx.xxx/xx  
- 해당 네트워크의 호스트들을 TCP ACK 패킷을 보내어 응답을 기다리고 열려진 UDP 포트를 보여준다.
```

