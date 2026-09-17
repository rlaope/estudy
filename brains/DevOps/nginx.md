# Nginx

### 아파치와의 관계

아파치는 1996년 부터 단 한번도 1등을 놓친적 없을 정도로, 사랑받는 웹서버이다.

하지만 오래된 만큼 신기술에 호환되지 않는 경우도 있을 수 있다.

현재 국내에선 아파치와 nginx 모두 많이 사용되고 있다.

### 특징
아파치와 다르게 비동기 이벤트 기반 구조를 가지고 있다.

동시접속 처리에 특화되어 있다.

1. Event-Driven 처리 기반 구조

Event-Driven 처리 기반 구조는 여러 개의 커넥션을 모두 Event-Handler를 통해 비동기 방식으로 처리해 먼저 처리되는 것부터 로직이 진행되도록 한다.

이러한 기법의 주 사용 목적은 대화형 프로그램을 만드는 데 사용하는데 PCP 처리와 유사하다.

### HTTP 서버의 역할

http 프로토콜을 준수한다.

### 리버스 프록시로서의 역할

클라이언트가 프록시 서버에 요청을 보내면, 프록시 서버가 배후 서버(reverse server)로 부터 데이터를 가져오는 역할을 한다.

직접 app 서버에 요청하는 경우, 프로세스 한개는 응답/대기 상태가 되어야 하는데, Reverse Proxy를 통해 요청을 배분해 줄 수 있다.

### 기본 원리
nginx는 하나의 master process와 여러개의 worker process로 구성되어 있다.

master process는 worker process를 관리하는 프로세스다.

worker process는 실질적으로 요청을 처리하는 프로세스다

<br>

## Nginx 적용하기

### 설치

ubuntu

```
sudo apt-get install nginx
```

아래 명령어를 통해 설치를 확인한다.
```
nginx -v
```

### 명령어

버전 확인
```
nginx -v
```

재시작
```
sudo service nginx restart
```

중지
```
sudo service nginx stop
nginx -s stop
```

설정 반영
```
sudo service nginx reload
nginx -s reload
```

설정 체크
```
nginx -t
```

### 설정

nginx.conf
```conf
user       www www; # 워커프로세스의 권한을 지정. 되도록 root는 사용하지 말자.
worker_processes  5;  # 요청을 처리할 때 몇개의 프로세스를 사용할지. 보통 auto
error_log  logs/error.log;	# 로그를 위치
pid        logs/nginx.pid;	# 프로세스 id 설정
worker_rlimit_nofile 8192;	# worker processes를 위해 열 수 있는 파일 개수 제한

events {					# connection 관련 처리
  worker_connections  4096;  # 하나의 프로세스에서 처리할 최대 connection 수
}

http {			# 웹 트래픽 처리 블록
  include    conf/mime.types;
  include    /etc/nginx/proxy.conf;
  include    /etc/nginx/fastcgi.conf;
  index    index.html index.htm index.php;	# 서버에 접속했을 때 index로 보여줄 이름 설정

  default_type application/octet-stream;	# response의 default mime.type 값 지정
  log_format   main '$remote_addr - $remote_user [$time_local]  $status '	# 로그형식 지정
    '"$request" $body_bytes_sent "$http_referer" '
    '"$http_user_agent" "$http_x_forwarded_for"';
  access_log   logs/access.log  main;		# 접속 로그 관리 파일 설정
  sendfile     on;			# sendfile() 설정 관리
  server_names_hash_bucket_size 128; # 최대 호스트 개수
  keepalive_timeout   65;	# 서버 연결 유지를 위한 지속시간 설정

  server {	# 가상 서버 설정. IP 기반 설정, 도메인 기반 설정 가능
    listen       80;	# 80포트 요청을 들음
    server_name  domain1.com www.domain1.com;	# 가상 서버 이름 섲렁
    access_log   logs/domain1.access.log  main;	# 요청에 대한 로그를 찍어줄 곳
    root         html;		# root 디렉토리

    location ~ \.php$ {	# 요청 URI에 따른 구성 설정
      fastcgi_pass   127.0.0.1:1025;
    }
  }

  server {
    listen       80;
    server_name  domain2.com www.domain2.com;
    access_log   logs/domain2.access.log  main;

    # 정적 파일
    location ~ ^/(images|javascript|js|css|flash|media|static)/  {
      root    /var/www/virtual/big.server.com/htdocs;
      expires 30d;
    }

    location / {
      proxy_pass      http://127.0.0.1:8080;
    }
  }

  upstream big_server_com {
    server 127.0.0.3:8000 weight=5;
    server 127.0.0.3:8001 weight=5;
    server 192.168.0.1:8000;
    server 192.168.0.1:8001;
  }

  server { # simple load balancing
    listen          80;
    server_name     big.server.com;
    access_log      logs/big.server.access.log main;

    location / {
      proxy_pass      http://big_server_com;
    }
  }
}

upstream smoothbear {
        server localhost:8082 weight=5 max_fails=3 fail_timeout=10s;	# 서버의 8082 포트로 보내준다는 의미. 
        keepalive 100;		# keepalive가 꺼져있다면 매 요청마다 핸드쉐이크가 발생하기 때문에, 최대 몇개의 커넥션을 유지할건지 설정
}

server {
        listen 80;
        server_name api.smooth-bear.live;

        location ~ /\.ht {
                deny all;
        }

        location / {
                proxy_pass http://smoothbear;
                proxy_redirect off;
                proxy_set_header Host $host;
                proxy_set_header   X-Real-IP $remote_addr;
                proxy_set_header   X-Forwarded-For $proxy_add_x_forwarded_for;
        }
}
```
