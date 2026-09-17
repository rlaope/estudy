# permission denied while trying .. Docker demon socket Troubleshooting

After running `sudo apt-get install docker, docker compose` to deploy a Spring Boot Application using Docker and Docker Compose.

However, when `docker ps` is entered,

```
permission denied while trying to connect to the Docker daemon socket at unix:///var/run/docker.sock: Get "http://%2Fvar%2Frun%2Fdocker.sock/v1.24/containers/json": dial unix /var/run/docker.sock: connect: permission denied
```

This error message appeared.

Docker commands were not working due to a Docker permission issue. The problem was that I was trying to run Docker as a non-root user.

<br>

## Solution

### Create docker group

```zsh
$ sudo groupadd docker
```

### Add user to docker group

```zsh
$ sudo usermod -aG docker $USER
```

### Log in to the new group

```zsh
$ newgrp docker
```

By doing this, you can see it working correctly.
