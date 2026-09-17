# What is TDD (Test-Driven Development)

### TDD
- Test Driven Development
  - Test-Driven Development: **Tests drive development.**

### TDD Concept at the Concrete Behavioral Level
It means creating tests first and then writing code to pass those tests. In other words, during the development process, you first write a test, then create code that passes it, and repeat this cycle to actively receive feedback on whether it's working correctly.

- Usually, when developing software, testing is done after all the coding is finished.
  - After coding is finished: When a developer believes the coding is complete.
  - Applying TDD means reversing this order.
- Example of applying TDD
  - For instance, a program that outputs the current age (output) when a birth date (input) is entered.
  - The initial goal is simplicity (entering the birth year and current year).
    - 2015, 2018 -> (actual) 3 years old. The idea is to build this first.
    - Before building, design what to test after it's built.
    - Create a test program (code to test the program to be built) that outputs 2 when 2015, 2018 are input.
    - Then, create a program that passes that test.
      - Current year - Birth year
      - 2018 ~ 2015
    - Run the program with the test program (code corresponding to 3.).
    - If it passes, add a new test.
      - This time, a program that calculates the age when the birth month is added.
    - Continue with the above process.

<br>

### Core TDD Concept at an Abstract Level (Important)
It can be described as `an awareness of the gap between decision and feedback`, and furthermore, `a technique to adjust the gap between decision and feedback`.

- Kent Beck (creator of Extreme Programming)
  - What is TDD?
    - Awareness of the gap between decision and feedback
    - A technique to adjust the gap between decision and feedback
  - TDD can be seen as more psychological than a programming technique or technical skill.
- Decision
  - When programming, you make decisions like 'I'll use this method' or 'I'll use this for this part.'
- Feedback
  - When programming, you receive feedback in the form of success/failure (errors).
- A gap arises between these two (decision and feedback).
  - The larger the gap, the bigger the problem.
  - If I'm unaware of that gap, it's an even bigger problem.
  - In other words, explaining with the example above:
    - Decision: When writing code for goal 1, you decide, 'I'll calculate the age by subtraction.'
    - Feedback: You receive program-level feedback (it works/doesn't work) as a result of running the test program with the subtraction calculation code.
    - If you are aware of the gap between these two, you are doing TDD.

<br>

### Effects of TDD

#### Why should we do TDD?
**When uncertainty is high, `feedback` and `collaboration` are important.**

- Reasons why feedback and collaboration are important
  - When uncertainty is high, using feedback and collaboration increases the probability of better outcomes.
  - TDD also enhances feedback and collaboration, making it helpful when uncertainty is high.

#### In what situations should TDD be used?
- If you've coded a particular part many times and the outcome is obvious, you don't need TDD.
- Also, if the benefits of TDD are minimal, you don't need TDD.
- So, in what situations should TDD be used?
  1. A programming topic you're trying for the first time
    - When your own uncertainty is high.
  2. Projects where customer requirements may change
    - When external uncertainty is high.
  3. When you anticipate needing to change the code frequently during development.
  4. When you don't know who will maintain the code after you develop it.
- In short, TDD should be used when uncertainty is high.

<br>

### TDD Benefits
All Agile practices simultaneously enhance feedback and collaboration.

1. Feedback
  - TDD increases feedback.
    - You can frequently check if things are working well by passing tests.
    - This is something people can easily feel.

2. Collaboration
  - You can show test code to others, and they can run that code directly.
  - Sharing enhances collaboration.
    - Faster understanding of code written by others.
    - Easier understanding of code written by others.
    - It builds confidence.

<br>

### TDD Pros and Cons

#### Pros

1. Produces more robust object-oriented code

TDD explicitly guarantees code reusability, leading to thorough modularization by function during software development.

This enables the development of software composed of modules with low coupling and dependencies, ensuring that adding or removing modules does not affect the overall software structure.

2. Reduced redesign time
Because test code is written first, developers clearly define what needs to be done before starting development. Additionally, writing test scenarios allows for consideration of various edge cases. This prevents overall software design changes during development.

3. Reduced debugging time

This is also an advantage of unit testing. For example, if user data is incorrect, you would typically have to debug all layers—database, business layer, UI—to find the problem. However, TDD, by presupposing automated unit testing, makes it easy to pinpoint specific bugs.

4. Can replace test documentation
   
In SI projects, test definition documents are often created to specify what elements have been tested. These are merely simple integration test documents. However, TDD automates testing and simultaneously produces more accurate test evidence.

5. Ease of adding new features

When adding a new feature to completed software, the biggest concern is not knowing how it will affect existing code. However, TDD, by presupposing automated unit testing, can dramatically shorten the testing period.

> Despite these advantages of TDD, not everyone follows this development process. Why?

#### Cons

**The biggest drawback is reduced productivity.**
- Many people believe development speed slows down, leading to skepticism about TDD.
- This is because you have to write two sets of code from the start and continuously fix things while testing.
- TDD development time typically increases by approximately 10% to 30% compared to conventional development methods.
- In SI projects, meeting deadlines is far more important than software quality, so TDD is not commonly used.

<br>

### Reasons Why TDD is Difficult

#### You have to significantly change your established development methods.

The more ingrained your habits, the harder they are to change.

Conversely, it's easier for those with less development experience to adopt.

#### There's an image (framework) of how TDD "should" be done.
- People think they absolutely must use tools (unit test frameworks) for development.
- Being constrained by such rules is not an Agile approach.
- Ultimately, people get stuck in rules and copy & paste the same tests.
- TDD becomes difficult because of an obsession with tools/rules.

<br>

### How to Get Good at TDD

**You must continuously upgrade your working methods.**
For example, when developing a game and testing stage 3,
you always have to clear stage 1 and 2 before testing.
-> Increased testing cost

> At times like these, you should consider how to reduce costs.

-> Make it possible to go directly to stage 3.
You can get feedback more cheaply and frequently.
Back Door approach: Applying parameters during testing to go to the desired starting point of the system.
> In other words, you can improve by automating repetitive efforts.
