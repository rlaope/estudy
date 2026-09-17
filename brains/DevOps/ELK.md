# ELK와 로깅 구성

마이크로서비스 기반의 응용 프로그램을 설계할 때 고려해야하는 6가지 영역중

Telemetry라는 영역이 있다. 혹은 Observation

이런 영역은 로깅, 모니터링, 트레이싱을 담당하며 각 기능은 다음과 같다.

- 로깅 | 여러개로 분리된 서비스에서 생성되는 분산된 로그 파일에 대한 중앙화, 집중화 방안
- 모니터링 | 다중화된 서비스에 대한 모니터링 방안
- 트레이싱 | 클라이언트의 단일 요청에 대해 마이크로서비스간 다중 호출 관계가 있을 때 이에 대한 추적 방안

마이크로서비스에서 가장 많이 사용되는 로깅 방식은 ELK가 대표적이며 Elasticsearch, Logstash Kibana로 구성된다 (EFK는 Fluentd)

<br>

### ELK를 이용한 로깅 구성

ELK의 각 솔루션별 기능을 보면 logstash를 통해 데이터를 집계하고 Elasticsearch를 통해 데이터 항목을 인덱싱하고 저장한 후 kibana를 통해 인덱싱된 데이터를 보여주는 시각화(Visualization) 구조를 가진다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fl7NK2%2FbtrpU0vLDzL%2FRSijGTDKRWZIvIHoapgEA1%2Fimg.png)

위 그림을 보면 beats라는 추가 솔루션이 있는데, beats는 경량의 데이터 수집용 솔루션으로 데이터 수집을 위해 다양한 제품들로 구성되어있다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FODhBl%2Fbtrp2Cfp7dy%2Fo9qVBKM7czufeyNA9ri6D0%2Fimg.png)


아래는 Beats + ELK로 구성할 수 있는 다양한 아키텍처들이다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fdyvl4u%2FbtrpVvWEzin%2FWFEs6qNZKzE6JUsU7lRyH1%2Fimg.png)