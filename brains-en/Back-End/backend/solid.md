# SOLID Principles and the Pros and Cons of Each

The SOLID principles are fundamental principles of software design, used in object-oriented programming.

## SRP (Single Responsibility Principle)

A class should have only one responsibility.

In other words, a class should perform only one function.

### Pros
Lower coupling between classes makes maintenance easier.

Since a single class performs only one function, code readability and reusability increase.

### Cons
The number of classes increases.

This means that names must be given to distinguish classes, and additional time and effort are required to understand the relationships between classes.

<br>

## OCP (Open-Closed Principle)
Open for extension, closed for modification.

This principle means that software entities should be designed so that new functionality can be added without modifying existing code.

### Pros
When new requirements or features are added, new code can be added without modifying existing code.

This has the advantage of increasing code reusability and maintainability.

Since changes to existing code are minimized, highly reliable code can be written.

Furthermore, developers can write new code without needing to understand and modify existing code, which improves productivity.

### Cons
Additional consideration is required for interface design.

Applying OCP too excessively can increase code complexity, so it should be applied at an appropriate level.

<br>

## LSP (Liskov Substitution Principle)
Subclasses should be able to substitute for their parent classes.

This is necessary to support polymorphism.

### Pros
Using LSP, code that uses a parent class works identically with a child class, increasing code readability and reusability.

Even if new features are added or existing features are changed in a child class, it does not affect the code using the parent class, making maintenance easier.

### Cons
Following LSP requires additional consideration for interface design.

Furthermore, if LSP is not followed, polymorphism may not be supported, and issues with reduced readability and maintainability can arise, so interfaces must be well-designed and inheritance relationships appropriately structured.

<br>

## ISP (Interface Segregation Principle)
Clients should not be forced to depend on methods they do not use.

This principle is necessary to reduce coupling through interface segregation.

### Pros
Following ISP makes interfaces small and simple, so changing an interface's implementation does not affect client code.

Additionally, following ISP means client code using an interface only uses the features it needs, which increases code readability and reusability.

### Cons
However, following ISP requires additional consideration for interface design.

Applying ISP too excessively can lead to an increase in the number of interfaces, which can reduce code readability.

<br>

## DIP (Dependency Inversion Principle)
Abstractions should not depend on concretions. This principle helps write flexible code by inverting dependencies.

### Pros
Following DIP means depending on abstract interfaces rather than concrete implementations, so changing an implementation does not affect client code.

Client code using an interface and its implementation can be developed independently, which increases code reusability and maintainability.

### Cons
Following DIP requires additional consideration for interface design.

Applying DIP too excessively can lead to an increase in the number of interfaces, which can reduce code readability.

It is important to balance abstraction and concretization at an appropriate level.

<br>

## Conclusion
In my opinion, all principles seem to have keywords like 'maintainability' and 'readability' in their advantages, and issues like 'interface design,' 'design considerations,' and 'excessive application' in their disadvantages.

Perhaps because they are object-oriented design principles, their pros and cons seem quite similar.

Anyway, let's apply them well to write more object-oriented code.
