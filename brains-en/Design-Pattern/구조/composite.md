# Composite Pattern

## What is a Composite
In OOP, a composite is an object designed to be composed of one or more similar objects, all exhibiting similar functionality.
This allows you to **manipulate a single object as if you were manipulating a group of objects**.

## Composite Pattern
The Composite pattern aims for clients to treat `composite objects` and `individual objects` uniformly.
Here, the intent of the composite is to represent whole-part relationships by structuring them as a tree.

When dealing with tree structures, programmers must distinguish between leaf nodes and branches.
This often introduces significant complexity into the code, leading to many errors.
To solve this, an interface can be created to treat complex and primitive objects uniformly.

Consequently, the Composite pattern leverages the **inherent concept of composition through interfaces**.

## When to Use the Composite Pattern
If the processing methods for composite objects and individual objects are not different, they can be defined as a whole-part relationship.
A typical example of a whole-part relationship is Directory-File.
It is useful when efficiently defining such whole-part relationships.

- When you want to represent whole-part relationships as a tree structure
- When you want clients to treat parts and related objects uniformly within a whole-part relationship

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Ft1.daumcdn.net%2Fcfile%2Ftistory%2F99E9FF455C84AF1E20)

The meaning of the UML diagram is as follows
We can see that the Client class does not directly reference the Leaf and Composite classes, but rather references the common interface, Component.
The Leaf class implements the Component interface.
The Composite class maintains child Component objects and forwards requests, such as operation(), to its children.

Looking at each from a code perspective:

- Component
  - An abstract concept for all components, serving as the interface for Leaf and Composite classes.
- Leaf
  - Implements the Component interface and represents a concrete class.
- Composite
  - Implements the Component interface, has children that implement it, and implements methods to manage these children. Additionally, methods defined in the interface typically delegate processing to the children.
  - Composite.operation() => Leaf.operation() A more detailed understanding can be gained through the example below.

Based on this, understanding becomes easier when looking at the object diagram.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Ft1.daumcdn.net%2Fcfile%2Ftistory%2F9923A84E5C84B5203A)

The Client sends a request to the top-level Composite in the tree structure. Then, objects implementing the Component interface forward this request downwards to all child elements based on the tree structure.
This can be seen as an action that occurs at runtime.
