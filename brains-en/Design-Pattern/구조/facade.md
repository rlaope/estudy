# Facade Pattern

The Facade pattern provides a unified interface to a set of interfaces in a subsystem.

A facade is a higher-level interface that makes the subsystem easier to use.

The participants and roles in the Facade pattern are as follows:

- Facade: Delegates client requests to the appropriate subsystem classes.
- Subsystem classes: Implement the subsystem's functionality. Subsystem classes are used only by the facade.
- Client: Requests the Facade to perform a specific action.

Let's take an online shopping mall's order system as an example.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbXeHtA%2FbtqKjbJzoIU%2FRWg13WRnONqPPYL4Ulcls0%2Fimg.png)

In the current situation, the client must interact multiple times with services implemented by subsystem classes and needs to know information about these subsystem classes.

In other words, the client is tightly coupled with the subsystem.

Therefore, changes in the service layer affect the client. (If the DB needs to change from Oracle to NoSQL, this affects the client as well.)

In such cases, the Facade pattern can be considered.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FcYthYY%2FbtqJ8fUPLhT%2FIaGSHoaKFKlaNuzOFdRdz1%2Fimg.png)

What the client wants is to place an order; they are not interested in inventory, shipping, or payment details.

Therefore, an interface (facade) that makes the subsystem easier to use was introduced to simplify placing an order.

The facade handles the order by appropriately using each of the services.

As a result of applying the Facade pattern, changes in subsystem classes do not affect client code (loosely coupled).

The core of the Facade pattern lies in reducing interaction complexity. Thus, even when three services are processed sequentially within a single transaction, as shown above, it's a Facade pattern, and using them independently, as shown below, is also a Facade pattern.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FcyUZSz%2FbtqKhllBRzx%2Fiop6ljryxru8kz3Wk0Ru2K%2Fimg.png)

A facade named ShapeMaker helps the client draw shapes without needing to know about each shape's class and methods (reducing complexity).

## Comparison

### Facade vs Adapter
The purpose of the Facade pattern is to reduce interaction complexity with a subsystem, while the Adapter pattern aims to adapt an existing interface to a different interface that the client wants to use.

### Facade vs Front Controller

A facade can be thought of as a layer that wraps complex functionality and provides simpler methods for interaction, aiming to hide the complexity of the internal system.

A facade should not contain logic beyond listening to what the client wants and using the internal system appropriately to fulfill the request (which can be seen as a form of translation).

On the other hand, a controller itself can have its own logic. Furthermore, the purpose of a Front Controller is not so much to hide the complexity of the internal system, but rather to effectively apply common logic that needs to be processed for all requests by having a single controller that receives all requests first.
