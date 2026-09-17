# Kernel

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FNteRW%2FbtrMPPw33ab%2FNPdVAQTykVgiZHxGdRKAh1%2Fimg.png)

kernel은 번역기로 돌리면 알맹이라는 번역 결과를 얻을 수 있는데, 왜 알맹이 인 것일까? unix 구조를 살펴보면 바로 이해가 가능하다.

맨 위에 applcations, 그리고 가작 안쪽에 작은 알맹이로서 kernel이 존재한다. 

그 외에 shell, system calls, library routines등이 applications의 요청을 받아 중요한 역할을 수행한다.

근데 여기서 알맹이라는 말도 이해도 조금 추상적이다. 그래서 유닉스에서 운영체제의 핵심 요소라고 불릴 수 있는 요소는 무엇일까? 이를 이해하기 위해서는 unix system 프로그램 동작 과정을 더 자세히 살펴보아야한다.

unix 운영체제에는 두 가지 mode가 존재한다. 바로 **user mode, kernel mode** 이다.

user mode는 일반 사용자가 프로그램을 실행하면, 구동되는 모드다. 조금 더 쉽게 설명하면 우리가 크롬을 열게되면, 이때 컴퓨터는 유저모드에서 크롬이라는 application의 샐행을 처리한다. 다만 컴퓨터 리소스에 직접적으로 접근할 수 없다. 즉, 일반 유저가 자유롭게 컴퓨터 자원을 접근하는 것은 unix에서는 막혀있다.

kernel mode는 일반 사용자가 직접적으로 접근할 수 없는 모드라고 간단하게 설명하면 된다.

여기까지가 간단한 설명이고 이제 본격즉으로 둘을 이해하기 위해 unix에서 프로그램 실행 과정을 이해해보자/

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbjoZ4I%2FbtrMQNr9TvA%2FPNkYH0W0CQV707NnZEIWuK%2Fimg.png)

위 그림은 c로 작성된 프로그램 user application이 실행되는 과정을 user mode와 kernel mode 두 모드 중심으로 펴현한 것이다.

프로그램을 클릭해 시작한다고 생각해보자, 프로그램을 실행하려면 당연히 컴퓨팅 자원이 필요하다. 당장 생각 나는 것은 cpu, memeory, disk 같은것 외에도 네트워크 연결(소켓), 키보드, 마우스, 모니터와 같은 주변 장치와 같은 것들도 모두 컴퓨팅 자원으로써 프로그램 실행에 기여한다.

그럼, 프로그램은 어떻게 자원을 받아서 실행되는 걸까? 바로 user mode, kernel mode의 긴밀한 연결을 통해 프로그램은 자원을 할당받는다. 앞서 언급했든 user mode는 직접적으로 컴퓨팅 리소스에 접근할 수 없기 때문에, user mode의 user는 `system call`을 통해서 컴퓨터 리소스를 할당해달라는 요청을 **system call interface** 에 보낸다. system call interface는 user mode, kernel mode를 매개해주는 역할을 수행한다.

kernel mode에 진입하면 system call interface가 받은 요청을 처리하기 위해, 우선 system call vector table이라는 곳으로 요청을 매개한다. system call vector table에서는 해당 시스템 콜 코드가 어디에 위치해 있는지 알려주는 역할을 수행한다. open이라는 시스템콜 요청이 들어오면 system call vector table에서는 해당 시스템 콜을 수행하기 위해 테이블을 검색한다. 그러다 해당하는 시스템 콜을 찾으면 해당 위치로 가서 코드를 실행한다.

최종적으로 실행 결과를 system call interface에게 돌려주고, 이는 다시 user application에 전달된다. 다 쓰고나니 내용이 되게 많아보이지만, 핵심은 간단하다. user mode 에서는 컴퓨터 리소스에 접근할 수 없고 항상 kernel mode를 거친다는 것이다.

<br>

### Kernel

자 다시 언급했다. Kernel 이제 커널이 무엇일까? 위에서 자세히 알아보았다싶이 커널은 앞의 리소스를 할당해 주는 주체 이것이 바로 커널이다. 커널은 단단한 알맹이로서 전화기, 노트북, 서버 컴퓨터 가리지 않고 **하드웨어의 모든 주요 기능을 제어하고 조율한다.**

유저는 kernel에 직접 접근하는 대신, shell, library routines(c의 lib)을 통해 system call을 system call interface에 전달하거나 아니면 곧바로 system call interface에 system call 요청을 보내는 식으로 해당 프로세스 실행에 필요한 자원을 요구한다.