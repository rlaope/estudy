# Producer 주요 Options

### bootstrap.servers

카프카 클러스터에 처음 연결하기 위한 호스트와 포트 정보로 구성된 리스트 정보를 나타낸다.

### acks

프로듀서가 카프카 토픽의 **리더에게 메세지를 보낸 후 요청을 완료하기 전 ack(승인) 수**이다.

- **ack=0**: 프로듀서는 어떤 서버로부터 어떠한 ack도 기다리지 않는다. ack 요청을 기다리지 않기 때문에 매우 빠르게 메세지를 보낼 수 있지만 메세지 손실 가능성이 높다.
- **ack=1**: 리더 데이터를 기록한다. 그렇지만 모든 팔로워는 확인하지 않기 때문에 메세지 손실이 발생할 수도 있다.
- **ack=all**: 리더는 ISR의 팔로워로부터 데이터에 대한 ack를 기다리기 때문에, 팔로워가 있는 한 데이터 무손실에 대해 강력하게 보장할 수 있다. 완벽하게 사용하기 위해서는 브로커 설정도 같이 조정해야한다

기본값은 ack=all 이다.

### ack=all과 brodker min.insync.replicas 옵션의 관계

`min.insync.replicas` 옵션은 성공적으로 간주되는 메세지에 쓰기를 승인해야하는 **최소 복제본 수를 지정하는 옵션이다.** 다시 말하면 최소 리플리케이션을 유지해야하는 수로도 말할 수 있다.

#### ack=all, min.insync.replicas=1

![](https://user-images.githubusercontent.com/45676906/211141530-4f964427-8d8d-4760-92af-73af12bddd36.png)

min.insync.replicas 옵션이 1이기 때문에 프로듀서가 리더에게 메세지를 보냈을 때 리더는 최소 1개의 브로커에게만 메세지를 잘 받았는지 확인하면 된다. 즉 자기 자신이 잘 받았으면 바로 ack 응답을 보낸다.

#### ack=all, min.insync.replicas=2

![](https://user-images.githubusercontent.com/45676906/211141989-3d3b4e54-491d-4259-a167-67fab09ab38e.png)

프로듀서는 메세지를 리더에게 보낸다. 리더는 메세지를 받은 후 저장하고 팔로워는 해당 메세지를 가져와 저장한다. 리더는 팔로워에게 메세지가 잘 복제되었는지 확인한다 **min.insync.replica** 옵션이 2이기 때문에 리더 1, 팔로워 1개만 확인하고 리더가 acks 응답을 프로듀서에게 보낸다.

#### ack=all, min.insync.replica=3

![](https://user-images.githubusercontent.com/45676906/211142238-ef4c58a3-d488-41f5-9ebb-4663d9643feb.png)

리더는 producer로 부터 메세지를 받고 저장한 후 팔로워는 해당 메세지를 가져와 저장한다. 리더는 팔로워에게 메세지가 잘 복제되었는지 확인한다. min.insync.replica 옵션이 3이기 때문에 리더1, 팔로워 2개에 대해 확인하고 acks를 보낸다.

여기서 권장되는 옵션은 `ack=all` `min.insync.replicas=2`이다. 

그 이유는 min.insync.replicas 옵션이 2라면 하나의 브로커에 문제가 생겨도 클러스터 전체 장애로 이어지는 것이 아니다. (복제본 하나가 문제가 생겼을때 ISR 그룹에서 하나가 제외됨) 그러나 3을 사용했을 때는 하나의 브로커에 문제가 생겨도 클러스터 전체 장애로 이어진다. 그렇기 때문에 min.insync.replicas 옵션 2와 ack=all 옵션을 적용하는 것을 권장한다.



