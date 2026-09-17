# Observer Pattern

## What is the Observer Pattern
The Observer pattern is a design pattern where a list of observers, which are objects that observe changes in an object's state, is registered with the object. Whenever the object's state changes, it directly notifies each observer in the list, typically through a method. It is primarily used to implement distributed event handling systems. It is also known as the publish/subscribe model.

To put it very simply, the Observer pattern is a design pattern that sends notifications to related objects when the state of a particular object changes.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fdpoa8U%2FbtqZjvUHSeB%2Ff7deiNNAvQGkeu8CGa4Twk%2Fimg.jpg)

In the Observer pattern, there is a subject object and observer objects that need to be aware of state changes. Their relationship can be 1:1 or 1:N.

## Advantages and Disadvantages of the Observer Pattern

### Advantages
1. Changes in one object can be propagated to other objects in real-time.
2. Loose coupling makes the system flexible and eliminates dependencies between objects.

### Disadvantages
1. If used too extensively, state management can become difficult.
2. Problems with data distribution can potentially lead to significant issues.
