# AWS ELK (Elastic Search Logstash Kibana)

## ELK
ELK는 로그 데이터 분석 도구이다.

### Elastic Search
Logstash를 통해 수신된 데이터를 저장소에 저장하는 역할  
**분산 검색 엔진이다.**

### Logstash
수집할 로그를 선정해, 지정된 서버에 인덱싱하여 전송하는 역할

### kibana
데이터를 시각적으로 탐색, 실시간으로 분석하는 역할

### ELK Stack
위의 ELK에 `Beats`가 추가된 것

### Beats
에이전트로 설치하여 다양한 유형의 데이터를 Elastic Search 또는 Logstash에 전송하는 데이터 발송자
