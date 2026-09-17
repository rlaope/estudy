# Service - Headless, Endpoint, ExternalName

이번에는 파드입장에서 네트워킹을 생각해보자.

파드A가 다른 파드 B에 접속하려면 어떻게 할 수 있을까?

1. 파드A - 파드B IP를 통해 직접 연결
2. 서비스를 통해 연결

그런데 여기서 문제가 있다. 일단 당연하게 생각할 수 있는 문제가 파드의 아이피는 변경이 되기 때문에 직접 아이피를 참조하는 것은 좋은 방법이 아니다.

그리고 파드와 서비스가 동시에 배포될 때 IP를 동적할당하니 배포되기전까지 아이피를 알 수 가 없는 문제가 생긴다.

그렇기에 헤드리스와 DNS Server가 필요하다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2F2ImmB%2FbtqYW2fbUd7%2F9Pdja1IZ5gR0oJKhryKJz1%2Fimg.png)

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2F04D4g%2FbtqYZ53N4z0%2FFCISXD2Sxkt5cUdMbeW6Ck%2Fimg.png)

default 네임스페이스에 두 개의 파드와 서비스가 연결이 되어있다.

이 때 파드가 서비스에 연결하고 싶을 때를 생각해보자.

위에 첫번째 케이스에서는 서비스는 클러스터 아이피 방식이고 동적 할당되기 때문에 시작 시점에 알 수 없다.

DNS가 결국 해결책이다. DNS이름은 `cluster.local` DNS에서는 파드건 서비스건 고유의 주소가 있다. 그리고 그 고유의 주소는 사용자가 입력한다.

그렇기에 파드를 생성할 때 DNS 주소만 넣으면 되기 때문에 생성 시점에 연결할 수 있게 된다.

### DNS 구조

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbKDnrK%2FbtqYTN3NJts%2FO0rZBaY0AKF4vE4JKuL8E0%2Fimg.png)

일반적인 DNS 서버를 두었을때 DNS 네임은 FQDN = Fully Qualified Domain Name으로 작성된다.

- `service1.default.svc.cluster.local`: 서비스는 서비스 이름.네임스페이스.서비스의 약어인 svc.DNS 이름으로 구성
- `20-109-5-11.default.pod.cluster.local`: 파드는 아이피.네임스페이스.파드를 뜻하는 pod.DNS 이름으로 구성
- 이러한 긴 이름을 **FQDN = Full Qualified Domain Name**이라고 함
- 서비스는 앞부분 service1만 쓰면 되는데 파드는 풀네임으로 다 써야함

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FcgjBXd%2FbtqYQwuxHXQ%2Fn9F3c3IDNy1y9rcxbKdPM0%2Fimg.png)

Pod에서 Pod4로 연결할 때 직접 연결하고 싶으면 headless로 만들어야한다.

Headless로 만드는 방법은 clusterIP:None으로 설정하면 된다.

파드4, 파드5도 hostname에 파드의 호스트네임을 넣어야 하고(pod4, pod5) subdomain에 headless1을 넣어야함

이렇게 만들면 DNS에서 서비스는 Headless가 없을 때와 똑같이 구성이 된다.

그런데 서비스의 아이피가 없기 때문에 서비스의 이름을 호출하면 연결되어 있는 모든 파드의 아이피를 주게된다.

파드는 기존에 아이피.default 형식으로 구성이 되어있는데 headless에서는 pod4.headless1.default 식으로 구성됨

게다가 파드는 앞에 pod4.headless1와 같이 앞부분만 호출하면 된다.

**따라서 Pod는 pod4나 pod5에 연결하고 싶으면 pod4.headless1의 이름만 저장해 놓으면 됨 --> 해결!**

<br>

### Endpoint

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FSmKvj%2FbtqY7oIgvqh%2FHHmPKyr7PWTbMkJHLTYAVk%2Fimg.png)

서비스와 파드를 연결할 때 라벨을 사용해서 연결을 한다.

이는 사실 쿠버네티스가 라벨을 통해 Endpoint를 만들어주는 것이다.

엔드포인트는 서비스와 동일한 이름을 가지고 있고, 엔드포인트 안에 파드의 아이피 정보를 담는다.

이 규칙을 알면 라벨과 셀렉터를 안 만들어도 연결이 가능하다.

서비스 파드를 만들 때 엔드포인트를 직접 만들어도 되고 타겟 아이피는 외부 아이피도 가능하다.

<br>

### ExternalName

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FCSPbB%2FbtqYZ6n5Uki%2FCtlzUzI8ETVB5RnnGE8v31%2Fimg.png)

서비스의 ExternalName 즉 도메인 이름을 넣을 수 있다.

DNS 캐시가 내부, 외부 DNS 서버에서 찾아 아이피를 알아낸다.

파드는 서비스를 가리키고만 있으면 서비스에서 필요할 때마다 도메인 주소를 변경할 수 있어서 파드를 수정하고 재배포 안 해도 됨



[[Service - ClusterIP, NodePort, LoadBalancer]]

