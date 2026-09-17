# Decorator Pattern

This pattern dynamically adds responsibilities (features) to an object based on the given situation and use case.
  
As the name "decorator" suggests, it's easy to think of it as decoration.
  
It's a design approach where you create a class with basic functionality and then design it to easily add additional features.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbnP6V5%2Fbtq1JVdeaQb%2F0DLKqgOGPfhb2qhfZmKWRk%2Fimg.png)

- Component: Controls the actual instance.
- ConcreteComponent: A part of the Component's actual instance, acting as the subject of responsibility.
- Decorator: Makes Component and ConcreteDecorator interchangeable.
- ConcreteDecorator: The actual decoration instance and definition, acting as the subject of added responsibility.

## Pros and Cons of the Decorator Pattern

### Pros
1. Behaviors can be extended through the Decorator pattern without modifying existing code.
2. New behaviors can be added at runtime through composition and delegation.

### Cons
1. Too many meaningless objects can be added.
2. Overuse of decorators can make the code unnecessarily complex.

### The Decorator pattern is beneficial in the following situations.

1. When a structure is needed where class elements are continuously modified and used.
2. When a class structure combines multiple elements.
