# State Machine Pattern

The State Machine Pattern is **a pattern that allows an object to change its behavior when its internal state changes**.

When states and behaviors associated with those states are given, and you want to execute the behavior corresponding to a state when it changes, defining the object's state and state-specific behaviors, and when you want to add new states or modify existing state behaviors independently of other states.

![](https://user-images.githubusercontent.com/35602698/103554291-00f46180-4ef2-11eb-9457-0efae99578e2.png)

As shown above, one way to implement a structure where states and their associated behaviors are given, and the behavior corresponding to a state is executed when the state changes, is to define state-specific behaviors directly within a class that depends on the internal state. When defining states and state-specific behaviors inside a class, conditional statements or switch statements can be used. Each conditional branch implements the behaviors corresponding to a state.

However, this approach makes it impossible to define state-specific behaviors within a class and later add new states or modify existing behaviors independently of the class. Classes containing state-specific operations would also be much harder to implement, modify, test, and reuse. Due to these issues, the GoF provides several approaches that allow new states to be added and existing state behaviors to be changed independently.

![](https://user-images.githubusercontent.com/35602698/103554326-136e9b00-4ef2-11eb-80f7-cb4292feaa3e.png)

As shown above, state-specific behaviors are separated into State objects and then encapsulated.

A class that has a state does not implement state-specific behaviors directly within itself but delegates them to the current State object.

The core idea of this pattern is to encapsulate an object's state-specific behaviors into State objects. This encapsulation allows the object's state to be managed by separate State objects, enabling independent modification from other State objects.

The implementation of a State object is as follows.
- For all states, define a State interface, and within the state interface, define an `operation()` method that specifies the behavior for the state.
- Each state defines a concrete class that implements the state interface.
- No separate conditional statements are needed because the behavior changes simply by changing the current state.
- It provides compile-time flexibility due to inheritance. All state-specific code resides in state subclasses, and adding a new state is easily done by defining a new state subclass.

The class that has a state delegates the responsibility of executing state-specific behaviors to the State object. It gains runtime flexibility due to object composition. By changing the current state at runtime, its behavior can be altered.

- Context: The class that holds the state
- State: The state class
- State N: The state implementation class

### Dynamic Object Collaboration

The Context object delegates state-specific behaviors to the current State object, and interaction begins with the Context object calling the behavior on the current State object. The Context object passes itself to the State object. Each State executes its own behavior. If the behavior is executed and the state needs to change, it calls `context.setState()` to change the Context object to a different state.

This pattern allows new states to be easily added, eliminates the need for conditional statements, and ensures consistent states. Since the Context's state can only be changed by the current State object, consistency of the state is guaranteed, and state changes occur implicitly.

However, the Context interface might need to be extended, or the Context interface might need to be extended to allow State objects to change the Context's state.

> Introduces an additional level of indirection. State achieves flexibility by introducing an additional level of indirection (clients delegate to separate State objects), which makes clients dependent on a State object.
