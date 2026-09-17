# State Machine

A state machine is a discrete mathematical model where a system exists in one of a finite number of states and transitions to another state based on an input (event).

In other words, it's an abstract machine, where this machine has only one state at a time, referred to as the current state at any given moment.

A specific finite automaton (a discrete mathematical model) is defined by its transition states from the current state and the set of conditions that trigger these transitions.

> Transition: In quantum mechanics, the process where a particle moves from one stationary state to another with a certain probability. Synonymous with "전이 (轉移)".

It is used throughout computer science and can be seen as a modeling technique frequently employed in programming, communication protocols, and workflow engines.

- STATE: A single state where the current system resides.
- EVENT/INPUT: An external stimulus that triggers a state change.
- TRANSITION: The movement from one state to another.
- ACTION: An operation performed during a transition or upon entering/exiting a state.
- INITIAL STATE: The starting state.
- FINAL STATE: The ending state (may or may not exist).

Consider a traffic light, for example. A traffic light can be viewed as a Finite State Machine (FSM) where the number of states is finite.

For instance, green light, red light (and yellow light) are the states, and the event is the passage of time.

And as time passes, the action performed is the change from a red light to a green light (with a yellow light possibly in between).

<br>

## Finite, Infinite

Typically, state machines are categorized into finite state machines or infinite state machines, consisting of a finite or infinite number of states, respectively. However, in the real world, products are primarily built using finite state machines. Therefore, it's generally acceptable to refer to state machines as finite state machines.

Now that we've learned about FSMs, a question might arise: In programming, handling event statements with `if else` or `switch-case` seems to work perfectly fine and doesn't appear to be any different. So, what exactly is this state machine, and why would we use it if it only takes more time to implement? Why would a single object manage the state itself, and why are actions separated?

These are certainly valid questions to ponder, and it's more problematic *not* to think about them and just assume they're good. So, if you've had these thoughts, I commend you. (Generally, state machine libraries are difficult to use directly, and often you only adopt the structure, having to completely re-implement the internal logic from scratch. This is common, so most people implement them themselves.)

The primary reason for using state machines is stability, as a mathematical model inherently possesses stability by its very definition.

This is because it controls controllable variables to produce the desired output. While `if-else` or `switch-case` might not have design flaws, during implementation, there's a possibility of variable corruption or difficulty in handling unforeseen `else` conditions. The problem of variable corruption becomes even more severe in concurrent environments like multi-threading. (States also require handling for concurrency, but the scope that needs to be handled is reduced.)

Of course, if the requirements presuppose that the complement of the `else` conditions can be an empty set, then I believe using `if else` would be acceptable.

<br>

### init, idle, processing, error, recovery

The pattern common to models that can be defined as state machines is as in the title. Let's look at each one.

1.  init: This state performs actions to initialize the initial state.
2.  idle: After initialization is complete, it transitions to the idle state, waiting to detect external actions or events.
3.  processing: When an external input is detected, it transitions to the processing state and performs the appropriate processing actions.
4.  error: If an error is detected during processing, the operation is halted, and the error code is analyzed.
5.  recovery: Based on the analyzed error results, it performs recovery operations.

Once recovery is complete, it returns to idle, repeating the cycle of detection and transition.

Looking at the advantages of state machines in the example above, the flow can strictly follow the design, and even if a failure occurs, the possible cases can be accurately and simply identified based on the state. This means faster debugging.

Since transitions only occur through predefined events, the designer can take charge of designing all flows, and implementers can implement and understand it exactly as designed by simply referring to the design document. The design itself becomes simpler, and it is not susceptible to contamination from other external factors like multi-threading or processing environments. This is because the conditions for reaching a particular state are already defined by an event protocol, ensuring a stable execution environment.

The overall flow also becomes more organic and clear.

As the world develops and more problems need to be solved, the operations we need to implement in software become diverse and complex. While humans cannot detect every single case, the state machine pattern is one that can flexibly detect exceptional cases and enforce stable operation. I believe that through the state machine model, we can write more stable and extensible code, however.

I recommend considering whether the business requirements I am currently implementing truly necessitate a state machine.
