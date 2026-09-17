# 페이지 교체 알고리즘 FIFO, LRU, LFU, MFU, NUR

## 페이지 교체 알고리즘

운영체제는 주기억장치보다 더 큰 용량의 프로그램을 실행하기 위해 프로그램의 일부만 주기억장치에 적재하여 사용한다.

이를 가상메모리 기법이라고 한다.

페이징 기법으로 메모리를 관리하는 운영체제에서 필요한 페이지가 주기억장치에 적재되지 않았을 시(페이지 부재) 어떤 페이지 프레임을 선택하고 교체할 것인지 결정하는 방법을 **페이지 교체 알고리즘**이라고 한다.

### 종류
- OPT - Optinal: 앞으로 가장 오랫동안 사용되지 않을 페이지 교체
- FIFO: First In First Out: 가장 먼저 들어온 페이지를 교체
- LRU - Least Recently Used: 가장 오랫동안 사용되지 않은 페이지 교체
- LFU - Least Frequently Used: 참조 횟수가 가장 작은 페이지 교체
- MFU - Most Frequently Used: 참조 횟수가 가장 많은 페이지 교체
- NUR - Not Used Recently: 최근에 사용하지 않은 페이지 교체

<br>

## OPT(Optimal) 가장 앞으로 오랫동안 사용되지 않을 페이지 교체

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FS8lUX%2Fbtq9JNyN39f%2FoYoWX91sjF34LkuFfuJxrk%2Fimg.png)

- 가상 이상적이다.
- 프로세스가 앞으로 사용할 페이지를 미리 알아야 함 -> 불가능
- 비교 연구 목적을 위해 사용된다.

## FIFO First In First Out 가장 먼저 들어온 페이지를 교체

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FU5nSm%2Fbtq9PUpYAl8%2FWtyueVDWhp6E2nLGbNHYWK%2Fimg.png)

- 메모리에 가장 먼저 올라온 페이지를 먼저 내보냄.
- 간단하고, 초기화 코드에 대해 적절한 방법
- 들어온 시간을 저장하거나 올라온 순서를 큐에 저장.
- 직관적으로 생각할 때 프레임 수가 많아질수록 페이지 결함의 횟수는 감소함
- Belady's Anomaly(FIFO anomaly)
실제로 그렇지 않게 되는 현상이 나타날 수 있다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbBk7Bp%2Fbtq9Vka7iP3%2FOyRlrUsTMJ4owdIJDC1KL0%2Fimg.jpg)
![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fukhyy%2Fbtq9NROjAZc%2Fy1yPHPRuZenlX3VTUBIq01%2Fimg.jpg)

## LRU(Least Recently Used) 가장 오랫동안 사용하지 않은 페이지를 교체


![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fb1csvE%2Fbtq9IZlYnx0%2FwbSZzZfsBkbmQ80nnf9LdK%2Fimg.png)

가정: 가장 오랫동안 사용하지 않았떤 데이터라면 앞으로도 사용할 확률이 적을 것이다.

시간 지역성(temporal locality) 성질을 고려함(최근에 참조된 페이지가 가까운 미래에 다시 참조될 가능성이 높은 성질)

사용된 시간을 알수있는 부분을 저장하여 가장 오랫동안 참조되지 않은 데이터를 제거(페이지마다 카운터 필요)

큐로 구현 가능, 사용한 데이터를 큐에서 제거하여 맨 위로 다시 올리고, 프레임이 모자랄 경우 맨 아래에 있는 데이터를 삭제

### 단점
프로세스가 주기억장치에 접근할 때마다 참조된 페이지 시간을 기록해야 하므로 막대한 오버헤드가 발생

카운터나 큐, 스택과 같은 별도의 하드웨어 필요

> 카운터: 걱 페이지별로 존재하는 논리적인 시계로, 해당 페이지가 사용될 때마다 0으로 클리어 시킨 후 시간을 증가시켜 시간이 가장 오래된 페이지를 교체

<br>

## LFU(Least Frequently Used) 참조 횟수가 가장 낮은 페이지를 교체

페이지의 참조 횟수로 교체할 페이지 결정

LRU는 직전 참조된 시점만을 반영하지만, LFU는 참조횟수를 통해 장기적 시간규모에서 참조성향을 고려할 수 있음

### 단점
가장 최근에 불러온 페이지가 교체될 수 있음, 구현 더 복잡, 막대한 오버헤드

<br>

## MFU 참조 횟수가 가장 많은 페이지 교체
가정: 가장 많이 사용된 페에지가 앞으로는 사용되지 않을 것이다.

<br>

## NUR Not Used Recently, 클럭 알고리즘

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FqfdHU%2Fbtq9PTLnM9o%2F7Xg5UGO0UYs3qeXjJwTiW1%2Fimg.png)

최근에 사용하지 않은 페이지 교체 LRU를 근사한 알고리즘

교체되는 페이지의 참조 시점이 가장 오래되었다는 것을 보장하지는 못함

적은 오버헤드로 적절한 성능

동일 그룹 내에서 선택 무작위

각 페이지마다 두개의 비트 참조 비트(Reference Bit)와 변형 비트(Modified Bit, Birty Bit)가 사용됨

- 참조 비트: 페이지가 참조되지 않았을 때 0, 호출되었을 때 1 (모든 참조비트를 주기적으로 0으로 변경)
- 변형 비트: 페이지 내용이 변경되지 않았을 때는 0, 변경되었을 때 1

우선순위: 참조비트 > 변형비트