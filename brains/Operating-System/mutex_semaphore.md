# 뮤텍스(Mutex)와 세마포어(Semaphore)

동시성 프로그래밍의 가장 큰 숙제는 `공유 자원의 관리`일 것이다.
  
공유자원을 안전하게 관리하기 위해 상호배제(Mutual exclusion)를 달성하는 기법이 필요하다.
  
뮤텍스와 세마포어는 이를 위해 고안된 기법으로 서로 다른 방식으로 상호배제를 달성한다.

## 뮤텍스 Mutex
한 쓰레드, 프로세스에 의해 소유될 수 있는 `Key`를 기반으로 한 상호배제기법
  
공유된 자원의 데이터 혹은 임계영역 등에 하나의 프로세스, 스레드가 접근하는 것을 말아줌(동기화 대상이 하나)
  
임계구역을 가진 스레드들의 실행시간이 서로 겹치지 않고 각각 단독으로 실행되도록 하는 기술

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fcvk7nh%2FbtrjvSw2BoX%2FZfh0o0VsZrMmAOi6PxLvg0%2Fimg.png)

### 예시
뮤텍스는 예를들어 화장실이 하나 뿐인 식당과 비슷하다.  
  
화장실을 가기 위해서는 카운터에서 열쇠를 받아 가야 한다.  
  
당신이 화장실을 가려고 하는데 카운터에 키가 있으면 화장실에 사람이 없다는 뜻이고 당신은 그 열쇠를 이용해 화장실에 들어갈 수 있다.

![](https://cdn-images-1.medium.com/max/1600/1*6JKj81oYsxQlDhjAlMXQjg.png)

당신이 화장실에서 행복한 시간을 보내고 있는데 다른 테이블에 있는 어떤 남자가 화장실에 가고 싶어졌다.  
  
이 남자는 아무리 용무가 급하더라도 열쇠가 없기 때문에 화장실에 들어갈 수 없고, 결국 남자는 당신이 용무를 마치고 나올때까지 카운터에서 기다려야한다.

![](https://cdn-images-1.medium.com/max/1600/1*kbTbS09Yvah9ja7nozSAMA.png)

곧이어 옆 테이블에 있는 남자도 화장실에 가고싶어 졌는데 이 남자 또한 화장실에 들어가기 위해서는 카운터에서 대기해야한다.

![](https://cdn-images-1.medium.com/max/1600/1*CgUE8ByDUKnVkkrEbMPwCQ.png)

이제 당신이 화장실에서 나와 카운터키를 돌려놓았다. 이제 기다리던 사람들 중 맨 앞에 있던 사람은 키를 받을 수 잇고 이를 이용해 화장실에 갈 수 있다.

![](https://cdn-images-1.medium.com/max/1600/1*dIIfI3ezb3Gt2YH1uWhauQ.png)

이것이 뮤텍스가 동작하는 방식이다. 화장실을 이용하는 사람은 **프로세스 혹은 쓰레드**이며 화장실은 **공유자원**, 화장실 키는 공유자원에 접근하기 위해 필요한 어떠한 오브젝트다.

![](https://cdn-images-1.medium.com/max/1600/1*CdLr52i_BZjnEf3uWZyRVQ.png)

즉 뮤텍스는 key 해당하는 어떤 오브젝트가 있어야 이 오브젝트를 소유한 (쓰레드, 프로세스)만이 공유자원에 접근할 수 있다.

## 세마포어 Semaphore
Signaling mechanism. 현재 공유자원에 접근할 수 있는 쓰레드, 프로세스의 수를 나타내는 값을 두어 상호배제를 달성하는 기법
  
공유된 자원의 데이터 혹은 임계영역등에 여러 프로세스 혹은 스레드가 접근하는 것을 막아줌(동기화 대상이 하나 이상)

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FcYZOiu%2FbtrjvrzaimS%2FQtooHYav5Sj1JpT9yTtb1K%2Fimg.png)

### 예시
세마포어는 손님이 화장실을 좀 더 쉽게 이용할 수 있는 레스토랑이다. 
  
세마포어를 이용하는 레스토랑의 화장실에는 여러개의 칸이 있다. 그리고 화장실 입구에는 현재 화장실의 빈 칸의 개수를 보여주는 전광판도 있다.

![](https://cdn-images-1.medium.com/max/1600/1*ZJXrQu8rFhSQxW6LVAI-GA.png)

만약 당신이 화장실에 가고 싶다면 입구에서 빈 칸의 개수를 확인하고 빈 칸이 1개 이상이라면 빈칸의 개수를 하나 뺀 다음에 화장실에 입장해야한다.

![](https://cdn-images-1.medium.com/max/1600/1*lFNABipdkdtxFvW9UZmCaw.png)


모든 칸에 사람이 들어갔을 경우 빈 칸의 개수는 0이 되며 이때 화장실에 들어가고자 하는 사람이 있다면 그 사람은 빈 칸의 개수가 1개로 바뀔 때까지 기다려야한다.

![](https://cdn-images-1.medium.com/max/1600/1*wP9yqG6QBuS7A8i_kKTyRA.png)

사람들은 나오면서 빈 칸의 개수를 1씩 더한다. 그리고 기다리던 사람은 이 숫자에서 다시 1을 뺀 다음 화장실로 돌진한다.

![](https://cdn-images-1.medium.com/max/1600/1*36aMopAPHO3e80YYADmY6w.png)

이처럼 세마포어는 공통으로 관리하는 하나의 값을 이용해 상호배제를 달성한다.
  
세마포어도 아까와 똑같이 화장실이 공유자원이며 쓰레드, 프로세스다. 그리고 화장실 빈칸의 개수는 현재 공유자원에 접근할 수 있는 쓰레드, 프로세스의 개수를 나타낸다.