# SNS SImple Notification Service

## 정의
구독중인 엔드포인트 또는 클라이언트에 메시지 전달을 조성 및 관리하는 웹 서비스이다.  
MOM을 구현한 Message Broker라고 보면 될 것 같다.  
이벤트를 생상하는 쪽을 게시자 Publisher라고 하고, 이벤트를 구독하는 쪽을 구독자(Subscriber)라고 한다.  

![](https://camo.githubusercontent.com/a944b28e4c72aeebd8821d043b6cf783bc3a479ce3aa7fd5408b81fd80dbefdf/68747470733a2f2f74312e6461756d63646e2e6e65742f6366696c652f746973746f72792f393936424534343135433233363245443230)

AWS SNS는 구독자 SQS/Lambda등에게 전송된 메시지 전달 상태에 log를 남겨준다.