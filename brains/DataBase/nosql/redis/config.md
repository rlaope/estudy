# Redis 효율적인 서버 구성

레디스를 사용하기 위해 일부 서버 설정 파일과 레디스의 구성 파일을 변경하는 것이 좋다.

몇가지 이점을 내기 위해 활성화시키면 좋을 설정사항들을 확인해보자.

### THP 비활성화

리눅스는 메모리를 페이지 단위로 관리하며, 기본 페이지는 4KB로 고정이다. 메모리 크기가 커질수록 페이지를 관리하는 테이블인 TLB의 크기도 커져, 메모리를 사용할때 오버헤드가 발생할 수 있는 이슈가 있어 페이지를 이부러 크게 만든 뒤 자동으로 관리하는 THP(Transparent Huge Page) 기능이 도입됐다.

하지만 레디스와 같은 데이터베이스 애플리케이션에서는 오히려 이 기능을 사용할 때 퍼포먼스가 떨어지고 레이턴시가 올라가는 현상이 발생하기 때문에 비활성화 하는 것을 추천한다.

```sh
echo never > /sys/kernel/mm/transparent_hugepage/enabled
```

위의 명령어를 통해서 일시적으로 비활성화할 수 있고 영구적으로 반영하고 싶다면 redis의 `/etc/rc.local` 파일에 다음 구문을 추가하면 된다.

```sh
if test -f /sys/kernal/mm/transparent_hugepage/enbled; then
	echo never > /sys/kernal/mm/transparent_hugepage/enbled
fi
```

<br>

### vm.overcommit_memory = 1

레디스는 디스크에 파일을 저장할 때 `fork()`를 이용해 백그라운드 프로세스를 만들고 이때, COW(Copy On Write)라는 메커니즘이 동작하게 된다. 이 메커니즘에서는 부모 프로세스와 자식 프로세스가 동일한 메모리 페이지를 공유하다가 레디스로 인해서 데이터가 변경될 때마다 **메모리 페이지를 복사하기 때문에 데이터 변경이 많이 발생하게 된다면 메모리 사용량이 빠르게 증가할 수 있다.**


따라서 레디스 프로세스가 실행되는 도중 메모리를 순차적으로 초과해 할당해야하는 상황이 발생할 수 있으며, 이를 위해서 `vm.overcommit_memory`를 1로 설정한다. 이는 메모리를 순간적으로 초과해 할당해야 하는 동작을 수행하며, 잘못된 동작을 방지하고 백그라운드에서 데이터를 저장하는 과정에서 성능 저하나 오류를 방지할 수 있게 설정하는 것이좋다. (기본적으로는 0으로 설정되어있다.)

`/etc/sysctl.conf`파일에 vm.overcommit_memory=1 구문을 추가하면 영구적으로 해당 설정을 적용할 수 있으며, 재부팅 없이 설정을 적용하려면 sysctl vm.overcommit_memory=1을 수행하면 된다.

<br>

### somaxconn, syn_blocking 설정 변경

레디스의 설정파일의 tcp-backlog 파라미터는 레디스의 인스턴스가 클라이언트와 통신할 때 사용하는 tcp backlog 큐의 크기를 지정한다.

이때 redis.conf에서 지정한 tcp-backlog 값은 서버의 somaxconn(socket max connection)과 syn_blocking 값보다 클 수 없다. 기본 tcp-backlog 값은 511이므로, 서버 설정이 최소 이 값보다 크도록 설정해야 한다.

/etc/sysctl.conf 파일에 구문을 추가해서 해결할 수도 있지만 재실행없이 적용하려면 sysctl를 사용한다.

```bash
# redis config 
tcp-backlog 1024 

# sysctl 
sysctl -w net.ipv4.tcp_max_syn_backlog=1024
sysctl -w net.core.somaxconn=1024
```

 
 **크게 설정해야하는 이유?**
 
 syn_backlog은 tcp half open 시 syn + ack flag packet을 보내고 3-way handshaking 전까지 backlog에 저장한다. 클라이언트에서 ACK 응답이오면 비워지고 오지않으면 timeout 까지 보관하며, redis의 backlog 크기는 syn_backlog , somaxconn 설정값을 넘을 수 없게된다.

syn_backlog 또는 somaxconn = 128 이면 128 +1 = 129  -> 129 * 2 = 258 -> 258 의 근사값 = 256 즉 redis에서 약 256개 이상 backlog를 가져갈 수 없다. redis에서는 tcp-backlog 511 이지만 syn_backlog, somaxconn = 128 이면 256개 밖에 처리 못한다.

> 쉽게 말해서 레디스가 리눅스에 연결될 client의 수를 저장할 backlog 큐의 사이즈는 크지만 레디스가 적어 효율적으로 처리할 수 없게 되니 설정을 통해서 늘려준다는 것이다.







