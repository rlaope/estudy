# 스프링/Spring URL Mapping

### Servlet/JSP URL 주소
- 사용자가 서버에 접속해서 서비스를 받기 위해 입력하는 주소를 URL 이라고 부릅니다.
- URL 주소는 여러 의미를 가지고 있는 값들로 구성됩니다.
- 프로토콜://도메인주소(IP):포트번소/경로1/경로2/...
- 프로토콜 : 서버와 클라이언트 간의 통신을 위한 약속 (생략 시 http)
- 도메인 주소(IP 주소) : IP 주소는 같은 네트워크 망에서 컴퓨터를 구분하기 위해 제공되는 숫자로 구성된 고유 주소
- 포트번호 : 1부터 65535번까지 구성된 숫자, 컴퓨터내에서 프로그램을 구분하기 위해 사용 생략시 80
- Servlet/JSP에서 첫 번째 경로는 ContextPath라고 부른다. 하나의 서버에서 각 웹 애플리케이션을 구분하기 위해 지정되는 이름이며, 폴더의 이름이 ContextPath가 된다. 그 이후는 하위 경로가 된다.

<br>

### 요청 방식 지정하기
- Spring MVC는 요청 주소별로 메서드를 정의할 수도 있지만 같은 요청 주소에서 요청 방식에 따라 메서드를 정의할 수도 있습니다.
  
GET,POST,PUT,DELETE,PATCH 대해 처리할 수 있습니다.
```java
@Controller
public class TestController {
	
	@RequestMapping(value = "/test1", method = RequestMethod.GET)
	public String test1() {
		return "test1";
	}
```

### 하위 경로 통합하기
```java
@Controller
public class Sub1Controller {
	
	@RequestMapping(value = "/sub1/test3", method = RequestMethod.GET)
	public String sub1Test3() {
		return "sub1/test3";
	}
	
	@RequestMapping(value = "/sub1/test4", method = RequestMethod.GET)
	public String sub1Test4() {
		return "sub1/test4";
	}
}
```
위 처럼, 하위 경로가 중복될 시 이를 통합할 수 있다.

```java
@Controller
@RequestMapping("/sub1")
public class Sub2Controller {
	
	@RequestMapping(value = "/test3", method = RequestMethod.GET)
	public String test5() {
		return "sub2/test5";
	}
	
	@RequestMapping(value = "/test4", method = RequestMethod.GET)
	public String test6() {
		return "sub2/test6";
	}
}
```