# UDS, 0000.sock

### UDS

UDS(Unix Domain Socket)은 IPC Socket 이라고도 하며, tcp의 소켓과 동일한 api로 데이터를 주고받을 수 있는 local file 기반의 소켓이다.

TCP socket과의 차이점이라 함은 localhost의 process 간의 통신이므로 속도가 매우 빠르고 메모리 소모가 적다는 장점이 있다.

uds를 사용할 때 또 하나의 장점은 socket을 생성할 때, sockaddr_un 이라는 구조체를 사용하는 것 외에는 일반적인 소켓을 다루는 것과 동일하므로 UDS로 구성해 놓은 후에 성능이나 보안상의 문제로 서버를 클라이언트와 분리하기 매우 용이하다는 점이다.

그리고 UDS의 대표적인 사용 예로는 mysql의 socket인 mysql.sock이 있다.

<br>

### UDS 확인

ls -I option으로 uds를 지정하면, 맨 앞에 s가 붙는 것을 확인할 수 있다.

```bash
$ ls -l /var/lib/mysql/mysql.sock

srwxrwxrwx. 1 mysql mysql 0 Jul 19 10:20 /var/lib/mysql/mysql.sock
```

여기서 s는 socket을 의미하며 file type을 알려주는 file 명령어를 해도 소켓 타입으로 표시한다.

```bash
$ file /var/lib/mysql/mysql.sock 
                       
/var/lib/mysql/mysql.sock: socket
```

lsof 명령어를 -U 옵션을 주고 실행하면 시스템의 전체 유닉스 도메인 소켓 목록을 확인할 수 있다.

```bash
$  lsof -U 

COMMAND      PID           USER   FD   TYPE             DEVICE SIZE/OFF    NODE NAME
systemd        1           root   15u  unix 0xffff9b9f575fbf00      0t0     164 /run/systemd/private type=STREAM
systemd        1           root   17u  unix 0xffff9b9fb1acf980      0t0 1573299 /run/systemd/journal/stdout type=STREAM
systemd        1           root   46u  unix 0xffff9b9f58366780      0t0   20542 type=DGRAM
systemd        1           root   63u  unix 0xffff9ba0e8266300      0t0   33828 type=STREAM
systemd        1           root   83u  unix 0xffff9b9f56470900      0t0   16877 /run/systemd/notify type=DGRAM
systemd        1           root   84u  unix 0xffff9b9f56476780      0t0   16879 /run/systemd/cgroups-agent type=DGRAM
systemd        1           root   85u  unix 0xffff9b9f56473a80      0t0   16881 type=DGRAM
systemd        1           root   86u  unix 0xffff9b9f56473f00      0t0   16882 type=DGRAM
systemd        1           root   91u  unix 0xffff9b9f565a6300      0t0   24191 /run/libvirt/libvirt-admin-sock type=STREAM
```

<br>

### mysql.sock

tcp/ip 소켓이 아이피 주소와 포트를 이용해 접속을 하듯 Unix Domain Socket은 파일을 이용한다. 

`mysql.sock` 이라는 것은 바로 이용도로 사용되는 파일이다. 따라서 mysqld를 실행시키면 mysql.sock 파일이 존재하지만 중지시키면 사라지게 된다.

그렇기에 mysql이 다른 시스템과 통신하기 위해서는 mysql.sock 파일이 서버와 클라이언트 모두 접근이 가능해야하며, mysql.sock 파일이 생성되는 위치에 서버가 파일을 쓰거나 읽을 수 없으면 서버는 에러를 발생시키고 중지되며, 클라이언트가 그 파일에 접근할 수 없으면 접속이 이루어지지 않는다. 보통 mysql.sock의 문제는 이 접근 권한이 잘못되어 발생하지는 않는 것이다.

rpm으로 설치하게 되면 보통 mysql.sock 파일의 위치는 /var/lib/mysql/mysql.sock 이다. 이것은 mysql을 컴파일할 때 --with-unix-socket-path= 에서 설정할 수 있다. 이렇게 되면 mysqld, mysql 그리고 libmysqlclient.a(so)도 모두 이 설정을 이용해 통신을 하게 된다. 이 도메인 소켓의 위치는 컴파일 후에라도 명령행 인수를 이용해 바꿀 수 있으며 또는 /etc/my.cnf 파일을 지정해 사용할 수 있다.