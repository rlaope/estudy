# sealed class

Let's look at the background of `sealed class`. When a single parent class is inherited, **the compiler doesn't know which child classes have inherited from that parent class.**

For example, let's say we're building an app that records running workouts. I'll create classes for a person's state. The types of states are Running, Walking, and Idle, which can be translated into the following code.

```kotlin
abstract class PersonState

class Running : PersonState()
class Walking : PersonState()
class Idle : PersonState()
```

Let's write a function to get a status message for each PersonState.

```kotlin
fun getStateMessage(personState: PersonState): String {
    return when (personState) {
        is Running -> "Person is running"
        is Walking -> "Person is walking"
        is Idle -> "Person is doing nothing"
    }
}
```

However, the code above throws an error, asking to add an `else` branch, as shown below.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fb2p56T%2Fbtrd5jePobA%2FLnzixISyxpt9dblcUXa58k%2Fimg.png)

Adding `else` does solve the issue...

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fdrgxol%2Fbtrd05a3Yar%2FkNxxjUKGR2iMjUmYKrEY0k%2Fimg.png)

Why did it ask to add `else`? As mentioned above, it's because the compiler doesn't know the types of PersonState's child classes.

This isn't a simple problem. Let's try removing the `Idle` check from that `when` statement. You'll see that it compiles without errors.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fbf9Ez5%2Fbtrd04JXPSM%2FpnIw4NEQXlvUMOKpnCMmqK%2Fimg.png)

If this were deployed in a real application at runtime, it wouldn't work correctly because the `Idle` state wouldn't be handled. In other words, a critical problem arises that cannot be caught at compile time, even if there's an issue in the code.

`sealed class` solves these kinds of problems.

### sealed class

A `sealed class` is an `abstract class` that has the characteristic of restricting the types of child classes that can inherit from it. In other words, **the compiler knows which child classes a `sealed class` has.**

```kotlin
package com.example.demo

sealed class PersonState

class Running : PersonState()
class Walking : PersonState()
class Idle : PersonState()
```

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbLcMR4%2Fbtrd642NmG2%2FmmcKRdFK7aOrgPyqK29FJK%2Fimg.png)

As shown above, no error occurs even without adding an `else` branch, because the compiler knows that the child classes of `sealed class PersonState` are `Running`, `Walking`, and `Idle`. Therefore, you can receive only the necessary messages in a `when` expression without using an `else` branch.

Now, let's say we extend our app and add a `RunningFast` state.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FVA6FL%2Fbtrd2yjiK9F%2FIr9Mg8QNfZr1YwfOZLWf00%2Fimg.png)

If you check, you'll see an error indicating that the `RunningFast` state hasn't been added. The error disappears once you handle the branch for `RunningFast`.

### sealed class Inheritance

1. Inheriting with `class`.

We saw earlier that inheritance was done using the `class` keyword. However, sharp-eyed readers might have already noticed the strange yellow warning line.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2F3ftck%2Fbtrd5kSnFLd%2FzPhNXlKhVRTxduqzjfpgQ0%2Fimg.png)

> 'sealed' subclass has no state and no overridden 'equals()'
> Interpretation: The subclass of the sealed class has no state and does not override `equals()`.

This means you should only inherit using the `class` keyword if there are state variables or if `equals` is overridden. Otherwise, `object` should be used to save memory. Therefore, changing the code above as follows removes the warning.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FcFKrgq%2Fbtrd05a3Ybw%2FwPI5I1MLumEczD8jZzCjlk%2Fimg.png)

2. Inheriting with `object`.

An `object` follows the singleton pattern, being loaded into memory only once and reused. Therefore, if there's no state, creating an object more than once and loading it into memory can be considered a waste of memory. Thus, changing all values without state variables to `object` removes the warning.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FnvAXB%2Fbtrd2wZ6vSa%2FIMumPdG7HSfzcM0XTs1igK%2Fimg.png)

<br>

### sealed class Characteristics

Only child classes within the same package can inherit.

Because it would consume too many resources for the compiler to search all packages for child classes, **a `sealed class` restricts the declaration of its child classes to within the same package.**

If a `sealed class` exists within `com.example.demo.controller`, all inheriting classes must also reside within that package; otherwise, an error like the following will occur.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbfmNkU%2Fbtrd03j1moY%2FSOwlL7kQKMk3djjOLvBMlK%2Fimg.png)

> Inheritor of sealed class or interface declared in package com.example.demo.controller but it must be in package com.example.demo where base class is declared
>
> **Interpretation: A `sealed class` can only be inherited if the `Running` class is declared in `com.example.demo`.**

Also, direct object instantiation is not possible for abstract classes.

A `sealed class` is an `abstract class` and cannot be instantiated directly.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbdjeDu%2Fbtrd026sBQO%2FBEoAuelJGRN14tMUKeXpj1%2Fimg.png)

> Sealed types cannot be instantiated
> Interpretation: A `sealed class` cannot be instantiated.
