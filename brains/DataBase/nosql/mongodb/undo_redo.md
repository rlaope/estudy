# MongoDB 데이터 복구(UNDO, REDO)

MongoDB에서는 트랜잭션을 지원하지 않는다.

그렇기에 데이터 복구가 안된다는 단점이 있다.

여러 환경에서 데이터 복구를 구현하려면 어떻게 해야하고 어떤 것을 지원하고 있는지 알아보겠다.

- UNDO - 복구, 원상태 롤백
- REDO - 복구, 실패시 재시도

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbzyU8S%2FbtrmY4M91aK%2FCEMPXOxQvhVCFjQ1lFk2Sk%2Fimg.png)

몽고디비에서는 UNDO와 REDO가 존재하지 않는다 -> DB가 보장되지 않음 (정보가 100퍼센트 정확하하지 않는다.)

해결하기 위해서는? -> 이런 경우에는 프로그래밍 레벨에서도 처리해줄 수는 있다. (try catch 같은 것으로 실패했을 경우 다시 update, insert)

하지만 위와 같은 방법을 해도 몽고 디비가 데이터를 보장해주지는 않는다.

그렇다면 몽고디비에서는 undo, redo를 프로그래밍으로 처리할 수 밖에 없는가?

그건 아니고 방법은 맨 아래에서 설명하도록 하겠다.

<br>

### 위 그림에서 서버가 하나라도 망가지면 시스템 전체에 문제가 생기는 문제점이 있다.

따라서 우리는 **리플리카**라는 복제 방법을 사용해볼 수 있다.

서버들이 각각 3개씩 복제된 상태에서

1번 서버의 문제 -> 2번 서버가 돈다. -> 2번 서버가 돌 동안 1번서버에서는 복구 작업을 시작 -> 1번과 2번이 두개 다 무너짐 -> 3번 서버가 돔 -? 1, 2번은 복구 작업을 시작.

리플리카를 만들 때는 3개를 보통 한 쌍으로 만든다. 이 3개를 묶어 **리플리카 셋**이라고 한다.

이러한 과정을 데이터가 RAID 된다고 한다.

> RAID(Redundant Array of Independent Disk: 독립된 디스크의 복수 배열) - 여러개의 디스크를 묶어 하나의 디스크처럼 사용하는 기술

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fbxm1dj%2FbtrmWsvjQh3%2FNISd4zrEOIvO7DDBJbcHC1%2Fimg.png)


### OPLog

여기서 OPLog라는 것이 등장하게 되었는데. OPLog는 Operand를 저장하는 영역으로

10이 들어온다면 10을 저장해! 라는 Operation 저장하게 된다.

그리고 Primary, Secondary에 각각 연결된 OPLog 들 끼리 동기화를 위해 서로 리퀘스트하여 통신을 한다.

통신하는 순서는

1. OPLog끼리 리퀘스트
2. 새로 들어온 오퍼레이션이 있는지 확인
3. 응답이 있으면 명령어 저장

위와 같은 작업을 **HeartBeat**이라고 한다.

HeartBeat는 그물처럼 서로 연결되어져 있다.

> Polling ?
> 하나의 장치또는 프로그램이 충돌 회피를 위해 동기화 처리 등을 목적으로 다른 장치또는 프로그램의 상태를 주기적으로 검사하여 일정한 조건을 만족할 때 까지 송수신 등의 자료처리를 하는 방식이다.


OPLog라는 것을 통해서 우리는 프로그래밍 레벨이 아닌 몽고디비레벨에서 데이터 복구를 진행할 수 있다.

<br>

## REDO

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FcO0n08%2FbtrmY4zGm6j%2FdQhf1j0hZzbkoV0lGhCzwK%2Fimg.png)


위 그림에서는 Primary Server에는 1, 2가 저장이 되어있다.

Secondary Server에는 1만 저장되어 있는 상태이다.

OPLog에는 모두 1, 2를 저장하라는 명령이 들어와 있고 P와 S Server 데이터를 비교해
2가 저장되어 있지 않은 것을 확인하고 OPLog를 재실행한다.

이 때, 실패하게 되면 S Server와 연동된 OPLog의 내용은 삭제되고 P Server에 연동된 OPLog와 HeartBeat를 하여 동기화 한다.

그리하여 다시 1, 2를 저장하라는 명령이 들어오면 명령을 실행한다.

이 과정을 명령이 성공할 때까지 실행한다. -> REDO

<br>

### UNDO

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FMLTRT%2FbtrmXRA6hMd%2FRqLmTukKerGSVEThVaYOM1%2Fimg.png)

위 그림에서 19.9초에 HeartBeat가 실행이 되었고, 20초에 Primary OPLog에 2를 5로 수정하라는 명령이 들어왔다고 가정해보자,

일 명령이 수행되어 Primary Server의 2가 5로 변경이 되었고 다음 HeartBeat는 21.9초에 수행되어 2를 5로 수행하라는 명령을 받게 된다.

그러나 21.5초때 Primary Server와 OPLog가 고장이 났다고 하자.

이렇게 되면 남은 두개의 Secondary Server중 하나가 Primary Server로 산출되어 두 개의 Server로 운용이 된다.

이 타이밍에서 다른 클라이언트가 5를 요청하면 데이터를 확인할 수 없다 -> 지금은 데이터 복구 X

10초 정도 후 기존의 Primary Server가 살아나게 되면 자동으로 Secondary Server가 된다.

서로 HeartBeat를 통해 동기화를 진행하여 Operand 기준이 되어 2가 5로 수정되는 명령이 모두 들어오게된다.

복구 이후 HeartBeat의 Polling 시간을 매우 짧게 진행하여 이러한 문제를 방지할 수 있으나 MongoDB에 과부화가 발생하게 된다..

위와 같은 문제를 해결하기 위해 `저널링` 이라는 변경사항을 반영하기 전에 변경사항을 추적할 수 있는 어떠한 데이터들을 보관하는 시스템을 통해 복구를 진행한다. 

**타이밍 : OPlog에 2를 5로 수정하라는 명령이 들어와 수행직전에 파일로 명령을 기록한다.**

만일 위의 상황처럼 서버가 죽어버린 경우, 저널링부터 찾아가 Operand가 같은지 확인하고 다르다면 동기화 시킨다. 이를 저널링 시스템이라고 한다. 클라이언트의 요청에도 문제 없다.


