# OSIV(Open Session In View)

## OSIV
JPA의 영속성 컨텍스트와 하이버네이트의 session을 뷰까지 열어두는 기능이다.

### 사용 이유
기존에 @Transactional이 붙어있는 서비스 내부에서 유지가 되던 영속성 컨텍스트의 영향 범위를 뷰 렌더링까지 유지될 수 있도록 하는 것.
  
@Transactional이 붙은 서비스를 호출하면 스프링의 트랜잭션 AOP가 동작한다.  
  
트랜잭션 AOP는 해당 메서드를 호출하기 직전에 트랜잭션을 시작하고, 메서드가 정상 종료되면 트랜잭션을 커밋하면서 종료하게 된다. 만약 예외가 발생하면 롤백을 진행한다.


## 동작 원리
![](https://github.com/leeseojune53/yatudy/raw/main/images/persistent_context.png?raw=true)

1. Request가 오면 Servlet Filter/ Interceptor에서 Persistence Context를 생성한다. 단, 트랜잭션은 시작하지 않는다.
2. 서비스 계층에서 @Transactional로 트랜잭션을 시작할 때 생성되어있는 Persistence Context를 찾아서 트랜잭션을 시작한다.
3. 메서드가 정상 종료되면 트랜잭션을 커밋하면서 종료하게된다. 만약 예외가 발생하면 롤백을 진행한다.
4. 컨트롤러 / 뷰까지 Persistence Context가 유지되므로 조회한 엔티티는 영속 상태를 유지한다.
5. Servlet Filter / Interceptor로 요청이 들어오면 Persistence Context를 종료한다. 이때 Flush를 호출하지 않고 바로 종료한다.

### 주의사항
`spring.jpa.open-in-view`를 기본값 true로 어플리케이션을 구동하면, WARN 로그가 남는다.
  
WARN 로그의 이유는 OSIV가 최초 데이터베이스 커넥션을 시작 지점부터 API response까지 영속성 컨텍스트와 데이터베이스 커넥션을 유지한다. 덕분에 Controller에서 지연로딩이 가능한 것이다.
  
하지만 OSIV는 장단점이 동시에 존재한다. 위의 기능안 장점이지만 이로인해 오랜시간 데이터베이스 커넥션 리소스를 사용하므로 이후에 DB 커넥션이 부족할 수 있다. 이것은 장애로 이어질 가능성이 크며 치명적인 단점이기도 하다.
  
OSIV를 종료하려면 `spring.jpa.open-in-view`를 false로 설정하면 된다.
  
OSIV를 제외하고 복잡성을 관리하는 다른 방법으로는 커멘드와 쿼리 분리(CQRS)가 있다.
  
간단하게 읽기 전용 트랜잭션과 핵심 비즈니스 로직을 따로 놔두는 것이다.