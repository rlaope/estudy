# 스프링 빈(Bean) 이란?
**스프링 컨테이너가 관리하는 자바 객체를 빈이라 한다.**

- 제어의 역전이란, 간단히 말해서 객체의 생성 및 제어권을 사용자가 아닌 스프링에게 맡기는 것이다.
- 지금까지는 new 연산을 통해 객체를 생성하고 메소드를 호출했다. 하지만 IoC가 적용된 경우에는 이러한 객체의 생성과 사용자의 제어권을 스프링에게 넘긴다.
- 사용자는 직접 new를 이용해 생성한 객체를 사용하지 않고, 스프링에 의하여 관리당하는 자바 객체를 사용한다. 이 객체를 `빈(Bean)`이라 한다.

<br>

## 스프링 빈을 스프링 컨테이너에 등록하는 방법

### 1 컴포넌트 스캔과 자동 의존관계 설정

- @Component 어노테이션이 있으면 스프링 빈으로 자동 등록된다.
- @Component를 포함하는 @Controller, @Service, @Repository 어노테이션도 스프링 빈으로 자동 등록된다.

![spring container](./image/bean.png)

회원 컨트롤러를 등록하고, 회원 컨트롤러가 회원 서비스와 회원 리퍼지토리를 사용할 수 있게 의존관계를 설정해보자.

```java
package hello.hellospring.controller;

import hello.hellospring.service.MemberService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Controller;

@Controller
public class MemberController {

    private final MemberService memberService;

	@Autowired
    public MemberController(MemberService memberService) {
        this.memberService = memberService;
    }
}
```

- **@Autowired**
  - 생성자에 @Autowired가 있으면 스프링이 연관된 객체를 스프링 컨테이너에 찾아서 넣어준다. 
  - 이렇게 객체 의존관계를 외부에서 넣어주는 것은 `의존성 주입(DI)`라 한다. 
  - 위 관계 그림에서 -> 의존관계를 표현한 화살표 역할을 해준다.(생성자가 1개만 있으면 생략 가능)


하지만 이 상태로 서버를 작동시키면 오류가 발생할 것이다.
![springcontainer2](./image/bean2.png)
  
바로 memberService가 스프링 컨테이너에 스프링 빈으로 등록되어 있지 않기 때문이다.  
memberService도 스프링 빈으로 등록해주어야 한다.

```java
@Service
public class MemberService {

	private final MemberRepository memberRepository;

	@Autowired
	public MemberService(MemberRepository memberRepository) {
		this.memberRepository = memberRepository;
	}
}
```

memberRepository도 스프링 빈으로 등록해야 한다.

```java
@Repository
public class MemoryMemberRepository implements MemberRepository {}
```
![spring container](./image/bean.png)

memberService와 memberRepository가 스프링 컨테이너에 스프링 빈으로 등록되었다.  
  
참고로, 스프링은 스프링 컨테이너에 스프링 빈을 등록할 때, 기본으로 싱글톤으로 등록한다.(유일하게 하나만 등록해서 공유한다.) 따라서 같은 스프링 빈이면 모두 같은 인스턴스다. 설정으로 싱글톤이 아니게 설정할 수 있지만, 특별한 경우를 제외하면 대부분 싱글톤을 사용한다.

## 자바 코드로 직접 스프링 빈 등록하기
- @Configuration과 @Bean 어노테이션을 이용해 스프링 빈을 등록한다.
- @Configuration을 이용하면 스프링 프로젝터에서 Configuration 역할을 하는 Class를 지정할 수 있다.

src/main/java 하위 폴더에 SpringConfig파일을 생성하고 코드를 쓴다.

```java
package hello.hellospring.service;

import hello.hellospring.repository.MemberRepository;
import hello.hellospring.repository.MemoryMemberRepository;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;

@Configuration
public class SpringConfig {

    @Bean
    public  MemberService memberService() {
        return new MemberService(memberRepository());
    }

    @Bean
    public MemberRepository memberRepository() {
        return new MemoryMemberRepository();
    }
}
```

### 참고
- XML로 설정하는 방식도 있지만 최근에는 잘 사용하지 않는다
- DI에는 필드 주입, setter 주입, 생성자 주입 이렇게 3가지 방법이있다. 의존관계가 실행중에 동적으로 변하는 경우는 거의 없으므로 생성자 주입을 권장한다.
- 실무에서는 주로 정형화된 컨트롤러, 서비스, 리퍼지토리같은 코드는 컴포넌트 스캔을 사용한다. 그리고 정형화 되지 않거나 상황에 따라 구현 클래스를 변경해야 하면 설정을 통해 스프링 빈으로 등록한다.
- @Autowired 를 통한 DI는 helloConroller , memberService 등과 같이 스프링이 관리하는 객체에서만 동작한다. 스프링 빈으로 등록하지 않고 내가 직접 생성한 객체에서는 동작하지 않는다.