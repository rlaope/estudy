# Abstract Factory Pattern

## Abstract Factory Pattern
The Abstract Factory pattern groups related **objects together into a factory class**, and then creates **another factory to generate these factories based on conditions**, thereby creating objects.
  
> The Factory pattern delegates object creation based on conditions to a factory class, where the factory class creates the objects, which differs from the Abstract Factory.

When there's a factory producing computers, and let's say there are Samsung and LG as manufacturers for mice, keyboards, and monitors,
when producing a computer, all components must either be from Samsung or all from LG.
Let's assume that you can't have a Samsung keyboard and monitor with an LG mouse (though it might be possible).
  
Thus, computers must be produced with components from the same manufacturer,
in other words, a Samsung computer object must always be created as a bundle of Samsung mouse, keyboard, and monitor objects.
That is, objects must be produced consistently.
  
Also, at the code level, since there will be branching based on whether it's a Samsung computer or an LG computer,
similar to the Factory Method pattern, the part that creates objects based on conditions will be defined as a factory class.
  
<br>

### Why Use Abstract Factory - Problems with Using Factory Method
Let's implement the logic for producing computers using the Factory Method pattern.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Ft1.daumcdn.net%2Fcfile%2Ftistory%2F998FC33C5C25BD7C02)

Let's try producing a computer using the Factory Method pattern. However, there are many components like keyboards, mice, etc., not just the computer itself,
and if we use the Factory Method pattern as shown above, numerous factory classes will be created.

```java
    KeyboardFactory keyboardFactory = new KeyboardFactory();
    MouseFactory mouseFactory = new MouseFactory();
    BodyFactory bodyFactory = new BodyFactory();
    MonitorFactory monitorFactory = new MonitorFactory();
    SpeakerFactory speakerFactory = new SpeakerFactory();
    PrinterFactory printerFactory = new PrinterFactory();
```

If it's a Samsung computer, all components must be Samsung; if it's an LG computer, all must be LG. Therefore, the task of consistently creating objects can be written more concisely.
Thus, let's improve this by applying the Abstract Factory pattern to ensure all components are from the same manufacturer.

### Applying the Abstract Factory Pattern

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Ft1.daumcdn.net%2Fcfile%2Ftistory%2F995759355E34E9DE08)

The differences compared to before applying the pattern are:
- The factory class that decides which manufacturer's parts to select has been removed, and a computer factory class has been added.
- Samsung Computer Factory and LG Computer Factory are encapsulated by the Computer Factory interface, and since it's clear which manufacturer's parts will be created, they each create parts from their respective manufacturers.
- The `createComputer()` method, which produces a computer, is called from the `FactoryOfComputerFactory` class.

This concludes our look at what the Abstract Factory pattern is.
To summarize, before applying the pattern (using the Factory Method pattern), a factory was created for each component to form an object. However, since the components of that object are consistent,
by applying the Abstract Factory pattern, related objects are encapsulated all at once into a factory to consistently create objects.
