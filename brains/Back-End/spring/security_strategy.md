# SecurityContextHolderStrategy (ThreadLocal, InheritableThreadLocal, Global)

## TheradLocal
WebMVC 기반으로 프로젝트를 만든다고 가정할 때, 요청 한 개당 스레드 한 개가 생성된다.
  
이때 ThreadLocal을 사용하면 스레드마다 고유한 공간을 만들 수 잇고, 그곳에 SecurityContext를 저장할 수 있다.

- ThreadLocal로 SecurityContext를 관리하면, SecurityContext는 요청마다 독립적으로 관리된다.
- `IngeritableThreadSecurityLocalSecurityContextHolderStrategy`를 사용한다.

## InheritableThreadLocal

`InheritableThreadSecurityLocalContextHolderStrategy`를 사용하며 이를 사용해, 자식 스레드까지도 SecurityContext를 공유한다.

## Global

`GlobalSecurityContextHolderStrategy`를 사용하며, 글로벌로 설정되어 application 전체에서 SecurityContext를 공유한다.
