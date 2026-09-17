# Bastion 서버로 Private EC2에 접속하는 방법 + Hi-v2 트러블 슈팅

### 아키텍처

프로젝트를 진행하면서 나는 다음과 같은 아키텍처 구성의 AWS 환경을 구축했다.

![](https://github.com/GSM-MSG/hi-infrastructure-global-v1/raw/master/architecture/hi_architecture.png)

처음으로 메인 서버 EC2를 private subnet에 띄우고 Bastion을 통해서 private ec2로 접속하는 과정을 거쳤다.

bastion과 main server ec2는 terraform 스크립트를 통해 구축했다.

```tf
resource "aws_instance" "hi-bastion" {
    ami = "ami-04cebc8d6c4f297a3"
    instance_type = "t2.micro"
    subnet_id =  "${aws_subnet.hi-public-subnet-2a.id}"
    vpc_security_group_ids = [aws_security_group.hi-bastion-sg.id]
    key_name = "hi-key"

    tags = {
        Name = "hi-bastion"
    }
}

resource "aws_instance" "hi-main-server" {
    ami = "ami-04cebc8d6c4f297a3"
    instance_type = "t3.micro"
    subnet_id = "${aws_subnet.hi-private-subnet-2a.id}"
    vpc_security_group_ids = [aws_security_group.hi-main-server-sg.id]
    key_name = "hi-key"
    associate_public_ip_address = false
    source_dest_check = false
    
    tags = {
        Name = "hi-main-server"
    }
}
```

main server ec2에 접속하기 위해서는 bastion 서버에 private ec2 key pair를 저장해야한다.

현재 키페어는 `hi-key`를 사용하고있고, 내 컴퓨터에 저장이 되어있다. 이 키페어를 bastion로 보내려고 한다.

다음과 같이 내 컴퓨터 터미널에 입력을 해준다.

```zsh
$ sudo scp -i <key-pair-path\>/<key-pair\>.pem <key-pair-path\>/<key-pair\>.pem <username\>@<public_ip_dns\>:/home/ubuntu
```

그러면 나의 키페어가 bastion서버에 업로드된 것을 볼 수 있다.

<br>

## 트러블 슈팅

### keypair are too open

```
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@         WARNING: UNPROTECTED PRIVATE KEY FILE!          @
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
Permissions 0755 for 'hi-key.pem' are too open.
It is required that your private key files are NOT accessible by others.
This private key will be ignored.
Load key "hi-key.pem": bad permissions
ubuntu@ec2-43-201-86-41.ap-northeast-2.compute.amazonaws.com: Permission denied (publickey).
```

keypair가 너무 오픈되어있다는 오류가 뜰 때가 있다. 그때는 `chmod 400 <keypair>.pem` 을 입력해주면 바로 해결이 된다.

### keypair permission denied

```
hi-admin@ec2-43-201-86-41.ap-northeast-2.compute.amazonaws.com: Permission denied (publickey).
scp: Connection closed
```

이건 조금 멍청한 짓인데 security gruop인바운드가 오픈되어있는게 없다거나 그런 오류인줄 알았지만 `ubuntu@<public ip DNS>`  public ip dns를 넣어줘야했지만 그냥 public ip를 넣었거나 아니면 ubuntu처럼 접속 username을 잘못 입력해서 Permission denied가 뜨기도 했다.. 잘 알아두고 삽질 적게 하자!