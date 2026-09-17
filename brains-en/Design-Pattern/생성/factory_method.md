# Factory Method Pattern

The Factory Method pattern is a design pattern that encapsulates object creation within a factory class, allowing it to handle the creation instead.

In other words, instead of the client directly creating product objects using the `new` operator, a factory class is created to take charge of product object creation. Then, methods in sub-factory classes that inherit from this factory class each take responsibility for creating various product objects.

It also has the characteristic of pre-configuring the necessary object creation process like a template, allowing for flexible object determination by handling the creation process in various ways through pre-processing or post-processing related to object creation.

## Factory Method Pattern

## Factory Method Pattern Structure

![](https://velog.velcdn.com/images%2Fjamieshin%2Fpost%2F715a6f1b-2622-46fa-b945-531f0e5e874d%2Fimage.png)

- Creator
  - As the top-level factory class, it abstracts the factory method, allowing subclasses to implement it.
  - **Object Creation Processing Method**: A method that templates pre-processing and post-processing related to object creation.
  - **Factory Method**: An abstract object creation method to be overridden in sub-factory classes.
- ConcreateCreator: Each sub-factory class overrides the abstract creation method to return a product object that matches it. In other words, for each product object, there is a corresponding factory object responsible for its creation.
- Product: Abstracts the product implementation.
- ConcreateProduct: The product implementation.

In summary, the Factory Method pattern can be seen as a pattern for creating factories that produce objects. Which class instance to create is determined by the pre-defined factory subclass.

The reason for this seemingly cumbersome configuration for mere object creation is that it lowers the **coupling** between objects and facilitates maintenance.

## Relationship Between Template Method Pattern and Factory Method Pattern
Given their similar naming, one might assume a relationship, but the Template Method is a `behavioral pattern` and the Factory Method is a `creational pattern`, making them entirely different. However, their class structure can be considered similar because the Factory Method pattern is essentially a factory for creating instances configured using the Template Method pattern. In the Template Method pattern, abstract methods were inherited to allow subclasses to define the specific processing algorithm. Applying this logic not to algorithm content but to instance creation is what the Factory Method pattern does.
