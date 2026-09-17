# Nginx

### Relationship with Apache

Apache has been a beloved web server, never losing its top spot since 1996.

However, being old, it may sometimes not be compatible with new technologies.

Currently, both Apache and Nginx are widely used in Korea.

### Features
Unlike Apache, it has an asynchronous event-driven architecture.

It is specialized in handling concurrent connections.

1. Event-Driven Processing Architecture

An Event-Driven processing architecture handles multiple connections asynchronously through an Event-Handler, allowing logic to proceed for whichever connection is processed first.

The primary purpose of this technique is to create interactive programs, similar to PCP processing.

### Role of an HTTP Server

Complies with the HTTP protocol.

### Role as a Reverse Proxy

When a client sends a request to the proxy server, the proxy server retrieves data from the backend server (reverse server).

If requests are sent directly to the app server, one process must be in a response/waiting state, but a Reverse Proxy can distribute requests.

### Basic Principle
Nginx consists of one master process and multiple worker processes.

The master process is responsible for managing worker processes.

Worker processes are responsible for actually handling requests.

<br>

## Applying Nginx

### Installation

ubuntu

```
sudo apt-get install nginx
```

Verify the installation using the command below.
```
nginx -v
```

### Commands

Check version
```
nginx -v
```

Restart
```
sudo service nginx restart
```

Stop
```
sudo service nginx stop
nginx -s stop
```

Apply configuration
```
sudo service nginx reload
nginx -s reload
```

Check configuration
```
nginx -t
```

### Configuration

nginx.conf
```conf
user       www www; # Specifies the permissions for worker processes. Avoid using root if possible.
worker_processes  5;  # How many processes to use when handling requests. Usually auto
error_log  logs/error.log;	# Location of logs
pid        logs/nginx.pid;	# Sets the process ID
worker_rlimit_nofile 8192;	# Limits the number of files that can be opened for worker processes

events {					# Connection-related processing
  worker_connections  4096;  # Maximum number of connections a single process can handle
}

http {			# Web traffic processing block
  include    conf/mime.types;
  include    /etc/nginx/proxy.conf;
  include    /etc/nginx/fastcgi.conf;
  index    index.html index.htm index.php;	# Sets the name of the index file to display when connecting to the server

  default_type application/octet-stream;	# Specifies the default mime.type value for the response
  log_format   main '$remote_addr - $remote_user [$time_local]  $status '	# Specifies the log format
    '"$request" $body_bytes_sent "$http_referer" '
    '"$http_user_agent" "$http_x_forwarded_for"';
  access_log   logs/access.log  main;		# Sets the access log management file
  sendfile     on;			# Manages sendfile() settings
  server_names_hash_bucket_size 128; # Maximum number of hosts
  keepalive_timeout   65;	# Sets the duration for maintaining server connections

  server {	# Virtual server settings. IP-based and domain-based settings are possible
    listen       80;	# Listens for requests on port 80
    server_name  domain1.com www.domain1.com;	# Sets the virtual server name
    access_log   logs/domain1.access.log  main;	# Location to log requests
    root         html;		# Root directory

    location ~ \.php$ {	# Configures settings based on the requested URI
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
        server localhost:8082 weight=5 max_fails=3 fail_timeout=10s;	# Means sending to port 8082 of the server.
        keepalive 100;		# If keepalive is off, a handshake occurs for every request, so this sets the maximum number of connections to maintain.
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
