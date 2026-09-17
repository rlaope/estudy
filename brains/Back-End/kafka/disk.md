# Kafka Disk I/O

Kafka는 브로커의 로컬 디스크에 데이터를 저장한다. 그럼에도 카프카는 빠르다 그 이유가 무엇일까?

### 순차 I/O

디스크 I/O는 어떻게 사용하는지에 따라 느릴 수도 있고 빠를 수도 있다.

아래의 그림처럼 **디스크에 순차적으로 데이터에 접근하는 속도**는 디스크 랜덤 엑세스에 비해서 150,000배 빠르고 메모리 랜덤 엑세스보다 더 빠르다.

![](https://miro.medium.com/v2/resize:fit:1400/format:webp/1*XxYnFy3pc918ljOWJ8IOlg.png)

최신 os들은 `read-ahead`, `write-behind` 같은 기술을 제공해서 순차 읽기/쓰기 작업이 더 빠르게 수행되도록 지원한다. 카프카는 **데이터를 메시지 큐 방식으로 저장하는데, 이는 순차 I/O의 혜택을 볼 수 있어 빠른 성능을 제공한다.**


<br>

### Page Cache

디스크 검색을 줄이고 처리량을 높이기 위해 최신 운영체제들은 **Page Cache(디스크 캐시)** 를 위해 메인 메모리를 더 공격적으로 사용하게 되었다. 모든 디스크 읽기/쓰기는 페이지 캐시를 거치게되며, 사용자나 애플리케이션에 의해 관리되는 것이 아니라 os에 의해 관리되므로 사용자 영역과 커널 영역의 중복 저장 없이 2배 정도의 캐시를 저장할 수 있으며 애플리케이션 재시작시에도 빠르게 캐시의 혜택을 볼 수 있다.

![](https://miro.medium.com/v2/resize:fit:1400/format:webp/1*rjlyW4hhBaJGqVJfKu7dSw.png)


Kafka는 JVM 위에서 동작하고 Heap 메모리에 객체를 저장하는 비용은 매우 비싸고, 힙 메모리가 커질수록 GC가 느려진다는 단점이 있다. 결론적으로 순차 읽기/쓰기 혜택을 볼 수 있는 **File System과 page cache**를 사용하는게 메모리 캐시나 다른 구조를 사용하는 것 보다 좋은 성능을 낼 수 있다.

<br>

### Zero Copy

![](https://miro.medium.com/v2/resize:fit:1400/format:webp/1*wb6XhkM0vKf9YxyEHLlvsw.png)


일반적으로 **네트워크를 통해 데이터를 전달하는 절차**는 아래와 같다.

1. os는 디스크로부터 데이터를 읽어 커널 영역의 page cache에 저장한다.
2. 애플리케이션은 page cache의 데이터를 사용자 영역으로 읽어온다.
3. 애플리케이션은 커널 영역에 있는 socket buffer로 데이터를 쓴다.
4. os는 socket buffer에 있는 데이터를 NIC buffer로 복사하고 네트워크를 통해 전송한다.

위의 과정에서는 불필요한 system call이 발생하는데 os가 제공하는 sendfile 함수는 커널 영역의 page cache에서 nic buffer로 직접 복사가 가능해 **효율적으로 데이터를 전송할 수 있다.**

![](https://miro.medium.com/v2/resize:fit:1400/format:webp/1*XNOO-4gj7xOwSzbSjeQ4jw.png)

kafka는 이러한 **Zero Copy**기술을 이용해 메시지가 생성/소비될 때 불필요한 복사와 system call을 줄여 효과적으로 데이터를 전송한다. 한 건의 경우에 성능에 큰 영향은 없지만 수십, 수백만 건의 대량의 데이터가 카프카를 통해 전달될 경우에는 성능의 차이를 실감할 수 있다.

이 외에도 배치 전송이나 압축등이 더 있으니 찾아보면 좋겠다.