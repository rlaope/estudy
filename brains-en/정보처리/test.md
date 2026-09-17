# Testing

## What is Testing?
It is an activity that verifies the functions, performance, usability, and safety required by users of a developed application or system, and uncovers hidden defects that are not apparent.

- The roles required in the testing process are software architect and test manager.
  - The software life cycle proceeds in the order of requirements, analysis, design, implementation, or development, and may be performed iteratively depending on the project's characteristics and methodology.
  - Testing consists of `unit testing`, `integration testing`, `system testing`, and `acceptance testing` in that order.

> ### Software Architecture
> - The basic structure that forms the backbone of software
> - A system structure or construct that expresses the organic relationships between components.

<br>

## Seven Principles of Testing
1. Testing starts from the planning phase.
   - Testing activities should begin as early as possible in the software development life cycle.

2. Testing is an activity to uncover defects.
   - The purpose of testing is not to remove defects, but to find them.
   - Testing can show that defects exist, but it `cannot prove` that there are no defects.

3. Complete testing is impossible.
   - Testing everything (input values, paths, timings) is impossible due to resource limitations.

4. Testing is context-dependent.
   - Even in application testing, abnormal defect detection can occur for the same test, so to prevent this phenomenon, it is necessary to test in various ways.

5. Consider defect clustering.
   - Most defects tend to concentrate in a small number of specific modules.
   - 80% of defects are concentrated in 20% of the code. In other words, resources are concentrated where defects are high. `(Pareto Principle)`

6. Consider the pesticide paradox.
   - This refers to the phenomenon of immunity where repeated testing with the same test cases fails to find new bugs.

7. Consider the fallacy of absence of errors.
   - If the developed product does not meet user needs and expectations and is useless, then the activity of finding defects is meaningless.
   - The developed product must conform to requirements and be fit for use.


<br>

## Test Classification by Project Execution Phase

### 1. Unit Testing
- This involves testing small software units (components or modules), typically performed by the developers themselves.
- In the past, unit testing was omitted due to time constraints, but recently, with the advancement of development tools, it is automatically performed during the development process.
- Unit testing is a very important part and must be performed even if not supported by development tools.
- It includes structural testing, functional testing, resource-related testing, robustness testing, and other specific non-functional tests.
- It uses component specifications, detailed software design, data model specifications, etc., for testing.

Test Method | Description | Test Purpose
---|---|---
Structure-based | Its purpose is to test the results based on control flow and condition decisions for each business unit. | Control flow, condition decision
Specification-based| Its purpose is to verify user inputs, outputs, and internal events for equivalence partitioning and boundary value analysis. | Equivalence partitioning, boundary value analysis


> White-box Testing and Black-box Testing
> - White-box Testing: Structure and behavior-based testing from a developer's perspective
> `Types`: Basis path testing, control flow testing, condition testing, loop testing, data flow testing, branch testing. (`Structure-based`)
>
> - Black-box Testing: User perspective, specification-based testing
> `Types`: Equivalence partitioning, boundary value testing, cause-effect graph testing, comparison testing (`Specification-based`)

<br>

### Integration Testing
- Tests the interfaces between modules and the interactions between integrated components.
- In some cases, partial integration testing is performed when a single process is completed.
- Generally, it is based on sequential forms and an understanding of the architecture rather than the Big Bang approach.
- There are tests such as Big Bang, Bottom-up, Top-down, Sandwich, Central, Collaboration, and Layer integration.

1. Big Bang  
   - `Execution Method` : Integrate all modules simultaneously and then execute  
   - `Dummy Modules` : None  
   - `Advantages` : Quick testing possible, advantageous for small systems  
   - `Disadvantages` : Difficult to pinpoint defect locations, requires all modules to be developed

<br>

2. Bottom-up
   - `Execution Method` : Progressively execute from the lowest-level modules upwards with higher-level modules
   - `Dummy Modules` : Drivers required
   - `Advantages` : Easy to pinpoint defect locations, no wasted module development time
   - `Disadvantages` : Early prototyping is difficult, critical modules are likely to be tested last

<br>

3. Top-down
   - `Execution Method` : Integrate and execute from the highest-level modules downwards with lower-level modules
   - `Dummy Modules` : Stubs required
   - `Advantages` : Easy to pinpoint defect locations, early prototyping possible, critical modules can be tested first, early defect detection possible.
   - `Disadvantages` : Many stubs required, insufficient testing of lower-level modules

> Driver : Acts as an interface between non-existent higher-level modules in bottom-up testing.
> Stub : An easy-to-write test module for top-down testing.

<br>

### System Testing
- This tests whether the functions of integrated unit systems perform normally within the system, including performance and fault testing.
- System testing relates to the behavior of the entire system as defined at the development project level.
- To minimize environment-specific fault-related risks, it tests whether system performance and related functional/non-functional customer requirements are perfectly met, similar to the actual end-user environment.
- It uses requirements specifications, business procedures, use cases, risk analysis results, etc.

> ### Use Case
> - A scenario expressing system behavior from a user's perspective
> - The process of eliciting requirements related to the system

- It is divided into business-based functional requirements and system-based non-functional requirements.
  1. `Functional Requirements` : Specification-based black-box testing using requirements specifications, business procedures, use cases, etc.
  2. `Non-functional Requirements` : White-box testing for structural elements such as performance testing, recovery testing, security testing, internal system menu structure, and web page navigation.

<br>

### Acceptance Testing
- Generally, this testing is performed by end-users and business stakeholders to decide whether the developed product is ready for operation, and it is conducted **before actual business deployment**.
- It verifies some parts of the system or specific non-functional characteristics.

1. `User Acceptance Testing` : Business users verify the suitability of system use.
2. `Operational Acceptance Testing` : Testing activities performed by system administrators during system acceptance, verifying backup/restore systems, disaster recovery, user management, regular checks, etc.
3. `Contract Acceptance Testing` : Verifies compliance with contractual acceptance/inspection conditions.
4. `Regulatory Acceptance Testing` : Verifies development according to regulations such as government guidelines, laws, and rules.
5. `Alpha Testing` : Testing performed by potential customers within the developing organization.
6. `Beta Testing` : Testing performed by customers in a real environment.
