# RabbitMQ

서버간에 메시지를 전달해주는 오픈소스 메시지 브로커다.
  
A -> B에게 또는 A -> B,C,D,E,F 등 메시지를 보내려고 할 때 RabbitMQ가 이 메세지를 받아서 전달 해주는 것으로 이해하면 된다.

RabbitMQ는 AMQP 프로토콜을 구현한 메시지 브로커인데, 여기서 AMQP란 Advanced Message Queuing Protocol로 client application과 middleware broker와의 메세지를 주고 받기 위한 프로토콜이다.

RabbitMQ에서 메시지를 전송하고 받을 때 구성

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fm90Ws%2Fbtrq0uBxpL5%2FP7DmDoDWFoLiAPtDy5juck%2Fimg.jpg)

MSA 구조의 서버를 사용하다 보면, 서버와 서버끼리 메시지를 주고받아야할 때가 있는데, 이때 RabbitMQ를 사용한다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FmOUWA%2Fbtrq1mJC7Tf%2Fm0kzF3WVoA9AuKPfYHBwU0%2Fimg.png)

서버1이 서버2에게 메세지를 보낼 때 RabbitQM를 통해 메세지를 전송한다.

이때 서버1이 producer가 되는 것이고, 서버2가 consumer가 되는 것이다.

반대로, 서버2가 서버1에게 메세지를 보낼 때 서버2가 producer가 되고 서버1이 consumer가 되는 것이다.

### Exchange

producer에게 받은 message를 queue에게 전달해준다.  
queue에게 전달할 때는 4가지 타입이 있다.

1. Fanout
2. Direct
3. Topic
4. Headers

이 4가지 타입들은 producer에게 전달 받은 메세지를 어떤 queue에 발송 할 지 라우팅을 결정하는 것이다.

### Binding
exchange와 queue와의 관계를 의미한다.

binding이 되어있어야 exchange가 queue에게 메세지를 전달 할 수 있다.

exchange 규칙으로 어떻게 라우팅이 될지 결정 되었으면, binding은 결졍된 메세지를 어떤 queue에게 전달할 지 라우팅할 규칙을 결정하는 것이다.

즉, exchange는 라우팅을 결정하고 binding은 라우팅할 수 있는 규칙을 지정하는 것이다. 그래서 특정조건에 마즌ㄴ 메세지를 특정 큐에 전송하도록 해준다.

### Queue
exchange는 producer로부터 전달 받은 메세지를 binding되는 queue들에게 동일하게 전달한다.

이때, queue는 전달받은 메세지들을 consumer들에게 전달한다. 

이때 공평하게 전달하기 위해 round-robin 스케줄로 메세지를 전달한다. 

queue는 consumer들에게 전달되기 전에 메모리나 디스크에 저장해놓는다.

<br>

## Exchange의 4가지 타입

### Fanout

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FuEZzE%2FbtrqXwUebZI%2F3Lu7DhORrXb2y6lWMi3KxK%2Fimg.png)

exchange와 binding된 모든 queue에게 동일한 메세지를 보낸다. 전체메세지 같은 것이다.

### Direct

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fcz1GiD%2FbtrqY9RYcWf%2Fqk5isXnzc96DUenU5xCdAK%2Fimg.png)

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FFgf4U%2Fbtrq1gpf7lW%2FraHbV09woJxhPLH8OkHdOk%2Fimg.png)

routing key를 활용하여 라우팅한다. 이 말은 exchange에서 보낸 메세지를 routing key를 통해 queue와 직접 binding할 수 있다.

RabbitMQ에서 사용되는 디폴트 exchange는 Direct다. RabbitMQ에서 생성되는 queue가 자동으로 binding되고, 이때는 각 queue의 이름이 routing key로 지정된다.

하나의 queue에 여러개의 routing key를 지정할 수 있고 (예: direct example_2에서 routing key가 error 인 것), 여러 queue에 동일한 routing key를 지정할 수 있다. (예: direct example_2에서 두번째 queue가 routing key가 총 3개로 지정되어있다.)

즉, 메세지의 routing key를 기반으로 queue와 1:N을 할 수 있다.

### Topic

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FcnhJU6%2Fbtrq0t3JfxE%2FjpokZEjjDL8Opwk9jbFH6K%2Fimg.png)

routing key 패턴이 일치하는 queue에 메세지를 전달한다.

direct는 routing key가 완전 일치해야 메세지를 전달 할 수 있는데, topic은 패턴을 정해서 binding 규칙을 정의하기 때문에 direct 방식 보다는 좀 더 유연하게 정의해서 메세지를 전송할 수 있다.

이때, 규칙은  
*: 단어 1개를 대체  
#: 0개 이상의 단어를 대체(없거나 하나 이상의 단어를 의미)

어떤 queue에 routing key를 2개 지정했을 때,  
만약 routing key가 topic 패턴에 모두 일치하게 되었다고 가정해본다.  
그러면 메세지 전달이 2번 되는 것이 아니라 queue에 1번만 전달된다.  
즉, 하나의 queue에 N개의 routing key가 잇고, 패턴도 모두 일치한다고 하더라도 메세지는 한번만 전달된다.

**예시 producer에서 routing key를 지정하여 메세지를 보낼 때**

1. routing key를 quick.orange.rabbit으로 지정하여 전송했을 때: Q1, Q2 모두 메세지를 받는다.
2. routing key를 quick.orange.fox르 지정하여 전송햇을 때: Q1만 메세지를 받는다.
3. routing key를 lazy:brown.fox로 지정하여 전송했을 때: Q2만 메세지를 받는다.
4. routing key를 quick.orange.male.rabbit으로 지정하여 전송했을 때 : 메세지를 받는 Queue는 없다. -> \*.orange.\*이기 때문에 1개의 단어만 대체 할 수 있어서 Q1은 받을 수 없다.
5. routing key를 lazy.orange.male.rabbit으로 지정하여 전송했을 때 : Q2만 메세지를 받는다. -> lazy.#이기 때문에 #은 0개 이상의 단어를 대체할 수 있어서 가능한 것이다.
6. routing key를 orange로 지정하여 전송했을 때: 메세지를 받는 Queue는 없다.

### headers

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbPPFsY%2Fbtrq22YoG1a%2FDvm8p5aAkBYo2Ek5cJucKK%2Fimg.png)

key-value로 정의된 헤더에 의해 메세지를 queue에 전달하는 방법이다.

메세지를 전달하는 producer쪽에서 정의하는 header의 key-value와 메세지를 받는 consumer쪽에서 정의된 argument의 key-value가 일치하면 binding이 된다.

producer에서 정의하는 header는 메세지와 함께 전송되고, consumer쪽에서는 exchange와 queue가 binding되는 시점에 argument를 정의한다.

header에는 x-match라는 key가 있다. 그리고 옵션값으로는 any, all이 있다.


1. x-match: all
header의 key-value와 argument의 key-value가 정확히 일치해야 binding이 된다.
2. x-match: any
producer가 전송하는 header의 key-value 값과 argument의 key-value 값 중 하나라도 일치하는 것이 있으면 binding된다.