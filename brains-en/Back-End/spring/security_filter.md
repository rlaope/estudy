# Spring Security Filter

## Filter

A Filter is a component that performs certain operations `before a request and after a response`.

![](https://user-images.githubusercontent.com/80656733/155106878-d64ba544-8ddf-489d-adc6-816bcde7541b.png)

It's fair to say that Spring Security's operations are essentially driven by Filters.

- Various filters perform different functions.
  - To understand what a filter does, you can examine the `doFilter` method of the `Filter` class.
- These filters can be excluded or added.
- You can define the order in which filters operate.

When there are multiple filters, they operate as follows:

![](https://user-images.githubusercontent.com/80656733/155108797-6fa23fe8-cc66-4514-a3a4-e8dc4cf33e19.png)

Spring Security has several filters, and we will explore them one by one.

## SecurityContextPersistenceFilter

This is usually the second filter to be executed.

> The first filter is `WebAsyncManagerIntegrationFilter`, which helps process the `SecurityContext` even for asynchronous requests.

This filter retrieves the `SecurityContext` and places it into the `SecurityContextHolder`.

If a `SecurityContext` is not found, it simply creates a new one.

## BasicAuthenticationFilter

Even without logging in, if you encode the ID and password in Base64 and include them in every request, `BasicAuthenticationFilter` will authenticate them.

No session is required, and authentication occurs with every request.
> Stateless, meaning it does not store state.

Since the ID and password are repeatedly exposed with each request, it is vulnerable to security risks.
> When using this filter, it is strongly recommended to use HTTPS.

Disable it as follows:
```java
// SpringSecurityConfig.java

@Override
protected void configure(HttpSecurity http) throws Exception {
    http.httpBasic().dissable();
}
```

## UsernamePasswordAuthenticationFilter

This filter is responsible for username and password-based authentication using form data.

The `UsernamePasswordAuthenticationFilter` operates in the following sequence:

1. ProviderManager(AuthenticationManager)
   - Authentication information provider manager
2. AbstractUserDetailsAuthenticationProvider
   - Provides authentication information
   - Checks account status, password matching, etc.
3. DaoAuthenticationProvider
   - Provides user information
4. UserDetailsService
   - User-provided service

## CsrfFilter
Defends against Csrf Attacks.

### Csrf Attack
Refers to maliciously forging a page to send malicious requests to a legitimate system.

To defend against this, a Csrf Token is used.

> A legitimate page used by the system can make requests with the correct CSRF token, but a forged malicious page will not have the CSRF token used by the system, thus making requests with an incorrect CSRF token.

## RememberMeAuthenticationFilter
Allows login to be maintained for an extended period.

Even if the login session expires, it uses a `remember-me` cookie to re-establish the login session (login persistence style).

The default session expiration time is 30 minutes, but the default setting for `RememberMeAuthenticationFilter` is two weeks.

It is off by default and can be configured as follows:

```java
// SpringSecurityConfig.java

@Override
protected void configure(HttpSecurity http) throws Exception {
    http.rememberMe();
}
```

## AnonymousAuthenticationFilter
When an unauthenticated user makes a request, it creates an `Anonymous User` and inserts an anonymous user token into the `Authentication`.
> Even if not authenticated, it doesn't insert `null` but rather creates a default `Authentication`.

Other filters can then branch their processing based on whether the user is an `Anonymous User` or an authenticated `User`.

Enable it as follows:

```java
// SpringSecurityConfig.java

@Override
protected void configure(HttpSecurity http) throws Exception {
    http.anonymous().principal("anonymousUser");
    // principal 없어도 됨. 이름 지정 st.
}
```

## FilterSecurityInterceptor

Although it ends with "Interceptor," it is a Filter.

Based on the `authentication` content passed to `FilterSecurityInterceptor`, it makes the final authorization decision.
> Located towards the end of the filter chain.

- It retrieves the `Authentication` and, if there's an issue, throws an `AuthenticationException`.
- If there's no issue with the `Authentication`, it proceeds to determine authorization.
- If authorization is denied, it throws an `AccessDeniedException`.

## ExceptionTranslationFilter

Handles two types of Exceptions thrown by `FilterSecurityInterceptor`.
- `AuthenticationException`: Authentication failure
- `AccessDeniedException`: Authorization failure

It determines what action to take when authentication or authorization fails.

### Default Settings
In the following cases, it redirects to the login page:
- `AuthenticationException` occurs
- `AccessDeniedException` occurs for an Anonymous user

In the following case, it redirects to a 403 Forbidden Whitelabel Error Page:
- `AccessDeniedException` occurs for an authenticated User
