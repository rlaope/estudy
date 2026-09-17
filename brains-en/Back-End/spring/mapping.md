# Spring/Spring URL Mapping

### Servlet/JSP URL Address
- The address users enter to access a server and receive services is called a URL.
- A URL address is composed of values with various meanings.
- Protocol://DomainAddress(IP):PortNumber/path1/path2/...
- Protocol: A convention for communication between server and client (http if omitted).
- Domain Address (IP Address): An IP address is a unique, numerical address provided to distinguish computers within the same network.
- Port Number: A number ranging from 1 to 65535, used to distinguish programs within a computer (80 if omitted).
- In Servlet/JSP, the first path segment is called the ContextPath. It's a name assigned to distinguish each web application on a single server, and the folder's name becomes the ContextPath. Subsequent segments are sub-paths.

<br>

### Specifying Request Methods
- Spring MVC allows defining methods per request URL, but it also allows defining methods based on the request method for the same URL.
  
It can handle GET, POST, PUT, DELETE, and PATCH.
```java
@Controller
public class TestController {
	
	@RequestMapping(value = "/test1", method = RequestMethod.GET)
	public String test1() {
		return "test1";
	}
```

### Consolidating Sub-Paths
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
As shown above, when sub-paths are duplicated, they can be consolidated.

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
