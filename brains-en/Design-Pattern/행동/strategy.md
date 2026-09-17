# Strategy Pattern

## Strategy Pattern
It refers to a method of **flexibly extending an object's behavior by simply changing the strategy** when dynamic behavior modification is desired, rather than directly performing the action, by **creating a strategy class for each action** an object can perform and defining an interface that **encapsulates similar actions**.

In short, it's a pattern where each action an object can perform is made into a strategy, allowing behavior to be modified dynamically simply by changing the strategy when modification is needed.

### Why Use the Strategy Pattern

For example, let's assume there are Train and Bus classes, and both implement the Movable interface. There's also a Client that uses these objects.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Ft1.daumcdn.net%2Fcfile%2Ftistory%2F9916204B5BF8DAD105)

Train and Bus Classes

Trains move along tracks, and buses move along roads. Now, let's imagine that over time, a bus that moves along tracks is developed. In that case, only the bus's `move` method would need to be modified.

However, modifying it this way violates the OCP (Open/Closed Principle) among the SOLID principles. According to OCP, behavior should be modified without altering existing `move()` methods, but here, the bus's `move()` method was directly modified.

Furthermore, changes made in this manner make maintenance difficult when the system expands. For example, if taxis, private cars, express buses, motorcycles, etc., that move along roads like buses are added, they would all use the `move()` method just like buses. If, like the newly developed track-moving bus, track-moving taxis, private cars, express buses, etc., were to emerge, not only would the `move()` methods of taxis, private cars, and express buses have to be individually modified, but also, since the same method is defined identically across multiple classes, method duplication would occur.

In other words, the problems are:
- OCP violation
- Method duplication issues when the system grows and expands

Therefore, to solve this, we will use the Strategy Pattern.

## Strategy Pattern Implementation
This time, in a situation where a bus that moves along tracks has been developed as described above, we will try using the Strategy Pattern to allow the system to be flexibly changed and extended.

1. How to create a strategy

Currently, there are two types of transportation methods: rail and road. Therefore, we create Strategy classes for the moving methods (RailLoadStrategy, LoadStrategy).

These two classes implement the `move()` method, defining how they move. Additionally, to encapsulate these two strategy classes, we create a MovableStrategy interface. The reason for this encapsulation is a design consideration for cases where not only transportation strategies but also other strategies might be additionally extended.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Ft1.daumcdn.net%2Fcfile%2Ftistory%2F99804D3E5BF8E1C80C)

Creating Strategy Classes

2. Define classes for transportation methods

Transportation methods like trains and buses can move via a `move()` method. However, instead of directly implementing the movement method, we set a strategy for how they will move, and they move using that strategy's movement method.

Therefore, there is a `setMovableStrategy()` method for setting the strategy.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Ft1.daumcdn.net%2Fcfile%2Ftistory%2F997EE1415BF8E4BD13)

Defining Transportation Classes

3. Now, implement the Client that uses Train and Bus objects.

After creating Train and Bus objects, the `setMovableStrategy()` method is called to set how each transportation method moves.

And to see how flexibly logic can be modified in the program when using the Strategy Pattern, we create a scenario where a bus that moves along tracks has been developed and modify the bus's movement strategy.

```java
public class Client {
    public static void main(String args[]){
        Moving train = new Train();
        Moving bus = new Bus();

        /*
            기존의 기차와 버스의 이동 방식
            1) 기차 - 선로
            2) 버스 - 도로
         */
        train.setMovableStrategy(new RailLoadStrategy());
        bus.setMovableStrategy(new LoadStrategy());

        train.move();
        bus.move();

        /*
            선로를 따라 움직이는 버스가 개발
         */
        bus.setMovableStrategy(new RailLoadStrategy());
        bus.move();
    }
}
```
