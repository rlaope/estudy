# Namespace, Container + runc syscall tracking

docker 공식문서에서는 namespace = container 라는 설명이 존재한다.

container는 무엇일까? linux 컨테이너를 만들고 사용할 수 있도록 하는 컨테이너화 기술, 공유된 운영체제에서 격리되어 실행할 수 있는 형식의 소프트웨어 가상화 방법이다.

어떤식으로 container는 가상화를 하는 것일까? 왜 container = namepace라는 설명을 하는 것일까?

**결론부터 말하면 docker는 namespace라고 불리는 기술로 container의 격리된 공간을 제공한다.**

이해못해도 괜찮다. 오히려 이해 못해야 정상이다. 자 이제 이 문장을 이해해보자.

```yml
volumes:
  wordpress:
    driver_opts:
      type: none
      o: bind
      device: /User/Khope/data/woredpress
  mariadb:
    driver_opts:
      type: none
      o: bind
      device: /User/Khope/data/mariadb
```

이렇게 내 host 공간에 마운트하는 volume을 생성하는 컴포즈 파일을 작성했다.

이러면 이제 id를 치면 khope(host)와 (/data/mariadb) ctr 사용자가 생겼을 것이다.(/var/lib/mysql)

container에서 file을 하나 생성해보고 그 파일의 소유주를 찍어보자/ touch file.txt를 진행하고 `ls -al | grep "file"`를 진행해봤다.

```bash
# /var/lib/mysql (in container)
$ ls -al | grep "file"
>> ctr              ctr

# ~/data/mysql
$ ls -al | grep "file"
>> khope              khope
```

보면 컨테이너에서 file 그렙한 사용자와 호스트에서 grep한 사용자가 서로 자신의 소유주라고 명시하고 있다.

watch ps ax를 통해 프로세스의 출력을 확인해보면 어떨까? 같은 프로세스인데 container와 host에서 pid가 다른것도 확인이가능하다. (직접해보쇼)

docker와 virtual machine이 다른 것을 확실하게 이 부분에 대해서 알 수 있다.

바로 Host, Container의 자원을 공유한다는 것이다. 그리고 우린 이걸 사용할때 일정 수준의 격리를 통해 서로 간섭없이 실행될 수 있도록 기능하게 한다.  
그렇게 기능할 수 있도록 하는것이 바로 namespace이다. **정리해보면 일정 수준 격리는 container이고 독립된 가상공간은 namespace라고 정의해볼 수 있다.**

namespace = container를 위에서 언급했었는데 정확히는 namespace에 의해 만들어진 공간에서 container가 활동하므로 종속적인 의미라고 생각될수도 있겠다.

<br>

### containerd

pstree로 process 트리 구조를 보면 containerd, containerd-shim 등을 관찰할 수 있는데 containerd-shim에 의해서 watch가 돌아가는 것을 볼 수 있다.  
여기서 containerd는 무엇일까? 바로 단순성과 경고성 이식성에 중점을 둔 표준 컨테이너 런타임이다.

ㅋㅋ 말이 너무 어렵다. 쉽게 말하면 **호스트 시스템의 전체 컨테이너의 수명 주기를 관리하는 것이다.**

containerd의 runtime 요구사항은 매우적으며 linux container와 대부분 상호작용은 runc를 통해서 처리된다.

개념이 너무 많이 나오니까 runc는 일단 무시하고 간단하게 좀 알아보자

- docker build
- docker pull
- docker run

위의 명렁어들을 입력하면 dockerd로 전달되고 dockerd는 자동(기본)으로 containerd를 실행시킨다. (실제적인 컨테이너 관리는 runc가 하긴함.)

아 위의 문장 너무 어렵다고 생각될 수 있다. 왜냐면 이번 글에서는 얻을 수 없는 정보들이기 때문이다. 절망하지말자. 그냥 이런게 있구나 하고 알아두자.

> cli <-> dockerd -> containerd -> runc 이 구조로 통신한다.

우리가 cli를 치면 dockerd랑 통신하고 dockerd가 시작되기전에 containerd가 컨테이너를 관리하고 생성하기 때문에 dockerd에 의해서 자동으로 먼저 실행이 되고 contaierd는 실제로 컨테이너를 생성하고 실행하는데는 runc를 사용한다.


좀 더 파보자 docker log 볼라면 `strace -f -p $(pidof container) -o docker_log.txt`를 쳐라. 위에거 쓰면 containerd 프로세스를 추적한다.

이어서 `docker run -it --name debina debain:buster`를 통해 컨테이너를 새로 생성해 실행시켜보자

그럼 attached 됐다는 메시지와 함께 로그 파일이 남고 이를 까보면 syscall 추적이 가능하다.

![](https://velog.velcdn.com/images/az0856/post/33d0de39-bc3d-43a5-83aa-35636af59628/image.png)

대충 이렇게 나오는데 보면 rexecve 명령어로 runc를 실행하는 모습을 확인할 수 있다. runc가 실행된 후 로그를 보자

![](https://velog.velcdn.com/images/az0856/post/09c57fe0-80cd-4038-b3e8-717c8a38565d/image.png)

unshare이라는 syscall을 호출하는데 바로 윗줄을 보면 `prctl(PR_SET_NAME "runc:[1:CHILD]") = 0` 이라는 명령어가 보이는데 ptcl은 process의 thread를 조작하는 명령어고 PR_SET_NAME 이라는 옵션을 통해서 호출 thread의 이름을 지정한다. 하지만 스레드는 따로 생성안해놔서 미리 생성해주나 하고 찾아봤더니, PR_SET_NAME이 **프로세스 이름도 바꿀 수 있고 프로세스 상에서 쓰이면 프로세스 이름이 바뀌고 소속된 스레드도 영향을 받으며 스레드에서 호출하면 그 스레드의 이름만 바뀐다고 한다.**

여기서 unshare을 호출하면 자식 프로세스가 새로 생기게 된다. 그리고 CLONE_NEWPID라는 옵션이 있는데 새로운 pid namespace를 호출하 생성하고(기준 프로세스와 공유되지 않는) unshare를 호출한 프로세스는 새 namespace로 이동안하며 호출 프로셋에 의해 생성된 자식은 새로운 namespace에서 init(1)의 역할을 할 수 있도록 반영한다.

한마디로 새로운 프로세스를 만들어서 부모 프로세스로 fork 때리는거다.

**간지나니까 영어문서 넣겠다**. -> https://man7.org/linux/man-pages/man2/unshare.2.html 

정리해보면 컨테이너는 namespace로 부터 격리된 공간이며 runc가 실제로 컨테이너 생성 및 관리를 하고있으며 runc의 하위 스레드가 unshare() syscall을 통해서 새로운 namespace를 만든다.