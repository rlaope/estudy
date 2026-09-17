# SNS SImple Notification Service

## Definition
It is a web service that facilitates and manages message delivery to subscribed endpoints or clients.
It can be seen as a Message Broker that implements MOM.
The party that produces events is called a Publisher, and the party that subscribes to events is called a Subscriber.

![](https://camo.githubusercontent.com/a944b28e4c72aeebd8821d043b6cf783bc3a479ce3aa7fd5408b81fd80dbefdf/68747470733a2f2f74312e6461756d63646e2e6e65742f6366696c652f746973746f72792f393936424534343135433233363245443230)

AWS SNS leaves logs on the message delivery status for messages sent to subscribers like SQS/Lambda.
