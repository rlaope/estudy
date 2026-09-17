# Spring 프로젝트 Docker를 사용해 배포해보기

### 예제를 위해 요구사항
- Docker container를 렉 없이 올리기 위해 충분히 큰 용량의 ec2가 필요하다 따라서 t2.medium 기준으로 ec2를 마련했다.
- 자신은 렉이 많이 걸려도 상관없고 돈을 최대한 절약하기를 원한다고 하면 프리티어 단계에서 제공하는 t2.micro를 사용해도 상관 없다.


### EC2 환경 설정

우선 ec2가 생성 되었다면, update부터 시켜주고 git을 설치한다.

```bash
# ec2 업데이트
$ sudo yum update -y
# git 설치하기
$ sudo yum install git
```

다음으로 jdk를 설치해준다.

```bash
# aws coreetto 다운로드
sudo curl -L https://corretto.aws/downloads/latest/amazon-corretto-11-x64-linux-jdk.rpm -o jdk11.rpm

# jdk11 설치
sudo yum localinstall jdk11.rpm -y

# jdk version 선택
sudo /usr/sbin/alternatives --config java

# java 버전 확인
java --version

# 다운받은 설치키트 제거
rm -rf jdk11.rpm

```

위의 커맨드를 모두 입력하고 java --version을 통해 아래와같이 뜬다면 성공한겁니다.

```
openjdk 11.0.15 2022-04-19 LTS
OpenJDK Runtime Environment Corretto-11.0.15.9.1 (build 11.0.15+9-LTS)
OpenJDK 64-Bit Server VM Corretto-11.0.15.9.1 (build 11.0.15+9-LTS, mixed mode)
```

<br>

### Dockerfile 작성
스프링 프로젝트 파일에 Dockerfile을 작성해준다.

```Dockerfile
FROM openjdk:11-jdk

# JAR_FILE 변수 정의 -> 기본적으로 jar file이 2개이기 때문에 이름을 특정해야함
ARG JAR_FILE=./build/libs/DevopsTestKotlin-0.0.1-SNAPSHOT.jar

# JAR 파일 메인 디렉토리에 복사
COPY ${JAR_FILE} app.jar

# 시스템 진입점 정의
ENTRYPOINT ["java","-jar","/app.jar"]
```

- `FROM openjdk:11-jdk`
  - Docker를 올릴 때 jdk11 버전을 이용해서 올리겠다고 선언하는 커맨드다

- `ARG JAR_FILE=./build/libs/DevopsTestKotlin-0.0.1-SNAPSHOT.jar`
  - JAR 파일의 위치를 환경변수의 형태로 선언하는 것이다. 
  - 프로젝트를 빌드하게 되면 build/libs/xxxx.jar 의 형태로 jar file이 생성되어있을거다. 그 파일의 위치를 변수로 저장한다.
- `COPY ${JAR_FILE} app.jar`
  - 프로젝트의 jar 파일 위치를 참조하여 jar 파일을 가져와서 컨테이너의 루트 디렉토리에 app.jar의 이름으로 복사하는 커맨드입니다.
- `ENTRYPOINT ["java","-jar","/app.jar"]`
  - 도커파일이 도커엔진을 통해 컨테이너로 올라갈 때, 도커 컨테이너의 **시스템 진입점이 어디인지** 선언하는 커맨드다
  - 위의 커맨드에서는 java -jar 명령어를 이용해 컨테이너의 루트에 위치한 app.jar을 실행하라는 뜻의 커멘드입니다.
  - 위의 Dockerfile의 작성이 완료되었다면, git commit and push를 해준다음 ec2에 코드를 옮겨주면 된다. 

<br>

### docker-compose 작성

일단 ec2에 프로젝트가 복사가 되었다는 전제로 시작하겠다.  
  
springboot 프로젝트를 nginx를 이용해 배포할 예정이다. nginx의 이미지와 함께 사용을 하기 위해 docker-compose.yml 파일을 작성해 docker-compose로 올려보겠다. 일단 docker와 docker-compose에 ec2애 설치한다.

```bash
$ sudo yum install docker

# 버전 확인
sudo docker --version
```

다음과 같이 뜨면 정상이다.

```
Docker version <version> , build f0df350
```

다음으로, docker-compose를 설치하겠습니다.
```bash
sudo curl -L "https://github.com/docker/compose/releases/download/1.28.2/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
sudo chmod +x /usr/local/bin/docker-compose
sudo ln -s /usr/local/bin/docker-compose /usr/bin/docker-compose

### 버전 확인
docker-compose --version
```

```
docker-compose version <version> , build 67630359
```


여기까지 끝났다면, docker-compose.yml 프로젝트의 루트 디렉토리를 생성하고 다음과 같이 작성을 한다.

```yml
version: "3"
services:
  web:
    image: nginx
    ports:
      - 80:80
    volumes:
      - ./nginx/conf.d:/etc/nginx/conf.d
    depends_on:
      - spring
  spring:
    build: .
    ports:
      - 8080:8080
```
위의 docker-compose 파일을 해석해보겠습니다.
  
nginx 이미지의 경우 인바운드로 들어오는 80번 포트를 docker container 내부에 꽂히는 80번 포트로 포워딩해주는 기능을 수행합니다. 이는 Http 요청을 nginx 내부로 그대로 흘리겠다는 뜻으로 해석할 수 있습니다.
  
그리고 volumes를 통해서 ec2에 있는 nginx 설정 파일을 컨테이너의 /etc/nginx/conf.d 폴더로 옮겨서 실행시키는 것도 볼수 있습니다.
  
그런데 depends_on 이라는 속성이 보이는데, 이는 nginx가 실행되는 시점은 spring이 컨테이너로 올라간 다음에 올리겠다는 뜻 입니다.
  
그리고 spring은 매우 간단합니다. 현재 디렉토리에 있는 Docker image를 빌드시키고, 8080번 포트로 인바운드 들어오는 신호를 컨테이너 내부의 8080번 포트로 그대로 꽂아주겠다는 의미입니다.
  
그런데, 저희는 아직 nginx의 설정 파일을 건든적이 없으며, 작성한 적도 없습니다. 따라서 nginx의 설정 파일인 conf.d를 작성하겠습니다.
  
우선 아래의 커맨드를 통해서 nginx/conf.d 폴더 내부에 app.conf를 생성하겠습니다.
```
mkdir nginx
mkdir nginx/conf.d
cd nginx/conf.d
sudo vim app.conf
```

그리고 app.conf에 다음과 같이 작성한다.

```
server {
    listen 80;
    access_log off;

    location / {
        proxy_pass http://spring:8080;
        proxy_set_header Host $host:$server_port;
        proxy_set_header X-Forwarded-Host $server_name;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    }
}
```

대충 위의 내용을 해석하겠습니다.
  
일단은 nginx는 외부의 80번 포트의 신호를 읽어냅니다. 그리고 신호를 pass 시킬때는 spring의 8080번 포트로 포워딩시켜서 패스시킵니다. 그리고 이렇게 포워딩 된 신호는 spring 입장에서는 인바운드에 해당하기 때문에 위에서 docker-compose.yml을 작성한대로 spring은 그에 해당하는 신호를 8080번 포트로 포워딩시켜서 container 내부로 꽂아버립니다.
  
그리고 그 아래 3개의 라인은 모두 proxy에 header를 채우는 코드들입니다.

<br>

### 프로젝트를 올려보자.

아래의 커맨드를 통해 프로젝트를 빌드시켜보자

```
# gradlew의 권한 변경
sudo chmod + ./gradlew

# 빌드
./gradlew build
```

그리고 도커를 실행시킨다.
```
sudo systemctl start docker
```

도커가 정상 작동하고 있는지 확인한다.
```
systemctl status docker
```

```
docker.service - Docker Application Container Engine
   Loaded: loaded (/usr/lib/systemd/system/docker.service; disabled; vendor preset: disabled)
   Active: active (running) since 수 2022-04-20 17:48:42 UTC; 51s ago
     Docs: https://docs.docker.com
  Process: 3380 ExecStartPre=/usr/libexec/docker/docker-setup-runtimes.sh (code=exited, status=0/SUCCESS)
  Process: 3379 ExecStartPre=/bin/mkdir -p /run/docker (code=exited, status=0/SUCCESS)
 Main PID: 3384 (dockerd)
    Tasks: 8
   Memory: 119.3M
   CGroup: /system.slice/docker.service
           └─3384 /usr/bin/dockerd -H fd:// --containerd=/run/containerd/containerd.sock --default-ulimit nofile=32768:65536
```

이제 docker-compose를 통해 프로젝트를 올린다.
```
# docker-compose를 백그라운드에서 동작시키기
docker-compose up --build -d
```
```
Building with native build. Learn about native build in Compose here: https://docs.docker.com/go/compose-native-build/
Creating network "order-example-kotlin_default" with the default driver
Building spring
Sending build context to Docker daemon  45.52MB
Step 1/4 : FROM openjdk:11-jdk
11-jdk: Pulling from library/openjdk
6aefca2dc61d: Pulling fs layer
967757d56527: Pulling fs layer
c357e2c68cb3: Pulling fs layer
c766e27afb21: Pulling fs layer
a747e81e6111: Pulling fs layer
2859d18181fd: Pulling fs layer
3c6d59134c80: Pulling fs layer
c766e27afb21: Waiting
a747e81e6111: Waiting
3c6d59134c80: Waiting
2859d18181fd: Waiting
c357e2c68cb3: Verifying Checksum
c357e2c68cb3: Download complete
967757d56527: Verifying Checksum
967757d56527: Download complete
6aefca2dc61d: Verifying Checksum
6aefca2dc61d: Download complete
a747e81e6111: Verifying Checksum
a747e81e6111: Download complete
2859d18181fd: Verifying Checksum
2859d18181fd: Download complete
c766e27afb21: Verifying Checksum
c766e27afb21: Download complete
3c6d59134c80: Verifying Checksum
3c6d59134c80: Download complete
6aefca2dc61d: Pull complete
967757d56527: Pull complete
c357e2c68cb3: Pull complete
c766e27afb21: Pull complete
a747e81e6111: Pull complete
2859d18181fd: Pull complete
3c6d59134c80: Pull complete
Digest: sha256:95b2daeb07a18121a4309a053ff99aa741888528e5da068beef36db092a03e25
Status: Downloaded newer image for openjdk:11-jdk
 ---> e67a33049aa6
Step 2/4 : ARG JAR_FILE=./build/libs/DevopsTestKotlin-0.0.1-SNAPSHOT.jar
 ---> Running in a1ebd6481830
Removing intermediate container a1ebd6481830
 ---> e448756e884e
Step 3/4 : COPY ${JAR_FILE} app.jar
 ---> 1c1a9cfead2f
Step 4/4 : ENTRYPOINT ["java","-jar","/app.jar"]
 ---> Running in 7dfbfc10fc7e
Removing intermediate container 7dfbfc10fc7e
 ---> e1bfd62d0398
Successfully built e1bfd62d0398
Successfully tagged order-example-kotlin_spring:latest
Pulling web (nginx:)...
latest: Pulling from library/nginx
1fe172e4850f: Pull complete
35c195f487df: Pull complete
213b9b16f495: Pull complete
a8172d9e19b9: Pull complete
f5eee2cb2150: Pull complete
93e404ba8667: Pull complete
Digest: sha256:694f2ecdb88498325d70dbcb4016e90ab47b5a8e6cd97aaeec53f71f62536f99
Status: Downloaded newer image for nginx:latest
Creating order-example-kotlin_spring_1 ... done
Creating order-example-kotlin_web_1    ... done
```

그 후 정상적으로 동작하고 있는지 HTTP 요청으로 테스트를 해보면된다. 