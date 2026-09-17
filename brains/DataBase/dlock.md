# Deadlock

두개이상의 트랜잭션들이 동시에 진행될 대 서로가 서로에 대한 락을 소유한 상태로 대기 상태로 빠져 더이상 진행하지 못하는 상황이 **Deadlock** 상황이다.
- 데드락은 트랜잭션을 지원하는 디비에서 자주 발생하는 문제
- 멀티 스레드 애플리케이션에서 발생하는 데드락은 해당 어플리케이션을 완전히 멈추게 해버리기 때문에 위험
- 일반적인 dbms는 deadlock detection이 있어 데드락이 발생하면 자동으로 해소시켜주긴 한다.(실제 dl이 아니여도 timeout 설정시간을 초과하면 dl로 처리되기도 한다.)
  - 이 과정에서 작업중이던 트랜잭션이 일부가 취소되는 경우가 발생할수도 있는데 애플리케이션 레벨에서 해당 트랜잭션을 재실행해 복구할 수 있도록 구성해야한다.

### Lock 종류
- **shared lock**: slock으로 읽기 잠금으로 특정 row를 읽을때 사욯암. a에서 slock을 걸었다면 b tx에서는 읽기만 가능하고 쓰기는 불가능 즉 다른트랜잭션에서 slock은 가능하나 xlock은 못검
- **exclusive lock**: xlock 쓰기 잠금으로 특정 row를 write할때 사용되며 a 트랜잭션에서 특정 row에 xlock을 걸었다면 b 트랜잭션에서 해당 row를 읽기, 쓰기 모두 불가능함. 해당 트랜잭션에서 락이 종료되길 기다려야함. 즉 다른 트랜잭션이 slock, xlock아 ㅇ
- **Intention Lock**: 테이블 안의 특정 row에 대해서 나중에 어떤 row level lock을 걸 것이라는 의도를 알려주기 위해 미리 table level에 걸어두는 락임.
  - Intention Shared Lock(IS), Intention Exclusive Lock(IX)가 존재함
    - select .. lock in share mode이 실행되면 먼저 is가 걸리고 그 후 row level에서 slock이 걸림
    - select .. for update, insert, delete, update면 먼저 ix가 걸리고 그 후 row level xlock 걸림

인텐션을 쓰는 이유는 특정 트랜잭션a에서 이미 테이블에 락이 걸려있는데 다른트랜잭션 b에서 해당 테이블의 특정 row에 락을 거는 것을 원천적으로 방지하기 위함(데드락 피하려고)
- row level의 write가 일어나고 있을때 테이블 스키마가 변경되어서는 안됨. write query의 경우 이미 ix를 획득했기 때문에 테이블 스키마 변경도 막을 수 있음.
- update할때 보통은 update row에 걸리는데 where 인덱스를 적절히 태우지 않거나 없으면 테이블 lock이 걸림.





### 데드락을 줄이는 방법

- 트랜잭션을 간결하게
- 인덱스를 잘 구성 (더 적은 레코드를 스캔할 수록 락 범위가 좁아지기 때문)
- Locking read(select for update, select lock in share mode)를 사용시에 Read committed와 같이 더 낮은 레벨의 트랜잭션을 사용할 수 있다면 적극 활용하자. (일관성은 좀 떨어질수잇음)
- 트랜잭션 안에서 여러 데이터를 수정할 때 발생하는 lock은 순서를 항상 순차적으로 만든다.
  - ex. a, b, c table 수정시 각 트랜잭션에서 a b c 순서로 수정하면 윟머성이 적음
  - 예를 들어 애플리케이션에서 a c b, c b a등과 같이 각각 트랜잭션이 동시에 실행되면서 다른 데이터를 수정하게 되면 데드락 확률이 높아짐.
    - 테이블 락으로 예시를 들었을때 첫 트랜잭션에서 a, 두번째 c락을 잡고 첫번째 트랜잭션에서 c를 수정하려 할때 두번째 트랜잭션이 락을 이미 잡았고 b 수정후 a 락을 잡으려하지만 첫번째가 잡고있기 때문에 대기에 빠져 데드락이 걸리는 상황
- 하나의 구문에 여러 row가 포함되는 batch insert on duplicate key update에 주의해라. 하나의 batch 구문은 트랜잭션이 걸린 여러개의 구문처럼 동작하기 때문에 각각의 배치 query에 포함된 데이터의 pk가 겹치게 되면 데드락이 발생할 확률이 있다.
- 트랜잭션을 완전히 serialize하면 되는데
  - 1줄의 데이터만 갖고있는 세마포어 테이블을 생성해 각각의 트랜잭션들이 다른 테이블에 접근하기 전에 세마포어 용 테이블을 업데이트 하도록 하고 세마포어 테이블에 늦게 접근한 트랜잭션은 대기상태에 빠져 각 트랜잭션들의 완전한 순차 실행이 보장된다.

<br>

### Deadlock Stat

**SHOW ENGINE INNODB STATUS;** 명령어를 통해 데드락이 걸려있는지 혹은 어떤 락이 걸려있는지 알 수 있다.


------- TRX HAS BEEN WAITING 5 SEC FOR THIS LOCK TO BE GRANTED:
RECORD LOCKS space id 770 page no 3 n bits 80 index PRIMARY of table `example_db`.`test` trx id 774317 lock_mode X locks rec but not gap waiting

위와 같이 example db에 exclusive lock(lock mode x) 때문에 대기중인 트랜잭션이 있다는 메시지를 확인 가능하다.


### Example

Batch Insert on duplicate로 쿼리1: pk:1에 a를 인서트 pk:2에 b를 인서트, 쿼리2: pk:2에 a, pk:1에 b일때 쿼리1은 row upsert pk=1 lock획득, 쿼리2는 pk=2에 lock 획득해 두 쿼리가 운나쁘게도 다른 커넥션에 들어와 묶어놓은 것처럼 동작해 위와같은 순서로 동작하면 데드락이 발생한다.

insert구문도 마찬가지인데 a b c순서로 같은 key에 대한 insert를 실행할때 a b c가 시작되고 a가 먼저 락을 잡다가 롤백하는 순간 b c가 경쟁하게 되는데 insert의 경우 exclusive lock을 획득해야하지만 duplicated key error가 발생하면 해당 인덱스 레코드에 대한 shared lock을 먼저 획득하는 특성이 있음 그래서 b c가 동일 인덱스 레코드에 대해 shared lock을 먼저 획득한 후 exclusive lock을 획득하려 하기 때문에 데드락이 발생함. (b xlock 획득하려해도 c의 slock때문에 안됨. c->b도 vice versa) 여기서 a를 commit하면 b c는 바로 duplicated key error가 발생하지만 여전히 트랜잭션은 열려있음. 결국 이 상황에서 선행 트랜잭션이 롤백하면 나머지 트랜잭션은 하나만 성공하고 나머지는 데드락으로 강제 롤백됨. 이 케이스는 얼핏봐서는 발생 안할것 같지만 기억해두자.

바로 위에서 insert는 매우 특이한 상황이지만 delete는 특이하지 않게 잘 동작하는 것을 확인가능함.
- a b c 순서로 delete 같은 key에 대한 경쟁일때
  - a의 xlock 획득
  - b xlock 대기, c xlock 대기
  - a 커밋시 lock의 대상인 row가 사라져 b c모두 대기상태 종료 affected row=0을 리턴. 트랜잭션은 계속 열려있음
  - a 롤백시 b가 xlock 획득, c는 계속 대기

위 상황 두개를 비교해보면 insert의 경우에는 duplicated key error 발생시 shared lock을 획득 시도하기 때문에 하나 이상의 트랜잭션이 동시에 동일한 row에 대한 shared lock을 획득해 (shared lock은 동시에 획득 가능하기 때문) 트랜잭션 a가 종료되지만, 다시 트랜잭션 b가 xlock을 획득 시도하면 c의 shared lock 때문에 실패함. 반대도 vice versa

but delete의 경우에는 xlock 획득 시도하기 때문에 하나의 트랜잭션만 락 획득이 가능해서 나머지는 대기하고 데드락이 안생김.

### 조치 방법

- 쿼리 개선: 위 상황 생기지 않게 하기
- isolation level 조정: serialize하게 할 수 있도록
- 스토리지 엔진에서 dl 감지 옵션 조정
  - deadlock_search_depth_long
  - deadlock_search_depth_short
  - deadlock_timeout_long
  - deadlock_timeout_short
