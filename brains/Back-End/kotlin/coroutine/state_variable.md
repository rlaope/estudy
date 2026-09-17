# Coroutine 상태 변수

### Job의 상태 변수: isActive, isCancelled, isCompleted

Job의 상태 변수는 세가지가 있는데, 아래 그림처럼 접근이 가능하다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Flo6k2%2FbtrcHfsBY18%2F7GZyCKAPzy2uT7ELFpRIZK%2Fimg.png)

- `isActive`: Job이 실행중인지 여부를 표시
- `isCancelled`: Job cancel 요청이 되었는지 여부를 표시
- `isCompleted`: Job의 실행이 완료되었거나 cancel이 완료되었는지를 표시

#### Job이 CoroutineStart.LAZY 옵션으로 생성되었을 때 변화

Job이 생성됨에서 실행중 상태로 자동으로 넘기지 않기 위해 `CoroutineStart.LAZY` 옵션을 이용해 Job의 상태 변수를 다루어보자.

Job이 `CoroutineStart.LAZY`로 생성되면 Job은 생성됨(New)상태에 머문다. 이 때는 실행중도 취소중도 아니며 isActive, isCancelled, isCompleted 모두 false가 된다.

이제 start() or join()을 통해 Job이 실행 상태로 바뀌면 isActive가 true로 바뀌게 된다.

<br>

#### Job이 Cancel 되었을 때 상태 변화

Job의 cancel이 호출되었을 때 해당 상태에 어떤 일이 일어나는지 알아보자

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FU4uFO%2FbtrcMHaTK6C%2F6avfsllmHHqyRKDqXb1Nxk%2Fimg.png)

먼저 Job은 **취소 중**상태로 바뀌게 된다. isCancelled는 취소가 요청되었는지에 대한 변수로 cancel이 호출되면 항상 true가 된다. 하지만, 취소중인 상태에서 취소가 완료되지 않았으므로 isCompleted는 false이다.

만약 취소가 완료되면 isCompleted()는 true로 바뀐다. invokeOnCompletion은 isCompleted의 상태를 관찰하는 메서드로 isCompleted가 false에서 true로 바뀔 때 호출 된다. 따라서 취소가 완료되었을 때도 호출된다.

<br>

#### Job이 완료되었을 때 상태 변화


Job이 완료되었을 때는 isActvie가 true에서 false로 바뀌고 isCompleted가 false에서 true로 바뀐다.

isCompleted가 false에서 true로 바뀌었으므로 cancel과 마찬가지로 invokeOnCompletion을 통해 설정된 람다식이 호출된다.