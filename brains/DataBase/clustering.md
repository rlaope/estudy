# Clustering (Active Clustering, Standby Clustering)

## Clustering
데이터베이스 분산 기법 중 하나로 DB 서버를 여러 개를 두어 서버 한 대가 죽었을 때 대비할 수 있는 기법

## Active Clustring


![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fk.kakaocdn.net%2Fdn%2FdeVt2f%2FbtqEOPtyyNu%2FBm4k5ilo6dReFoyVVP23XK%2Fimg.png)

DB 서버를 여러 개 구성하는 데, 각 서버를 Active 상태로 둠  

### 장점
- 서버 하나가 죽더라도 다른 서버가 역할을 바로 수행하므로 서비스 중단이 없음
- CPU와 메모리 이용률을 올릴 수 있음

### 단점
- 저장소 하나를 공유하게 되면 병목현상이 발생할 수 있다.
- 서버를 여러대 한꺼번에 운영하므로 비용이 더 발생한다.


## Standby Clustering

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fk.kakaocdn.net%2Fdn%2Fded8LQ%2FbtqEO02Erjo%2Fer820balKxEB3dkaBGaGn0%2Fimg.png)

서버를 하나만 운영하고 나머지 서버는 Standby상태로 둔다.  
운영하고 있는 서버가 다운되었을 시에 Standby 상태의 서버를 Active상태로 전환한다.

### 장점
- Active-Active 클러스터링에 비해 적은 비용이 든다.

### 단점
- 서버가 다운되었을 때 Standby 상태의 서버를 Active 상태로 전환하는 시간이 필요하다.