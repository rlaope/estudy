# Difference between Mock and Stub Tests

## Preliminary Concepts

### Dummy
Dummy objects are passed but not used, typically serving only to fill parameter lists.

### Fake
Fake objects have a working implementation but typically use some shortcuts that make them unsuitable for production.

A representative example is an in-memory database.

### Stub
Stubs provide pre-programmed answers to calls made during a test and typically do not respond at all beyond what they are programmed for the test.

### Spy
A Spy is a Stub that records some information based on how it was called.

### Mock
A Mock is an object pre-programmed with expected expectations.

## Test Double

Before understanding the above concepts, one must first understand the concept of a test double.

![](https://user-images.githubusercontent.com/42582516/155876237-36a220bd-8e0f-4f1c-b0be-6a4a012fb3d2.png)

Test Double Types

A test double refers to an object that can replace another object when the object being tested is difficult to use due to complex relationships with other objects.

Test doubles are categorized into Dummy, Stub, Spy, Mock, and Fake.

## Mock vs Stub
The two most commonly used concepts can be summarized as follows:

> According to the principles of testing, a single test can have multiple stubs, but typically only one mock.

## Stub
An object that appears to function like a real one by using an instantiated and implemented fake object (Dummy, no functional implementation).

It implements the interface or class minimally.

It responds to requests made during the test with pre-prepared answers.

It does not respond beyond what it is programmed for the test.

If a specific part of a collaborating object is difficult to test, a stub can be used to facilitate testing.

### Stub's Lifecycle
- Setup, prepare for test
- Exercise, test
- Verify state, verify state
- Teardown, clean up resources

<br>

## Mock
An object programmed to specify expectations for calls and behave according to those expectations.

It is an object created to replace other objects that are intertwined with the code being tested when setting up the test environment is difficult.

It performs behavior verification.

### Mock's Lifecycle

- Setup data, prepare data
- Setup expectations, prepare expected results
- Exercise, test
- Verify expectations, verify expectations
- Verify state, verify state
- Teardown, clean up resources

## Difference between Stub and Mock
Other doubles, including stubs, use `state verification`, while Mock objects use `behavior verification`.

> State verification: A verification method that checks if a method has operated correctly by examining the object's state after its execution.
> Behavior verification: A verification method that checks if a specific action is performed when it cannot be determined by the method's return value.

The key takeaway is that the target of verification is different.

### State Verification Example
```java
StateClass stateClass = new StateClass();
stateClass.doSomething()

assertThat(stateClass.getStatus()).isEqualTo(true);
```

### Behavior Verification Example
```java
BehaviorClass behaviorClass = new BehaviorClass();

verify(behaviorClass).doBehavior();
```

## A More Detailed Example

### Stub
Easy to use and no additional dependencies.

```java
public class SimpleService implements Service {

    private Collaborator collaborator;

    public void setCollaborator(Collaborator collaborator){
        this.collaborator = collaborator;
    }

    // part of Service interface

    public boolean isActive(){
        return collaborator.isActive()
    }
}
```

```java
public void testActiveWhenCollaboratorIsActive() throws Exception {

    service.setController(new Collaborator(){
        public boolean isActive(){
            return true;
        }
    });

    assertTrue(service.isActive());
}
```

### Mock
```java
Collaborator collaborator = EasyMock.createMock(Collaborator.class);
EasyMock.expect(collaborator.isActive()).andReturn(true);
EasyMock.replay(collaborator);

service.setCollaborator(collaborator);
assertTrue(service.isActive());

EasyMock.verify(collaborator);
```

## When to Use Stub and Mock?

Basically, when deemed appropriate (obviously).

Behavior verification (Mock) is dependent on the implementation because it verifies calls to specific methods, etc.

State verification (Stub) may require many additional methods to expose the state.

In many cases, state verification is often better.

However, there are cases where state verification is difficult, and in such situations, behavior verification or conducting a full test can be good alternatives.
