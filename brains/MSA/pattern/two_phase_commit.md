# Two Phase Commit

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2F8zMJw%2Fbtrx0uOU61k%2FAKjHsJeXfH1YAxzMdi6XS0%2Fimg.png)

분산 데이터베이스 환경에서 쓰는 방법으로 주로 RDBMS에서 기능을 제공한다. Two-Phase Commit은 말 그대로 2 단계에 거쳐서 데이터를 영속화 하는 작업이다.

위 그림과 같이 여러 DB가 분산되었을 때, 트랜잭션을 조율하는 조정자가 존재한다.

조정자의 역할은 트랜잭션 요청이 들어왔을 때 두 단계를 거쳐 트랜잭션 진행을 담당하는 것이다.

첫 번째 단계는 Prepare이며 이는 쉽게 말해 연관 DB에게 데이터를 저장할 수 있는 상태인지 묻는 과정이다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FplqLW%2FbtrxXoWua3o%2FjWKzMmFsvJgXnlPvk2EPwK%2Fimg.png)


메시지를 받은 데이터베이스에서는 Commit 작업을 위한 준비를 진행한다.

이후 데이터를 영속화할 수 있는 준비가 완료되면 조정자에게 준비가 되었음을 알리고, 반대로 불가능 하다면 불가하다는 메세지를 보낸다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FcPqGWz%2Fbtrx0uBrKXJ%2FEx1ZHPJJqkaIK346CHkZ3k%2Fimg.png)

조정자는 첫 번째 단계에서 전달한 메시지에 대한 응답을 기다린다. 모든 메시지는 수신이 완료되면 두 번째 단계인 commit을 진행한다.

Commit 단계에서는 조정자가 연관된 DB에게 데이터를 저장하라는 메시지를 송신하며, 수신받은 DB에서는 각자 DB에 데이터를 영속화 한다.

모든 DB에서 트랜잭션 처리가 완료되면 전체 트랜잭션을 종료한다.

만약 두 단계를 거치는 과정에서 연관된 DB 중 하나의 DB라도 Commit을 할 수 없는 상황이라면, 모든 DB에게 Rollback을 요구한다.

트랜잭션을 종료하는 동시에 모든 DB 데이터가 영속화된다. 따라서 **트랜잭션의 범위는 데이터를 처리하는 DB 전체이**다.

<br>

## MSA 환경에서 Two-Phase Commit의 문제점
DBMS 간 분산 트랜잭션을 지원해야 적용이 가능하다. 하지만 NoSQL 제품군에는 이를 지원하지 않고, 함께 사용되는 DBMS가 동일 제품군이여야한다.

따라서 DBMS Polyglot 구성이 어렵다.

> Polyglot: 다중 언어를 지원하는 것을 의미한다.  
> DBMS Polyglot: 다양한 종류의 데이터베이스(MySQL, Oracle, PostgreSQL...)를 지원하는 관리 시스템


또한 Two-Phase Commit은 보통 하나의 API 엔드포인트를 통해 서비스 요청이 들어오고 내부적으로 DB가 분산되어있을 때 사용된다.  
하지만 MSA 환경에서는 각기 다른 APP에서 API간으로 통신을 통해 서비스 요청이 이루어지기 때문에 구현이 쉽지 않다.