# Volume - emptyDir, hostPath, PV/PVC

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fby3opf%2FbtqYQxNFeyU%2FHCEEJp53CvgAkCzIyW0nL0%2Fimg.png)

## empty Dir

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FEEKLg%2FbtqY5MCyP15%2FmE1vNFhYnHgCmzBevGCk80%2Fimg.png)

컨테이너들끼리 데이터를 공유하기 위해서 볼륨을 사용하는 것이다.

최초 볼륨이 생성될 때 볼륨 내용이 비어있기 때문에 emptyDir로 명명된다.

Container1이 web이고 Container2가 백엔드라고 하자. Container2도 Volume을 이용하기 때문에 두 서버가 파일을 주고받을 필요 없이 편하게 사용이 가능하다.

이 볼륨은 파드안에 생성되기 때문에 파드에 문제가 생겨 재생성되면 데이터가 없어진다.

볼륨에 쓰인 데이터는 꼭 일시적인 사용목적을 가져야한다.

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: pod-volume-1
spec:
  containers:
  - name: container1
    image: kubetm/init
    volumeMounts:
    - name: empty-dir
      mountPath: /mount1
  - name: container2
    image: kubetm/init
    volumeMounts:
    - name: empty-dir
      mountPath: /mount2
  volumes:
  - name : empty-dir
    emptyDir: {}
```

- 컨테이너가 2개
- 둘 다 볼륨을 마운트하고 있다.
- 마운트하는 path를 보면 컨테이너1은 mount1, 컨테이너2는 mount2
- path가 달라도 name이 empty-dir이여서 컨테이너마다 자신이 원하는 경로를 사용할 수 있다.
- 즉, 둘다 같은 볼륨을 사용하고 있는 것이다.

<br>

## hostPath

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fb3o0DT%2FbtqY9Y956M3%2FgfALaPoPheneDP0ik3bDk0%2Fimg.png)

파드들이 올라가진 노드의 path를 볼륨으로써 사용한다.

emptyDir이랑 다른점은 path를 파드들이 공유하기 때문에 파드가 죽어도 노드의 볼륨은 죽지 않는다.

문제점은 파드2가 죽어서 노드1이 아닌 노드2에 파드2가 재생성되면 노드1의 볼륨을 이용할 수 없다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FcEA6dd%2FbtqY7nCv8s6%2FDtUZ4Qk1cMahSN4aBDrcLk%2Fimg.png)

노드2가 생길 때, 똑같은 경로를 만들어서 노드에 있는 path끼리 마운트시켜주면 해결이 가능하지만 쿠버네티스가 해주는게 아니다.

운영자가 리눅스 시스템의 마운트 기술을 사용하는 것 -> 자동화가 아니니 안정적이지 않다. 

hostPath는 파드 자신이 할당되어 있는 노드의 데이터를 읽거나 쓸 때 사용한다.

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: pod-volume-3
spec:
  nodeSelector:
    kubernetes.io/hostname: k8s-node1
  containers:
  - name: container
    image: kubetm/init
    volumeMounts:
    - name: host-path
      mountPath: /mount1
  volumes:
  - name : host-path
    hostPath:
      path: /node-v
      type: DirectoryOrCreate
```

- 파드를 만들 때 컨테이너에서 볼륨을 마운트하는데 mountPath는 /mount1
- path에 대한 host-path의 볼륨이 /node-v에 있다.
- host-path의 타입은 Directory
- host-path의 path/node-v는 파드가 생성되기 전에 있어야 오류가 나지 않는다.
- hostPath는 파드의 데이터를 저장하는 용도가 아니라 노드의 데이터를 파드에서 쓰기 위함이다.

<br>

## PVC/PV

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fpkvrz%2FbtqY91eHlSw%2FeKkfTLmOQct4cCGKTvhyh1%2Fimg.png)

Persistent Volume Claim, Persistent Volume

PVC/PV는 파드에 영속성 잇는 볼륨을 제공한다.

볼륨은 Local도 있고 외부에 원격으로 활용하는 것도 있다.

이런 것들을 각각 PV로 정의하여 연결한다.

파드는 PV에 바로 연결하지 않고 PVC를 통해서 연결된다.

파드를 PV에 바로 연결하지 않는 이유는 쿠버네티스가 User와 Admin으로 영역을 나눠서 관리하기 때문이다.

Admin은 쿠버네티스 운영자, User는 파드에 서비스를 만들고 배포를 담당하는 자다.