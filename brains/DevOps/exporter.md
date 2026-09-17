# Prometheus Exporter - 모니터링 에이전트

### Exporter

프로메테우스는 모니터링 대상의 메트릭을 수집한다.

그렇다면 그 모니터링 대상의 메트릭은 어디서 제공이 되는 것일까?

바로 Exporter에서 제공이 되는 것이다.

Exporter는 모니터링 대상의 메트릭 데이터를 수집하고 프로메테우스가 접속했을 때 알려주는 역할을 한다. 호스트 서브의 cpu, memory 등을 수집하는 `node-exporter` 나 nginx의 데이터를 수집하는 `nginx-exporter` 도 있으며 다양한 데이터의 메트릭 내용을 제공하는 exporter들이 있다.

> - 만약 Exporter를 쓰기 어려운 배치잡등은 push gateway를 사용하면 된다. 
> - 웹 애플리케이션 서버 같은 경우는 메트릭을 클라이언트 라이브러리를 이용해 메트릭을 만든 후 커스텀 Exporter를 사용할 수 있다.

즉 프로메테우스 서버는 Exporter가 열어놓은 HTTP 엔드포인트에 접속해서 메트릭을 수집한다.

그래서 이것을 **Pull 방식**이라고 부른다.

Exporter들은 대표적으로

- node-exporter
- mysql-exporter
- wmi-exporter
- posgre-exporter
- redis-exporter
- kafka-exporter
- jmx-exporter

등등이 있으며 [공식 홈페이지](https://prometheus.io/docs/instrumenting/exporters/)에 더 자세한 exporters 항목이 있다.

