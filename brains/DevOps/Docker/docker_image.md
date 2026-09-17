# 도커 이미지 생성

우분투 컨테이너를 만든 후 아무 파일이나 하나 생성해 기존 이미지에서 변경사항을 만들어보겠다.

```s
$ docker run -it --name commit_test ubuntu:18:04
$ echo test_first! >> first (컨테이너 내부에서 실행)
```

컨테이너를 빠져나온 다음 docker commit 명령어로 컨테이너를 이미지화 시킨다.

```s
$ docker commit -a "ckstn0777" -m "my first commit" commit_test commit_test:first
```

- -a 옵션은 author를 의미하며 이미지 작성자를 나타내는 메타 데이터를 이미지에 포함시킵니다.
- -m 옵션은 커밋 메시지 입니다.
- commit_test는 커밋할 컨테이너의 이름입니다.
- commit_test:first는 생설될 이미지 이름과 태그입니다. 태그를 입력하지 않으면 자동으로 latest로 설정됩니다.

docker images로 잘 생성되었는지 확인해봅니다.

![](https://velog.velcdn.com/images%2Fckstn0777%2Fpost%2F38523607-56f7-42a6-82e9-be1f6add316e%2Fimage.png)

이제 방금 만든 이미지를 이용해 컨테이너를 생성할 수 있습니다.
  
first 파일도 존재하는 것을 확인할 수 있습니다.

```s
$ docker run -it --name commit_test2 commit_test:first
```

![](https://velog.velcdn.com/images%2Fckstn0777%2Fpost%2Fd350b244-8505-4f7d-a67a-739ace36c1bb%2Fimage.png)

## 이미지 구조 이해

```s
$ docker inspect ubuntu:18:04
```
![](https://velog.velcdn.com/images%2Fckstn0777%2Fpost%2F34672132-440f-4393-a84d-23e4bf84f1ad%2Fimage.png)

다음으로는 commit_test:first의 layer를 보겠다.

![](https://velog.velcdn.com/images%2Fckstn0777%2Fpost%2F8f3d41a4-68bf-420d-91fb-db5ff00469bc%2Fimage.png)

둘의 차이점은 commit_test:first layer 가장 밑에 한개가 더 있다는 점이다.
  
아마 이 부분이 바로 변경을 기록한 부분이라고 생각하면 된다.
  
위에서 ubuntu 이미지 크기는 64.2MB, commit_test 역시 64.2MB이었다. 그럼 도합 128.4MB를 차지하고 있을까?  
  
이미지를 커밋할 때 컨테이너에서 변경된 사항만 새로운 레이어로 저장하고, 그 레이어를 포함해 새로운 이미지를 생성하기 때문에 실제 크기는 64.2MB + first 파일 크기 만큼 차지하게 된다.

## 이미지 삭제
만약 commit_test:first로 commit_test2 컨테이너를 생성했다고 치고, commit_test:first를 삭제하려고 하면 제대로 될까? (참고로 이미지 삭제 명령은 docker rmi다.)

![](https://velog.velcdn.com/images%2Fckstn0777%2Fpost%2F6e9901be-894c-46f6-aa42-b998946f6184%2Fimage.png)

보니까 컨테이너 1a73..(=commit_test2)이 사용중이라고 한다. 강제로 삭제시키고 이미지 또한 삭제시켜보겠다.


```s
$ docker rm -f commit_test2
$ docker rmi commit_test:first
```

참고로, 말 안한게 있는데 commit_test2에서 second 파일 생성 후 commit_test:second라는 이미지를 생성해주었다.

![](https://velog.velcdn.com/images%2Fckstn0777%2Fpost%2F962a33fc-053a-4f06-8c29-6f93ff14be9e%2Fimage.png)

그렇다면.. commit_test:first라는 이미지를 삭제했다고 해서 그 레이어 파일은 사라졌을까? 아니다. commit_test:second가 가지고 있을거다.
  
commit_test:second를 삭제해보겠다

![](https://velog.velcdn.com/images%2Fckstn0777%2Fpost%2F36deb2da-b14c-44b0-b69f-03537a45309f%2Fimage.png)

이번에는 삭제되는 레이어가 나오고 있다.
  
결국 여기서 알 수 있는 점은 삭제되는 이미지의 부모 이미지가 존재하지 않아야만 해당 이미지의 파일이 실제 삭제된다는 점이다.

## 이미지 추출
도커 이미지를 별도로 저장하거나 옮기는 등 필요에 따라서 이미지를 단일 바이너리 파일로 저장해야할 때가 있다. 
  
하지만 레이어 구조가 아닌 단일 파일이기 때문에 이미지 용량을 각기 차지 하게 된다.
  

## 이미지 배포
사실 이미지 추출보다 이미지 배포가 더 중요하다.
  
방법은 도커 허브 이미지 저장소를 사용하는 것과 사용자가 직접 만드는 사설 레지스트리를 이용하는 방법이 있는데 더커 허브 저장소를 이용하는 방법을 실습해보겠다.
  
[도커 허브](https://hub.docker.com/)

![](https://velog.velcdn.com/images%2Fckstn0777%2Fpost%2Fc0341894-19d9-47f1-8700-bea9c1151a27%2Fimage.png)

### 이미지 저장소 생성
뭔가 깃허브랑 비슷하게 생겼다. Create Repository를 클릭한다.

![](https://velog.velcdn.com/images%2Fckstn0777%2Fpost%2F2c53c866-9834-4aa4-96cf-a908f8028bae%2Fimage.png)

### 저장소에 올릴 이미지 생성

```s
$ docker commit commit_test my-image-name:0.0
```

먼저 commit_test 컨테이너로 부터 이미지를 생성한다.
  
그러나 이 이미지로는 이미지 저장소에 올릴 수 없다. 특정 저장소에다 올릴때는 앞에 저장소 이름(사용자 이름)을 붙여야 한다.

```s
$ docker tag my-image-name:0.0 사용자이름/my-image-name:0.0
```

이렇게 하면 기존 이미지가 사라지는 건 아니고 이미지를 복붙하는거다.

![](https://velog.velcdn.com/images%2Fckstn0777%2Fpost%2Fde99bb20-4544-4fca-8b14-86e8f00eb143%2Fimage.png)

### 로그인하기

```s
$ docker login
```
```s
$ docker push ckstn0777/my-image-name:0.0
```

![](https://velog.velcdn.com/images%2Fckstn0777%2Fpost%2Faebb3e10-98f2-4ee3-aa64-4b6a40a04a19%2Fimage.png)

보면 알겠지만 변경된 레이어만 이미지 저장소로 전송되는 것이다.

![](https://velog.velcdn.com/images%2Fckstn0777%2Fpost%2Ff027c76b-d3c6-4fd2-be1b-5f452cadad35%2Fimage.png)

### 이미지 내려받기

```s
$ docker pull 사용자이름/my-image:0.0
```

![](https://velog.velcdn.com/images%2Fckstn0777%2Fpost%2Fb0539fd3-f0f0-49be-862d-2469ca4e1152%2Fimage.png)