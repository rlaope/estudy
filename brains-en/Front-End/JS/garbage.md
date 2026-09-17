# Garbage Collection
JavaScript performs memory management behind the scenes.

Everything we create – primitive values, objects, functions, etc. – occupies memory. So, what happens to things that are no longer useful? From now on, we'll explore how the JavaScript engine identifies and deletes unnecessary items.

<br>

## Garbage Collection Criteria
JavaScript uses the concept of reachability for memory management.

A `reachable` value, simply put, means a value that can be accessed or used in some way. Reachable values are not deleted from memory.

1. The values listed below are reachable by their very nature and are not deleted without explicit reason.

- Local variables and parameters of the current function
- Variables and parameters used by functions in the chain of nested functions
- Global variables
- And so on

Such values are called `roots`.

2. Values referenced by a root, or values that can be referenced from a root via a chain, become reachable.

Let's assume an object is stored in a global variable. If a property of this object references another object, and a property of that object references yet another object, then the object referenced by the property becomes a reachable value. Everything else referenced by this object is also considered reachable. We'll look at a detailed example below.

- Inside the JavaScript engine, the `garbage collector` constantly runs. The garbage collector monitors all objects and deletes those that are unreachable.

<br>

```js
// user엔 객체 참조 값이 저장됩니다.
let user = {
  name : "John"
};
```
The global variable `user` references an object `{name : "John"}`. John's `name` property is expressed within the object because it stores a primitive value.
If we overwrite the value of `user` with another value, the reference disappears.
```js
user = null;
```
Now, John has become unreachable. All ways to access John, and all references to John, have disappeared. The garbage collector will now delete the data stored in John and remove John from memory.
