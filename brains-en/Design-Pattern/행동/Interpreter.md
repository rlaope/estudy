# Interpreter Pattern

The Interpreter pattern is a design pattern that **defines frequently used expressions in a separate language and reuses them**.

It can be seen as a pattern that defines and interprets the grammar of a simple language (rules) (e.g., regular expression analysis).

![](https://upload.wikimedia.org/wikipedia/commons/thumb/b/bc/Interpreter_UML_class_diagram.svg/1920px-Interpreter_UML_class_diagram.svg.png)

- A TerminalExpression is the basic, lowest-level unit in grammar rules; this class obtains the smallest unit of grammar to be interpreted.
- A NonTerminalExpression is a component of a composite expression. This class interprets the components within grammar rules. It is used to represent the structure of the grammar.

For example, consider a postfix expression for arithmetic operations like `xyz+-`. `x`, `y`, and `z` themselves are TerminalExpressions. An Expression that interprets `+` and `-` and uses the relevant TerminalExpressions to process the operations can be seen as a NonTerminalExpression.

> While multiple Expressions can be implemented with classes that implement the representative Expression interface, they can also be implemented using static methods since Java 8.

<br>

### Examples

A prime example can be found in Spring, specifically the `@Value("#{}")` annotation. Within `#{}` you can reference properties using SpEL notation. This is also an interpretation of an expression, namely SpEL.

Furthermore, using the regular expression `Pattern` class to validate regex can also be seen as an application of the Interpreter pattern.

<br>

### Advantages

An advantage is the ability to create classes for various pattern expressions, allowing for flexible extension. OCP

### Disadvantages
Disadvantages include slightly difficult debugging and potential performance impacts when dealing with complex grammars or many expressions. Therefore, this pattern should only be applied to frequently used expressions.
