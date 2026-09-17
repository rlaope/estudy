# (Spring boot + Security) Configuring SecurityConfig

### Spring Security?
- It is a Java / Java EE framework that provides authentication, authorization, and other security features for enterprise applications.

> Used to control access or grant permissions to specific pages, or to encrypt passwords stored in the DB (and many other uses).

### Config File for Security Settings

```java
@Configuration
public class SecurityConfig extends WebSecurityConfigurerAdapter {

    @Bean // Method executed during login
    public AuthenticationProvider authenticationProvider(){return new LoginAuthenticationProvider();}

    @Bean // Method executed upon successful login
    public AuthenticationSuccessHandler successHandlerHandler() {
        return new LoginSuccessHandler();
    }

    @Bean // Method executed upon failed login
    public AuthenticationFailureHandler failureHandlerHandler() {
        return new LoginFailureHandler();
    }

    @Bean // Method related to password encryption
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
                .csrf().disable()// Proceeding by utilizing JWT tokens without sessions, disabling csrf token check
                .authorizeRequests() // Configure authentication procedures
                .antMatchers("/", "/error/*", "/login", "/loginProc").permitAll() // Configured URLs are accessible to anyone without authentication
                .anyRequest().authenticated()// Access to pages other than those above requires authentication (regardless of ROLE)
                .and()
                .formLogin().loginPage("/login")  // URL to redirect to when clicking a blocked page
                .loginProcessingUrl("/loginProc") // URL mapped during login
                .usernameParameter("userId")      // Name mapped to the login ID in the view form tag (name of the form)
                .passwordParameter("userPw")      // Name mapped to the login password in the view form tag (name of the form)
                .successHandler(successHandlerHandler()) // Method executed upon successful login
                .failureHandler(failureHandlerHandler()) // Method executed upon failed login
                .permitAll()
                .and()
                .logout() // Logout settings
                .logoutUrl("/logout") // URL mapped during logout
                .logoutSuccessUrl("/") // Redirect URL upon successful logout
                .invalidateHttpSession(true); // Clear session
    }
}
```

### @Configuration
- An annotation that helps with environment setup (Spring's basic configuration information).
- Declaring a class with the `@Configuration` annotation informs Spring that this class is an environment configuration file and that objects are Beans via the `@Bean` annotation.

### @Bean
- It makes the object returned by a method written by the developer into a Bean.
- A Bean is a Java object managed by the Spring IoC container.


<br>

### WebSecurityConfigureAdapter

- This is a class related to Spring Security configuration, and security is configured by overriding the methods within this class.

Methods related to login: The classes returned must be custom implemented.
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

Method related to password encryption
```java
@Bean
public PasswordEncoder passwordEncoder(){
    return new BCryptPasswordEncoder();
}
```

Method that configures static pages to be accessible from anywhere

```java
@Override
public void configure(WebSecurity web) throws Exception {
    web.ignoring().antMatchers("/css/**", "/js/**");
}
```

Method that controls access permissions

```java
@Override
protected void configure(HttpSecurity http) throws Exception {
    http
            .csrf().disable()// Proceeding by utilizing JWT tokens without sessions, disabling csrf token check
            .authorizeRequests() // Configure authentication procedures
            .antMatchers("/", "/error/*", "/login", "/loginProc").permitAll() // Configured URLs are accessible to anyone without authentication
            .anyRequest().authenticated()// Access to pages other than those above requires authentication (regardless of ROLE)
            .and()
            .formLogin().loginPage("/login")  // URL to redirect to when clicking a blocked page
            .loginProcessingUrl("/loginProc") // URL mapped during login
            .usernameParameter("userId")      // Name mapped to the login ID in the view form tag (name of the form)
            .passwordParameter("userPw")      // Name mapped to the login password in the view form tag (name of the form)
            .successHandler(successHandlerHandler()) // Method executed upon successful login
            .failureHandler(failureHandlerHandler()) // Method executed upon failed login
            .permitAll()
            .and()
            .logout() // Logout settings
            .logoutUrl("/logout") // URL mapped during logout
            .logoutSuccessUrl("/") // Redirect URL upon successful logout
            .invalidateHttpSession(true); // Clear session
}
```
