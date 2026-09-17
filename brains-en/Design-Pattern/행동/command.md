# Command Pattern

The Command Pattern is a pattern where an invoker **does not directly depend on multiple objects to execute actions, but rather performs actions through a command (Command)**.

Using the Command Pattern, requests can be encapsulated, making the command object's dependency on the objects that need to execute commands loose.

### Problem and Example

For example, let's say we've created a Button class. And let's imagine these buttons have a huge number of subclasses. There are buttons that perform various functions,

To use these buttons, the invoker would have to depend on and use all of them.

However, there's a problem here: every time the existing Button class is modified, there's a risk of breaking the code of these child classes.

Graphical user interface code awkwardly depends on the unstable code of the business logic.

### Solution
The correct solution is based on the principle of separation of concerns.

Buttons and clients are made to depend on each other, and they also depend on Command.

And by having the button (receiver) depend on the command, if related logic is placed into a single action (execute business logic),

from the user's perspective, there is the advantage that no constraints arise even if changes occur.

![구조](https://refactoring.guru/images/patterns/diagrams/command/structure.png?id=1cd7833638f4c43630f4a84017d31195)
Structure
