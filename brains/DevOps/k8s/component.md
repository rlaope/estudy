# 쿠버네티스 구성요소

쿠버네티스는 여러 개의 노드(ex: 가상 서버/ Virtual Machine)로 구성된 클러스터로 이루어져 있습니다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FdFM2jb%2Fbtq4jEL7V4O%2FsqemIpdhQNWEIQBYuvSr0K%2Fimg.png)

여기서 Node라는 개념이 나온다.

위의 그림에서 Node는 하나의 VM을 의미한다. 쿠버네티스는 컨테이너화 된 애플리케이션을 실행하는 Worker Node와 그러한 Worker Node를 관리하는 Master Node로 구성되어 있다.

이때 Worker Node와 Master Node는 다수로 이루어질 수 있으며, 쿠버네티스를 사용하기 위해서는 최소 1개의 Worker Node를 보유해야 한다.

<br>

## Master Node 구성요소
Master Node에는 클러스터에 관한 전반적인 결정을 수행하고, 이벤트를 감지하고 반응하는 역할을 해야합니다.

Master Node는 다음과 같은 그림으로 구성되어 있습니다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FvFgU9%2Fbtq4cfujghF%2F86KkmJEn2ed0WCwsATHR9K%2Fimg.png)

### 컴포넌트
- kube-apiserver: 모든 요청을 처리하는 역할
- kube-controller-manager: 다양한 컨트롤러(복제/배포/상태 등)를 관리
- kube-scheduler: 상황에 맞게 적절한 Worker Node를 선택
- etcd: 클러스터 내의 데이터를 담는 저장소

<br>

## Worker Node 구성요소
Worker Node에서는 컨테이너화된 애플리케이션을 동작하고 유지시키는 역할을 합니다.

Worker Node는 다음과 같은 그림으로 구성되어 있습니다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fc94mmX%2Fbtq4dcjdjIz%2FwO5ri20IjKWAk1wJYICYK1%2Fimg.png)

### 컴포넌트
- pod: 컨테이너화된 애플리케이션 그룹
- kubelet: Node에 할당된 pod의 상태를 체크하고 관리
- kube-proxy: pod로 연결되는 네트워크를 관리 