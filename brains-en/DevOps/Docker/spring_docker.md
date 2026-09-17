# Deploying a Spring Project Using Docker

### Requirements for the Example
- To run a Docker container without lag, a sufficiently large EC2 instance is required. Therefore, an EC2 instance based on t2.medium has been prepared.
- If you don't mind significant lag and want to save as much money as possible, you can use a t2.micro instance provided in the free tier.

### EC2 Environment Setup

First, once the EC2 instance is created, update it and install Git.

```bash
# ec2 업데이트
$ sudo yum update -y
# git 설치하기
$ sudo yum install git
```

Next, install JDK.

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

If you enter all the commands above and `java --version` shows the following, you have succeeded.

```
openjdk 11.0.15 2022-04-19 LTS
OpenJDK Runtime Environment Corretto-11.0.15.9.1 (build 11.0.15+9-LTS)
OpenJDK 64-Bit Server VM Corretto-11.0.15.9.1 (build 11.0.15+9-LTS, mixed mode)
```

<br>

### Writing the Dockerfile
Create a Dockerfile in your Spring project directory.

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
  - This command declares that Docker will be built using JDK 11.

- `ARG JAR_FILE=./build/libs/DevopsTestKotlin-0.0.1-SNAPSHOT.jar`
  - This declares the location of the JAR file as an environment variable.
  - When you build the project, a JAR file will be created in the format `build/libs/xxxx.jar`. This variable stores the path to that file.
- `COPY ${JAR_FILE} app.jar`
  - This command references the project's JAR file location, fetches the JAR file, and copies it to the container's root directory as `app.jar`.
- `ENTRYPOINT ["java","-jar","/app.jar"]`
  - This command declares the **system entry point** for the Docker container when the Dockerfile is used to create a container via the Docker engine.
  - The command above means to execute `app.jar` located in the container's root using the `java -jar` command.
  - Once the Dockerfile above is complete, commit and push it to Git, then transfer the code to the EC2 instance.

<br>

### Writing docker-compose

Let's start with the premise that the project has already been copied to the EC2 instance.

We will deploy the Spring Boot project using Nginx. To use it with the Nginx image, we will create a `docker-compose.yml` file and bring it up with docker-compose. First, install Docker and docker-compose on the EC2 instance.

```bash
$ sudo yum install docker

# 버전 확인
sudo docker --version
```

If it shows the following, it's normal.

```
Docker version <version> , build f0df350
```

Next, we will install docker-compose.
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

Once this is done, create `docker-compose.yml` in the project's root directory and write the following.

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
Let's interpret the `docker-compose` file above.

For the Nginx image, it forwards incoming port 80 to port 80 inside the Docker container. This can be interpreted as letting HTTP requests flow directly into Nginx.

You can also see that `volumes` is used to move the Nginx configuration file from the EC2 instance to the container's `/etc/nginx/conf.d` folder and execute it.

However, you'll notice the `depends_on` attribute, which means Nginx will only start after the Spring container has been brought up.

And Spring is very simple. It means building the Docker image in the current directory and forwarding incoming signals on port 8080 directly to port 8080 inside the container.

However, we haven't touched or written Nginx's configuration file yet. So, let's create the Nginx configuration file, `conf.d`.

First, we will create `app.conf` inside the `nginx/conf.d` folder using the commands below.
```
mkdir nginx
mkdir nginx/conf.d
cd nginx/conf.d
sudo vim app.conf
```

Then, write the following in `app.conf`.

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

Let's briefly interpret the content above.

First, Nginx reads signals from external port 80. When passing the signal, it forwards it to Spring's port 8080. Since this forwarded signal is considered inbound from Spring's perspective, Spring, as configured in `docker-compose.yml` above, forwards the corresponding signal to port 8080 and injects it into the container.

And the three lines below that are all code for populating proxy headers.

<br>

### Let's bring up the project.

Let's build the project using the commands below.

```
# gradlew의 권한 변경
sudo chmod + ./gradlew

# 빌드
./gradlew build
```

Then, start Docker.
```
sudo systemctl start docker
```

Check if Docker is running correctly.
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

Now, bring up the project using docker-compose.
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

Afterwards, you can test with an HTTP request to see if it's working correctly.
