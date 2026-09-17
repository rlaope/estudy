# Java Backend Web Technology History

### Past
- 1997 `Servlet`
  - Since it had to be written in Java code, dynamically generating HTML was difficult.
- 199 `JSP`
  - HTML generation was convenient, but it ended up taking on too many roles, including business logic.
- `MVC Pattern` combining Servlet and JSP
  - Developed by dividing roles into Model, View, and Controller.
- Early 2000s ~ Early 2010s `MVC Framework Warring States Period`
  - Automates the MVC pattern.
  - Supports various features that make complex web technologies easy to use.

<br>

### Present
- Annotation-based Spring MVC
  - @Controller
  - The Spring and Autumn Warring States period of MVC frameworks comes to an end.
- Emergence of Spring Boot
  - Spring Boot, with an embedded server, emerges.
  - In the past, WAS was directly installed on the server, and source code was packaged into a WAR file and deployed to the installed WAS.
  - Spring Boot simplified builds by including the WAS server within the JAR, which is the build artifact.

<br>

### Latest
Spring web technologies are diversifying.
- Web Servlet
  - Spring MVC
- Web Reactive
  - Spring WebFlux

#### WebFlux
- Handles non-blocking operations asynchronously.
- Achieves maximum performance with a minimum number of threads.
  - Optimizes thread context switching costs.
- Development is possible in a functional style.
  - Increases the efficiency of concurrent processing code.
- Does not use Servlet technology.
  - Uses Netty.

However, WebFlux has a very high technical difficulty. Support for RDBs is still lacking. The thread model of traditional MVC is also fast enough, so it is not yet widely used in practice.

<br>

### Java View Templates
This is a view feature that conveniently generates HTML.

#### JSP
It has various features and has solved speed issues.

#### Thymeleaf
As a natural template, it allows applying view templates while preserving the HTML structure. Its integration with Spring MVC makes it the best choice. However, Freemarker and Velocity are faster in terms of performance.
