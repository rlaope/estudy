# Application Service and Presentation Layer

## Presentation Layer and Application Layer
> If the domain is not well-established, proper software cannot be built.

The application layer and presentation layer act as intermediaries connecting the user and the domain.

![](https://camo.githubusercontent.com/34206c1c47ef39f4caefcdc6f4d5b971752f3eb1d0ce6f1ff9e53e5041286855/68747470733a2f2f333535333234383434362d66696c65732e676974626f6f6b2e696f2f7e2f66696c65732f76302f622f676974626f6f6b2d6c65676163792d66696c65732f6f2f6173736574732532462d4d35484f53747876782d4a723066715a6879572532462d4d424b42484e3544334c326d4941386d6173382532462d4d424b4338454a6c4361364a623873795a5f58253246372e312e706e673f616c743d6d6564696126746f6b656e3d36663436666462632d383666312d343730622d386437642d636163376633376137376261)

- `Presentation Layer` interprets user requests.
- `Application Layer` provides the actual functionality desired by the user.

The presentation layer responds to the user with the results of the application service execution in an appropriate format.

Since the presentation layer handles user interaction, the application layer does not depend on the presentation layer.

<br>

## Role of Application Service
> The application service simply controls the flow between domain objects.

### Connecting Presentation Layer and Domain Layer
From the perspective of the presentation layer, the application service connects the domain layer and the presentation layer. If an application service is complex, it might be implementing part of the domain logic. This can negatively impact code quality through code duplication, logic dispersion, and so on.

### Transaction Handling
The application service must handle domain state changes using transactions. To maintain data consistency, the application service should be executed within a transaction scope.

## Avoid Putting Domain Logic in Application Services
The application service controls the execution flow between domain objects using aggregates and related repositories.

e.g., If implementing a password change feature in an application service, it would use the Member aggregate and related repositories like MemberRepository.

Checking if the existing password was entered correctly is core domain logic, so it should be implemented in the main aggregate, not the application service.

If domain logic is dispersed across the domain layer and application services, the following problems arise:

- Reduced code cohesion
If domain data and the domain logic that manipulates it are located in different layers, **multiple layers must be analyzed** to understand the domain logic.

- Logic duplication

It becomes more likely that multiple application services will implement the same domain logic.

While a separate helper class can be created to prevent code duplication, if the logic is implemented in the domain layer, the application service only needs to use that functionality.

Such problems ultimately make code changes difficult. A decrease in changeability, a critical competitive factor for software, reduces the software's value. Therefore, domain logic should be **gathered in the domain layer to reduce code duplication, increase cohesion, and enhance value.**

<br>

## Implementing Application Services
> Application services should not depend on the presentation layer and should be small.

Application services play a role similar to the Facade design pattern. There are several considerations when implementing application services.

### Size of Application Service
Application services are typically implemented in one of two ways:
- Implement all functionalities of a single domain in one application service class
- Implement separate application service classes for each distinct functionality

Implementing all functionalities related to a single domain in one class can eliminate code duplication.

However, if all related functionalities are in one class, less related code accumulates, reducing readability.

Once they start accumulating in one class, even situations requiring separation get forced into it.

This degrades code quality, so classes should be separated by functionality, even if it means more classes.

Additionally, if the same logic is implemented for each function, a helper class can be created to prevent code duplication.

### Application Service Interfaces and Classes
There is ongoing debate about whether interfaces are necessary when implementing application services. Interfaces are needed when there are multiple implementation classes. However, it is very rare for an application service to have multiple implementation classes, and there is almost no need to replace implementation objects at runtime.

Therefore, creating an interface is not a good choice unless absolutely necessary.

### TDD
When doing Test-Driven Development, development usually starts from the presentation layer.

In such cases, the application service cannot be implemented beforehand, so an interface for the application service must be created.

Conversely, if development starts with the domain layer or application layer rather than the presentation layer, the application service class is created first.

In such cases, even with Test-Driven Development, the **presentation layer can be tested without an interface using testing tools like Mockito**.

### Method Return Values
Methods provided by an application service should receive values necessary to implement requirements as parameters.

Each value can be passed as an individual parameter, or a separate data class can be created to pass them.

Returning the aggregate itself from the application service can simplify implementation, but it disperses domain logic execution across the application service and presentation layer, reducing cohesion.

It is better to **return only the necessary data** from the presentation layer to increase cohesion.

### Do Not Depend on the Presentation Layer
When determining the parameter types of an application service, types related to the presentation layer should not be used.

If a dependency on the presentation layer occurs, the coupling between the application service and the presentation layer increases.

Therefore, the implementation technology of the presentation layer should not be used for service method parameters and return types.

### Transaction Handling
Transaction handling can be easily done using Spring Framework's @Transactional.

## Presentation Layer
> The presentation layer connects the user and the application service.

The responsibilities of the presentation layer are broadly divided into three categories:
- Provides and controls the flow (screens) that allow users to use the system.
- Forwards user requests to the appropriate application service and provides the results to the user.
- Manages user sessions.

![](https://user-images.githubusercontent.com/43809168/99542710-b9734e00-29f5-11eb-9cdf-b40c85292e53.png)

The presentation layer provides the content requested by the user as a response, processes the presentation layer's requests using application services, and transmits the results.

## Value Validation
> For the completeness of the application service, it is good to perform all validations within the application service.

In principle, all value validations are handled by the application service.

If value validation is performed in the application service rather than the presentation layer, it provides a poor user experience.

Of course, the application service can collect a list of exceptions and return them to the presentation layer at once.

The presentation layer can also handle this simply using Spring Framework's Validation feature.

If the presentation layer checks for required values and formats, the application service only needs to check for logical errors.

- Presentation Layer -> Validates required values, value formats, ranges, etc.
- Application Service -> Validates logical errors such as data existence.

The author acknowledges the inconvenience of writing more code but performs both value validation and logical validation in the application service for its completeness.

## Authorization Check
> Authorization checks can be performed in the presentation, application, and domain layers.

Authorization checking itself is not a complex concept, but the complexity of authorization varies by system.

The Spring Security framework has a flexible and extensible structure that can meet various situations.

However, its flexibility also means complexity, requiring a thorough understanding.

Authorization checks can be performed in the following layers:
- Presentation Layer
- Application Service
- Domain

## Presentation Layer
Checks the user's authentication status.
- Before forwarding web requests to the controller that handles this URL, it checks the authentication status and only forwards authenticated user web requests to the controller.
- If the user is not authenticated, they are redirected to the login screen.

**Servlet filters create user authentication information and check authentication status**.

In addition to authentication status, **URL-based authorization checks** can be performed.

## Application Service
If access control cannot be done solely by URL, authorization checks must be performed at the method level of the application service. AOP can be used to perform authorization checks on service methods.

## Domain
If authorization checks need to be performed at the individual domain object level, implementation becomes complex.

Since authorization checks cannot be performed at the application service method level, the authorization logic must be implemented directly.

Authorization logic at the domain object level varies by domain, requiring a high level of understanding of the framework.

If there is insufficient understanding of the framework, it is better to implement authorization features tailored to the domain directly.

## Query-Only Features and Application Services
> It doesn't necessarily have to be Presentation -> Application -> Query-Only Feature.
If query-only features are used in an application service, the service code might simply end up calling the query-only feature.

In this case, no separate transaction is needed, and there is no additional logic, so there is no problem using the query-only feature directly from the presentation layer.
