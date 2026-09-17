# LXC(Linux Container)

LXC와 비슷한 기술로 `chroot`라는 것이 존재한다.

chroot는 프로세스의 루트 디렉토리를 변경하는 것으로 이를 통해서 프로세스가 엑세스할 수 있는 디렉토리를 제한하거나 시스템 라이브러리와 관련된 라이브러리를 로드할 수 있다.

그러나 chroot에서 제어할 수 있는 파일이나 디렉토리에 대한 엑세스만으로 네트워크 및 프로세스등을 컨트롤할 수는 없다. 또한 FreeBSD에는 chroot를 발전시킨 툴로 jail이라는 기능이 탑재되어 있다.

**jail은 파일 시스템에 대한 엑세스 뿐만 아니라 프로세스 및 장치 등의 자원에 대해서도 제어가 가능하다.**

LXC가 jail과 유사한 개념으로 구현되어있다. **cgroups는 OS가 관리하는 다양한 리소스를 중앙에서 제어하기 위한 도구**다. 리소스를 그룹화하고 각 그룹에 대해 우선 순위 및 사용 가능한 리소스를 제한하거나 그룹을 분리하는 등의 기능을 제공한다.

cgroups에서 관리하는 대상은 파일 시스템이나 프로세스뿐만 아니라 cpu 리소스, 메모리, 각종 디바이스, 네트워크 패킷, 네트워크 인터페이스 등이다. cgroups의 사용 예로는 chroot와 같이 특정 디렉토리를 루트 디렉토리로 인식하도록 해서 cgroups의 namespace 기능을 사용해 다양한 자원을 격리함으로써 가상 환경을 구현한다.

컨테이너마다 분할되는 자원은 다음과 같은 것들이 존재한다.
- **프로세스 테이블**: 컨테이너 마다 별도의 프로세스 테이블을 관리해, **컨테이너의 프로세스에서 다른 컨테이너의 프로세스가 보이지 않도록 한다.**
- **파일 시스템**: 컨테이너마다 특정 디렉토리를 루트 파일 시스템으로 보이게 한다. `chroot`와 동일한 개념
- **네트워크**: 네트워크 namespace(nents)의 기능은 컨테이너마다 별도의 네트워크 설정을 구성한다. veth라는 가상 NIC 장치를 이용해 veth의 한쪽을 컨테이너 내부 네임스페이스에 할당한다.
- **cpu, memory 장치(/dev 다음 장치 파일)**: cgroups의 기능은 컨테이너에서 사용할 수 있는 범위를 제한한다.

![](https://www.redhat.com/rhdc/managed-files/styles/wysiwyg_full_width/private/virtualization-vs-containers.png.webp?itok=vvjopbiw)

Linux 컨테이너를 살펴보면 os의 내부는 물리적 자원을 관리하는 커널 공간과 사용자 프로세스를 실행하는 사용자 공간으로 나뉜다.

컨테이너형 가상화는 사용자 공간을 여러 공간으로 나누어 각각의 사용자 프로세스에서 보이는 리소스를 제한하는 것이다. 이와 같이 여러 **사용자 프로세스를 정리하여 분리한 공간이 바로 컨테이너다.**

> LXC(Linux Containers)는 단일 컨트롤 호스트 상에서 여러 개의 고립된 리눅스 시스템(컨테이너)들을 실행하기 위한 운영 시스템 레벨 가상화 방법이다. 리눅스 커널은 cgroups를 절충하여 가상화 머신을 시작할 필요 없이 자원 할당(cpu, memory, block i/o, network)을 한다. cgroups는 또한 애플리케이션 입장에서 프로세스 트리, 네트워크, 사용자 id, 마운트된 파일 시스템 등의 운영 환경을 완전히 고립시키기 위해서 namespace isolation을 제공한다. LXC는 cgroups + namespace의 애플리케이션을 위한 고립된 환경을 제공하는 것이다. Docker 또한 실행 드라이버의 하나로 LXC를 사용할 수 있으며 이를 통해 이미지 관리와 개발 서비스를 제공한다.

<br>

### 실습

Download lxc

```bash
ubuntu@master: sudo apt update
ubuntu@master: sudo apt install lxc
```

lxc 현재 사용할 수 있는 기능 확인

```bash
ubuntu@master: sudo lxc-checkconfig

# test 컨테이너 생성
sudo lxc-create -n test -t download

Distribution:
ubuntu
Release:
bionic
Architecture:
i386

Downloading the image index
Downloading the rootfs
Downloading the metadata
The image cache is now ready
Unpacking the rootfs

---

You just created an Ubuntu bionic i386 (20230118_07:43) container.

To enable SSH, run: apt install openssh-server
No default root or user password are set by LXC.
```

```bash
# 컨테이너 백그라운드 실행
ubuntu@master: sudo lxc-start -n test -d

# 컨테이너 확인
ubuntu@master: sudo lxc-ls --fancy

# 컨테이너 root 접속
ubuntu@master: sudo lxc-attach -n test

# test 계정 생성
ubuntu@master: sudo adduser test

# visudo 테스트 계정에 루트 권한 부여
ubuntu@master: visudo

test ALL=(ALL:ALL) ALL

# 컨테이너에 테스트 계정으로 접속
ubuntu@master: sudo lxc-console -n test

# openssh 설치
test@test:~$: sudo apt install openssh-server

# sshd active 확인
test@test:~$: sudo systemctl status sshd

# 네트워크 도구 설치
test@test:~$: sudo apt install net-tools

# 컨테이너 ip 확인
test@test:~$: ifconfig

# NAT로 eth0의 9999포트로 들어오는 tcp 패킷을 10.0.1.95 22번 포트로 전송
ubuntu@master: sudo iptables -t nat -A PREROUTING -i eth0 -p tcp --dport 9999 -j DNAT --to 10.0.1.95:22

```

```bash
ubuntu@master: ssh test@10.0.1.95  
  
The authenticity of host '10.0.1.95 (10.0.1.95)' can't be established.  
ED25519 key fingerprint is SHA256:oO9Yns7H//bnnuDBSAODiHW3UxsuHw6kEG1MJonKTXg.  
This key is not known by any other names  
Are you sure you want to continue connecting (yes/no/[fingerprint])? yes  
Warning: Permanently added '10.0.1.95' (ED25519) to the list of known hosts.  
test@10.0.1.95's password:  
Welcome to Ubuntu 18.04.6 LTS (GNU/Linux 5.15.0-56-generic i686)  
  
* Documentation: https://help.ubuntu.com  
* Management: https://landscape.canonical.com  
* Support: https://ubuntu.com/advantage  
Last login: Fri Jan 20 12:35:17 2023

# 컨테이너 중지  
sudo lxc-stop -n test  
  
# 컨테이너 삭제  
sudo lxc-destroy -n test  
  
# 컨테이너 확인  
sudo lxc-ls
```