# 💻 Understanding the Concept and Operation of Dispatcher-Servlet

#### Overview

While studying concepts like Spring AOP, Filter, and Interceptor, the Dispatcher-Servlet often comes up.

Since my understanding of the Dispatcher-Servlet is still lacking, I'd like to organize its concept and operational process.

#### Servlet (Java Servlet)

***A Java Servlet is a server-side program or specification that uses Java to create web applications.***

*Often simply called a Servlet, Java Servlets are a type of Java class used to improve the performance of web servers.*

*In other words, a Servlet is a web programming technology that processes client requests and returns the results.*

### Dispatcher-Servlet

The word "dispatch" means "to send," and incorporating this word, the Dispatcher-Servlet is defined as a **Front Controller** that

first receives all incoming HTTP requests and delegates them to the appropriate controller.

*What is a Front Controller?*

*It is a controller that primarily receives and processes all client requests coming into the server, positioned at the very front of the servlet container.*

*It is a design pattern used in the MVC architecture. (In the past, without this, there was the inconvenience of registering all URLs in web.xml...)*

Requests from clients are received by a servlet container like Tomcat, and the Dispatcher-Servlet, acting as the front controller, receives all these requests first. The Dispatcher-Servlet handles common tasks first, then finds the controller corresponding to the request and delegates the task.

**Advantages**

-> The Dispatcher-Servlet handles all requests coming into the application.

-> It handles common tasks. (We only need to implement the controller, and the Dispatcher-Servlet conveniently delegates tasks to it.)

**but**

The Dispatcher-Servlet encounters a problem when processing static resources. The reason is that the Dispatcher-Servlet

**processes all requests, which can lead to situations where static resources like images, HTML, CSS, and JavaScript cannot be loaded.**

To solve this, developers devised two methods:

1. Separate static resource requests from application requests.

2. Process requests for the application, and if not found, treat them as requests for static resources.

**1. Separate static resource requests from application requests**

-> Accessing `/app` URL is handled by the Dispatcher Servlet.

-> Accessing `/resource` URL is not handled by the Dispatcher Servlet as it cannot control it.

This seems like a good approach, but designing it this way makes the code quite messy, and since every request would need to include such URLs (app, resource), it wouldn't be an intuitive design.

**2. Process requests for the application, and if not found, treat them as requests for static resources**

Feeling the limitations of separating static resource requests from application requests, this method was devised.

**If a controller for a request cannot be found, the system secondarily searches for resources in configured resource paths.**

Separating areas in this way is not only efficient for resource management but also has the advantage of facilitating future expansion.

###

### Operation Process

![](image/dispatcher_servlet_0.png)

The image above illustrates the processing flow of the Dispatcher-Servlet. As mentioned earlier, the Dispatcher-Servlet **finds the appropriate controller and method and delegates the request.**

1. The Dispatcher-Servlet receives the client's request.

2. It finds the controller to delegate the request to, based on the request information.

3. It finds and passes the HandlerAdapter that will delegate the request to the controller.

4. The HandlerAdapter delegates the request to the controller.

5. The business logic is processed.

6. The controller returns a value.

7. The HandlerAdapter processes the return value.

8. The server's response is returned to the client.

**The Dispatcher-Servlet receives the client's request.**

![](image/dispatcher_servlet_1.png)

The image above illustrates the processing order. As explained earlier, the Servlet is the front controller that receives requests first.

After passing through filters in the Servlet Context (Web Context), the Dispatcher-Servlet in the Spring Context receives the request first.

Actually, the Interceptor does not delegate the request to the Controller. It merely illustrates the processing order as mentioned above.

**It finds the controller to delegate the request to, based on the request information.**

The Dispatcher-Servlet must find the controller to process the request and call its method.

One of the HandlerMapping implementations, **RequestMapping HandlerMapping, parses all controller beans annotated with `@Controller` and manages them as a HashMap (request information, target for processing).** Strictly speaking, it doesn't find the controller itself, but rather a HandlerMethod object that holds the controller and its method mapped to the request. So, when a request comes in, the Handler Mapping creates request information, which is a Key object, using the HTTP Method, URI, etc., and finds the HandlerMethod, which is the Value, to process the request, wrapping it in a HandlerMethodExecutionChain and returning it. The reason for wrapping it in a HandlerMethodExecutionChain is to include **interceptors** and other elements that need to be processed before passing the request to the controller.

**It finds and passes the HandlerAdapter that will delegate the request to the controller.**

The Dispatcher-Servlet does not directly delegate the request to the controller but delegates it through a friend called HandlerAdapter.

The reason for calling the controller via the adapter interface at this time is that there are various ways to implement controllers.

Although recently controllers are mostly written using `@Controller` and `@RequestMapping` annotations,

it's also possible to write classes using the Controller interface. Spring applies the **Adapter Pattern** through the HandlerAdapter interface, allowing it to **delegate requests regardless of the controller's implementation method.**

**The HandlerAdapter delegates the request to the controller.**

Before the HandlerAdapter delegates the request to the controller, common pre/post-processing steps are required.

This typically includes interceptors, ArgumentResolvers for processing `@RequestParm`, `@RequestBody`, etc.,

and **ReturnValueHandlers** that handle tasks such as serializing the `ResponseEntity`'s Body to JSON during the response. These are processed by the adapter before being passed to the controller.

Then, it delegates the request to call the controller's method. Reflection is actually used in the process of delegating the request.

The HandlerMethod object, which contains information about the target to process the request, has controller information and a method object, so it invokes the method object using reflection.

*What is Reflection?*

*A Java API that allows access to a class's methods, types, and variables even without knowing the specific class type.*

In fact, the HandlerMethod actually contains the controller bean name, method, and bean factory, so it finds the controller bean from the bean factory.

Then, it uses reflection on that controller bean object.

**The business logic is processed.**

The controller calls the service to process the business logic.

**The controller returns a return value.**

After the business logic is processed, the controller returns a return value. (ResponseEntity, View Name)

**The HandlerAdapter processes the return value.**

The HandlerAdapter processes the response received from the controller using the ReturnValueHandler, which is a response processor, and then returns it to the Dispatcher-Servlet.

If the controller returns a ResponseEntity, the HttpEntityMethodProcessor uses a MessageConverter to serialize the response object and set the response status. If the controller returns a view name, the view resolver returns the view.

**The server's response is returned to the client.**

The response returned through the Dispatcher-Servlet passes through filters again before being returned to the client.

References

<https://velog.io/@han_been/%EC%84%9C%EB%B8%94%EB%A6%BF-%EC%BB%A8%ED%85%8C%EC%9D%B4%EB%84%88Servlet-Container-%EB%9E%80>
