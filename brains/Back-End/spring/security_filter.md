# Spring Security Filter

## Filter

Filter는 `요청 전과 응답 후 어떤 작업`을 하도록 하는 처리하는 것

![](https://user-images.githubusercontent.com/80656733/155106878-d64ba544-8ddf-489d-adc6-816bcde7541b.png)

Spring Security의 동작은 사실상 Filter로 동작한다고 해도 무방하다.

- 다양한 필터들이 각자 다른 기능을 한다.
  - 필터가 어떤 기능을 하는지 알기 위해서는 Filter 클래스의 doFilter method를 살펴보면 된다.
- 이런 필터들은 제외할 수도, 추가할 수도 있다.
- 필터에 동작하는 순서를 정해줄 수 있다.

필터가 여러개 일 때, 다음과 같이 동작한다.

![](https://user-images.githubusercontent.com/80656733/155108797-6fa23fe8-cc66-4514-a3a4-e8dc4cf33e19.png)


스프링 시큐리티 필터는 여러개가 존재하며 하나씩 알아보겠다.

## SecurityContextPersistenceFilter

보통 두 번째로 실행되는 필터다

> 첫 번째 필터는 Async 요청에 대해서도 SecurityContext를 처리할 수 있도록 도와주는 `WebAsyncManagerIntegrationFilter다.

이 필터는 `SecurityContext`를 찾아와서 `SecurityContextHolder`에 넣어주는 역할을 한다.  
  
만약 SecurityContext를 찾았는데 없다면, 그냥 새로 하나 만들어준다.

## BasicAuthenticationFilter

로그인을 하지 않아도, id, password를 Base64로 인코딩해서 모든 요청에 포함해서 보내면 BasicAuthenticationFilter가 이를 인증한다.
  
세션이 필요 없고, 요청이 올 때마다 인증이 이루어진다.
> stateless, == 상태를 저장하지 않는다.
  
요청할 때마다 id, password가 반복해서 노출되기 때문에 보안에 취약하다.
> 이 필터를 사용할 때는 반드시 https를 사용하도록 권장된다.

다음과 같이 비활성화한다.
```java
// SpringSecurityConfig.java

@Override
protected void configure(HttpSecurity http) throws Exception {
    http.httpBasic().dissable();
}

```

## UsernamePasswordAuthenticationFilter

Form 데이터로 username, password 기반의 인증을 담당하는 필터다.
  
UsernamePasswordAuthenticationFilter는 다음과 같은 순서로 동작한다.

1. ProviderManager(AuthenticationManager)
   - 인증 정보 제공 관리자
2. AbstractUserDetailsAuthenticationProvider
   - 인증 정보 제공
   - 계정의 상태나 패스워드 일치 여부 등을 파악
3. DaoAuthenticationProvider
   - 유저 정보 제공
4. UserDetailsService
   - 유저가 제공하는 서비스

## CsrfFilter
Csrf Attack을 방어한다.

### Csrf Attack
악의적으로 페이지를 위조해서 정상 시스템에 악의적인 요청을 하는 것을 말함.
  
이를 방어하기 위해서, Csrf Token을 사용한다.

> 시스템에서 사용하는 정상적인 페이지는 올바른 csrf token을 갖고 요청할 수 있지만, 위조한 악의적인 페이지에는 본 시스템에서 사용하는 csrf token이 없기 때문에 틀린 csrf token으로 요청함

## RememberMeAuthenticationFilter
로그인을 장시간 유지 하도록 함.
  
remember-me cookie를 사용해서 login session이 만료되더라도, login session을 다시 연결 시켜준다. (로그인 유지 st)  
  
session 만료 시간의 기본값은 30분이지만 RememberMeAuthenticationFilter의 기본 설정은 2주다.
  
기본적으로 꺼져 있으며, 다음과 같이 세팅할 수 있다.

```java
// SpringSecurityConfig.java

@Override
protected void configure(HttpSecurity http) throws Exception {
    http.rememberMe();
}
```

## AnonymousAuthenticationFilter
인증이 안 된 유저가 요청을 하면 Anonymous User로 만들어서 anonymous user token을 Authentication에 넣어줌.
> 인증되지 않았다고 하더라도 null을 넣는 것이 아니라 기본 Authentication을 만들어준다.

다른 필터에서 Anonymous User인지 인증된 User인지 분기 처리를 할 수 있다.
  
다음과 같이 활성화

```java
// SpringSecurityConfig.java

@Override
protected void configure(HttpSecurity http) throws Exception {
    http.anonymous().principal("anonymousUser");
    // principal 없어도 됨. 이름 지정 st.
}
```

## FilterSecurityInterceptor

Interceptor로 끝나지만 Filter다.
  
FilterSecurityInterceptor로 넘어온 authentication의 내용을 기반으로 최종 인가 판단을 내린다.
> 필터중 뒤쪽에 위치.

- Authentication을 가져오고, 문제가 있다면 AuthentcationException을 발생시킨다.
- Authentication에 문제가 없으면 인가를 판단한다.
- 인가가 거절된다면 AccessDeniedException을 발생시킨다.

## ExceptionTranslationFilter

`FilterSecurityInterceptor에서 발생한 두 가지 Exception을 처리한다.
- `AuthentcationException`: 인증 실패
- `AccessDeniedException`: 인가 실패

인증이나 인가에 실패햇을 때, 어떤 행동을 취할지 결정한다.

### 기본 설정
다음 경우에는 로그인 페이지로 이동한다.
- AuthenticationException 발생
- Anonymous의 AccessDeniedException 발생

다음 경우에는 403 Forbidden Whitelable Error Page로 이동한다.
- 기명 User의 AccessDeniedException이 발생
