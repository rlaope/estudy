# 도커 네트워크 구조

## 네트워크 구조

도커는 IP를 `172.17.0.X` 형태로 순차적으로 나눠주게된다.

![](https://velog.velcdn.com/images%2Fckstn0777%2Fpost%2Fb3157db0-06fe-46ab-90b1-e49e67d3ecbb%2Fimage.png)

이 ip는 내부 ip로서 도커 컨테이너 내부에서만 쓸 수 있고, 외부와 연결될 필요가 있다.
  
그렇다면 어떻게 연결이 일어나는 걸까?
  
호스트에서 ifconfig 명령을 실행해보면 veth라고 시작하는 네트워크 인터페이스가 있다.  
virtual eth라는 의미인데 컨테이너가 생성 될 때 도커가 자도응로 생성해준다.

![](https://velog.velcdn.com/images%2Fckstn0777%2Fpost%2F673a3468-fd9c-4539-bd95-e621b1fd367f%2Fimage.png)

그 구조를 그림으로 보면 아래와 같다.

![](https://velog.velcdn.com/images%2Fckstn0777%2Fpost%2Faa686e2f-098a-4c22-8921-f425802c7156%2Fimage.png)

도커 컨테이너안에 eth0은 vethXX와 연결이 된다.
  
다시 vethXX들은 docker0이라는 브리지에 연결이 되고, 마지막으로 docker0은 호스트의 eth0와 연결이 되는 형태인 것이다.

![](https://velog.velcdn.com/images%2Fckstn0777%2Fpost%2F3f590e51-ebe8-4651-adee-35fb0b2c4dd1%2Fimage.png)

docker0는 각 veth 인터페이스와 바인딩되어 호스트의 eth0 인터페이스를 이어주는 역할을 한다.

<br>

## 도커 네트워크의 기능

네트워크 드라이버로는 브리지 말고도 호스트, 논, 컨테이너 등이 있다.
  
먼저 도커에서 기본적으로 쓸 수 있는 네트워크가 무엇이 있는지 확인해보자 `docker network ls`

![](https://velog.velcdn.com/images%2Fckstn0777%2Fpost%2Fade30515-043d-49eb-917e-f37c830d7718%2Fimage.png)

### 1. bridge 네트워크
docker0 브리지와는 비슷하지만 여기서는 사용자 정의 브리지를 새로 생성해 각 컨테이너와 연결하는 구조를 의미한다.
  
한번 사용자 정의 브리지를 생성해보겠다.

```s
$ docker network create --driver bridge mybridge
```

![](https://velog.velcdn.com/images%2Fckstn0777%2Fpost%2F6186940d-eec2-4d62-93fa-096d9a5313b5%2Fimage.png)

그리고 나서, mybridge로 연결하는 컨테이너를 생성해보겠다.

```s
$ docker run -it --name mynetwork_container --net mybridge ubuntu:18.04
```
컨테이너 접속 후, ifconfig를 통해 확인해본결과 이번에는 `172.18.0.2`라고 나왔다.

![](https://velog.velcdn.com/images%2Fckstn0777%2Fpost%2F18c028e9-86ce-4cd8-9ef2-039013244c4f%2Fimage.png)

이렇게 생성된 사용자 정의 네트워크는 docker network disconnect, connect를 통해 컨테이너에 유동적으로 붙이고 뗄 수 있습니다.

```s
$ docker network disconnect mybridge mynetwork_container
$ docker network connect mybridge mynetwork_container
```

추가적으로 네트워크의 서브넷, 게이트웨어, ip할당 범위를 설정하려면 docker network create 에서 옵션을 설정해주면 됩니다.

## 2. host 네트워크
host 네트워크는 말 그대로 컨테이너에서 호스트의 네트워크 환경을 그대로 쓸 수 있다.
  
`--net host` 옵션을 준 컨테이너를 생성해봅시다.

```s
$ docker run -it --name network_host --net host ubuntu:18.04
```

컨테이너에서 ifconfig를 해보면 host와 동일한 결과를 볼 수 있다.
  
따라서 별도 포트포워딩 없이 바로 서비스할 수 있게 된다.

## 3. none 네트워크

말 그대로 아무런 네트워크를 쓰지 않는 것을 의미한다. 외부와의 단절..

## 4. container 네트워크
다른 컨테이너의 네트워크 네임스페이스 환경을 공유할 수 있다. 
  
공유되는 속성은 내부 IP, MAC 주소다.
  
컨테이너 2개를 생성해주는데 network_container_2가 network_container_1의 네트워크 환경을 공유하도록 설정해준다.

```s
$ docker run -it -d --name network_container_1 ubuntu:18.04
$ docker run -it -d --name network_container_2 --net container:network_container_1 ubuntu:18.04
```

<br>

## 브리지 네트워크와 --net alias
--net-alias를 alicek106이라고 설정한 컨테이너 3개를 만듭니다.
  
컨테이너들은 mybridge라는 위에서 만든 사용자 정의 브리지와 연결되어있는 형태입니다.

```s
$ docker run -it -d --name network_alias_container1 --net mybridge \ --net-alias alicek106 ubuntu:18.04
$ docker run -it -d --name network_alias_container2 --net mybrdige \ --net alias alicek106 ubuntu:18.04
```

각 아이피는 172.18.0.X 부터 순차적으로 부여될 것입니다.
  
network_alias_ping 이라는 컨테이너를 하나 더 만든 후 이 안에서 ping을 발생시켜보겠습니다.

```s
$ docker run -it --name network_alias_ping --net mybridge ubuntu:18.04
```
```s
// apt-get install iputils-ping 설치 필요
$ ping -c 1 alicek106
$ ping -c 1 alicek106
```

![](https://velog.velcdn.com/images%2Fckstn0777%2Fpost%2F1b3e317b-4efb-4f65-b485-1cdf0a1fec01%2Fimage.png)

이렇게 발생시키면 컨테이너 2개의 IP로 각각 ping이 전송되는 것을 알 수 있습니다. 매번 달라지는 IP를 결정하는 것은 별도의 알고리즘이 아닌 라운드 로빈방식입니다.
  
이런 결과가 나오는 이유는 도커 내장 DNS가 alicek106이라는 호스트로 요청한 컨테이너에게 IP목록을 반환해주기 때문에 일어나는 현상입니다. (도커 내장 DNS가 IP로 변환해준다는 소리)

## MacVLAN 네트워크
호스트의 네트워크 인터페이스 카드를 가상화해 물리 네트워크 환경을 컨테이너에게 동일하게 제공합니다. (도커 컨테이너에게 기본적으로 할당되는 IP대역인 172.17.X.X가 아니라 진짜 네트워크 장비의 IP를 할당받는다는 의미)
  
따라서 MacVLAN을 사용하는 컨테이너들과 동일한 IP대역을 사용하는 서버 및 컨테이너들은 서로 통신이 가능합니다.(단, 기본적으로 호스트와 통신을 불가능합니다.)