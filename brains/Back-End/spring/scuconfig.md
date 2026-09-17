# (Spring boot + Security) SecurityConfig 설정하기

### Spring Security?
- 엔터프라이지 어플리케이션에 대한 인증, 권한 부요 및 기타 보안 기능을 제공하는 Java / Java EE 프레임 워크입니다.

> 특정 페이지에 대한 접근을 제어하거나 권한을 부여할 때 사용하거나, DB의 저장되는 비밀번호를 암호화 해줄 떄 사용 (그 외에도 많이 있음)

### Security 설정을 위한 Config 파일

```java
@Configuration
public class SecurityConfig extends WebSecurityConfigurerAdapter {

    @Bean // 로그인 시 실행되는 메소드
    public AuthenticationProvider authenticationProvider(){return new LoginAuthenticationProvider();}

    @Bean // 로그인 성공 시 실행되는 메소드 
    public AuthenticationSuccessHandler successHandlerHandler() {
        return new LoginSuccessHandler();
    }

    @Bean // 로그인 실패 시 실행되는 메소드
    public AuthenticationFailureHandler failureHandlerHandler() {
        return new LoginFailureHandler();
    }

    @Bean // 패스워드 암호화 관련 메소드
    public PasswordEncoder passwordEncoder(){
        return new BCryptPasswordEncoder();
    }

    @Override
    public void configure(WebSecurity web) throws Exception {
        web.ignoring().antMatchers("/css/**", "/js/**");
    }

    @Override
    protected void configure(HttpSecurity http) throws Exception {
        http
                .csrf().disable()// 세션을 사용하지 않고 JWT 토큰을 활용하여 진행, csrf토큰검사를 비활성화
                .authorizeRequests() // 인증절차에 대한 설정을 진행
                .antMatchers("/", "/error/*", "/login", "/loginProc").permitAll() // 설정된 url은 인증되지 않더라도 누구든 접근 가능
                .anyRequest().authenticated()// 위 페이지 외 인증이 되어야 접근가능(ROLE에 상관없이)
                .and()
                .formLogin().loginPage("/login")  // 접근이 차단된 페이지 클릭시 이동할 url
                .loginProcessingUrl("/loginProc") // 로그인시 맵핑되는 url
                .usernameParameter("userId")      // view form 태그 내에 로그인 할 id 에 맵핑되는 name ( form 의 name )
                .passwordParameter("userPw")      // view form 태그 내에 로그인 할 password 에 맵핑되는 name ( form 의 name )
                .successHandler(successHandlerHandler()) // 로그인 성공시 실행되는 메소드
                .failureHandler(failureHandlerHandler()) // 로그인 실패시 실행되는 메소드
                .permitAll()
                .and()
                .logout() // 로그아웃 설정
                .logoutUrl("/logout") // 로그아웃시 맵핑되는 url
                .logoutSuccessUrl("/") // 로그아웃 성공시 리다이렉트 주소
                .invalidateHttpSession(true); // 세션 clear
    }
}
```

### @Configuration
- 환경 세팅(스프링의 기본 설정 정보들)을 돕는 어노테이션  
- 클래스의 어노테이션을 @Configuration라고 선언하면 스프링에게 이 클래스는 환경 구성 파일이고 @Bean 어노테이션을 통하여 Bean임을 알려주게 된다.

### @Bean
- 개발자가 작성한 메서드의 return 되는 객체를 Bean으로 만드는 것이다.
- Bean이란 Spring IoC 컨테이너가 관리하는 자바 객체를 빈이라고 한다.


<br>

### WebSecurityConfigureAdapter

- 스프링 시큐리티 설정에 관련된 클래스로 해당 클래스에 있는 메서드를 오버라이딩하여 시큐리티를 설정한다.

로그인에 관련된 메서드:return 되는 클래스들은 커스텀으로 구현해야 한다.
```java
@Bean
public AuthenticationProvider authenticationProvider(){return new LoginAuthenticationProvider();}

@Bean
public AuthenticationSuccessHandler successHandlerHandler() {
    return new LoginSuccessHandler();
}

@Bean
public AuthenticationFailureHandler failureHandlerHandler() {
    return new LoginFailureHandler();
}
```

패스워드 암호화와 관련된 메소드
```java
@Bean
public PasswordEncoder passwordEncoder(){
    return new BCryptPasswordEncoder();
}
```

정적 페이지는 어디서든 접근 가능하도록 설정하는 메서드

```java
@Override
public void configure(WebSecurity web) throws Exception {
    web.ignoring().antMatchers("/css/**", "/js/**");
}
```

접속 권한을 제어하는 메서드

```java
@Override
protected void configure(HttpSecurity http) throws Exception {
    http
            .csrf().disable()// 세션을 사용하지 않고 JWT 토큰을 활용하여 진행, csrf토큰검사를 비활성화
            .authorizeRequests() // 인증절차에 대한 설정을 진행
            .antMatchers("/", "/error/*", "/login", "/loginProc").permitAll() // 설정된 url은 인증되지 않더라도 누구든 접근 가능
            .anyRequest().authenticated()// 위 페이지 외 인증이 되어야 접근가능(ROLE에 상관없이)
            .and()
            .formLogin().loginPage("/login")  // 접근이 차단된 페이지 클릭시 이동할 url
            .loginProcessingUrl("/loginProc") // 로그인시 맵핑되는 url
            .usernameParameter("userId")      // view form 태그 내에 로그인 할 id 에 맵핑되는 name ( form 의 name )
            .passwordParameter("userPw")      // view form 태그 내에 로그인 할 password 에 맵핑되는 name ( form 의 name )
            .successHandler(successHandlerHandler()) // 로그인 성공시 실행되는 메소드
            .failureHandler(failureHandlerHandler()) // 로그인 실패시 실행되는 메소드
            .permitAll()
            .and()
            .logout() // 로그아웃 설정
            .logoutUrl("/logout") // 로그아웃시 맵핑되는 url
            .logoutSuccessUrl("/") // 로그아웃 성공시 리다이렉트 주소
            .invalidateHttpSession(true); // 세션 clear
}
```