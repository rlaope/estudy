# permission denied while trying .. Docker demon socket 트러블 슈팅

docker, docker compose를 사용해 Spring boot Application을 배포하기 위해 `sudo apt-get install docker, docker compose`를 해준 상황

그러나 `docker ps`를 입력하면

```
permission denied while trying to connect to the Docker daemon socket at unix:///var/run/docker.sock: Get "http://%2Fvar%2Frun%2Fdocker.sock/v1.24/containers/json": dial unix /var/run/docker.sock: connect: permission denied
```

이러한 오류메세지가 뜨게된 상황..

도커 권한 문제 때문에 docker 명령어가 먹지 않는 상황 문제는 루트가 아닌 사용자로 도커를 실행시키려고 했기 때문이다.

<br>

## 해결 방법

### docker 그룹 생성

```zsh
$ sudo groupadd docker
```


### docker 그룹에 사용자 추가
```zsh
$ sudo usermod -aG docker $USER
```

### 새 그룹에 로그인

```zsh
$ newgrp docker
```

이렇게 하면 정상적으로 작동하는 것을 볼 수 있다.