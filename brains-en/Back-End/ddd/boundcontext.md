# Domain Model and Bounded Context

## Domain Model and Boundaries

> Each sub-domain should have a different model.

A single domain is divided into multiple sub-domains, so you should not try to represent all sub-domains with a single model.

Even if they appear to be logically the same entity, different sub-domains may use different terminology.

![](https://user-images.githubusercontent.com/43809168/98814430-00e36280-2469-11eb-828b-043450bc712.png)

- Each model should have explicitly defined boundaries to prevent them from mixing.
- A model has complete meaning only within a specific context.
- In DDD, a context with distinct boundaries is called a **Bounded Context**.

## Bounded Context
> A Bounded Context is a boundary that distinguishes a domain model.

A Bounded Context determines the boundaries of a model, and a single Bounded Context logically contains a single model.
Furthermore, the domain model implements the domain within the Bounded Context, which is the physical system that actually provides functionality to users.
A Bounded Context can also be determined by the team organizational structure of a company.

![](https://user-images.githubusercontent.com/43809168/98814818-93840180-2469-11eb-9c42-8bd99bc42a0e8.png)

When developing multiple sub-domains within a single Bounded Context, care must be taken to prevent the models of the sub-domains from mixing.
Even if a single Bounded Context includes multiple sub-domains, it should be implemented such that each sub-domain has its own distinct package.

![](https://user-images.githubusercontent.com/43809168/98815005-d9d96080-2469-11eb-9a9b-8086ddeca016.png)
Since a Bounded Context forms the boundary that distinguishes domain models, it is implemented to include models appropriate for its sub-domain.

![](https://user-images.githubusercontent.com/43809168/98815024-e2319b80-2469-11eb-88ae-8908132d059f.png)

<br>

## Bounded Context Implementation
> Not all Bounded Contexts need to be developed in a domain-driven manner.

A Bounded Context includes not only the domain model but also the presentation layer, application services, and infrastructure layer.

![](https://user-images.githubusercontent.com/42582516/159478300-47648192-ea1c-46d2-9635-a954e8b1fa9b.png)

Not all Bounded Contexts need to be developed in a domain-driven manner.
If the domain functionality is not complex and is simple, using a service-DAO based CRUD approach is not a problem.

![](https://user-images.githubusercontent.com/42582516/159478979-68ad9157-0a92-4f51-b218-1d615c471009.png)

It is also possible to use a hybrid of both approaches within a single Bounded Context.
A prime example is CQRS (Command Query Responsibility Segregation), a pattern that separates models for command functionality and query functionality.
As shown below, functionalities related to state changes can be implemented based on a domain model, while query functionalities can be implemented using service-DAO.
![](https://user-images.githubusercontent.com/42582516/159479376-500e3456-298e-4b53-82d4-af3e9b8c48cb.png)

Each Bounded Context can use different implementation technologies and does not necessarily need to have a UI visible to the user.
You can directly call REST APIs from a web browser to process JSON data, or you can have a UI server that communicates with the Bounded Context.

## Integration Between Bounded Contexts
> Related Bounded Contexts can be integrated via REST API or messaging.

By introducing personalized recommendation features to the catalog sub-domain, there is a Bounded Context for the existing catalog and another for recommendation features.

![](https://user-images.githubusercontent.com/42582516/159481370-4d50619f-0609-45f0-9846-9a4508659a2f.png)

As two teams develop related Bounded Contexts, integration between the two Bounded Contexts naturally occurs.
Integration between Bounded Contexts is necessary in situations like the following:

- When a user views a product detail page, a list of similar products to the one being viewed is displayed at the bottom.

When a user requests a list of recommended products from the Catalog Bounded Context, the Catalog Bounded Context reads recommendation information from the Recommendation Bounded Context and provides the list of recommended products.

The catalog system receives recommendation data from the recommendation system, but the catalog system must represent recommended products using its catalog domain model.

![](https://user-images.githubusercontent.com/43809168/98815536-a0552500-246a-11eb-894e-ec55004f2e2a.png)

As shown below, the implementation, using a domain service based on the catalog's model, resides in the infrastructure layer.
The implementation class handles integration with external systems and is responsible for converting between the external system's model and the current domain model.
If the conversion process between the two models is complex, it can be handled in a separate class as follows.

![](https://user-images.githubusercontent.com/43809168/98815760-fd50db00-246a-11eb-8d34-2941f5908d97.png)

Calling a REST API is a direct integration method between two Bounded Contexts.
There is also an indirect integration method using a message queue, as shown below.

![](https://user-images.githubusercontent.com/43809168/98815854-1eb1c700-246b-11eb-9762-7f66c365156b.png)

### Microservices and Bounded Contexts
Microservices are structured by dividing into small services, running each service as an independent process, and having each service or REST API communicate using messaging.
Implementing Bounded Contexts, which form the boundaries of models, as microservices naturally separates models by context.
Separating models at the code level prevents the models of the two Bounded Contexts from mixing.

- Since Bounded Contexts form the boundaries of models, integration between boundaries occurs.

- When integration between boundaries occurs, communication can happen via REST API or asynchronously using a message queue.

- The data structure used when employing a message queue depends on which Bounded Context provides it.

- If provided by the catalog domain, it follows a pub/sub model.

- Conversely, if provided by the recommendation domain, it is not different from a REST API, except for being asynchronous.

## Boundaries Between Bounded Contexts
> Relationships between Bounded Contexts can be expressed in various ways.

### Customer - Supplier
Among the relationships between Bounded Contexts, the most common is one where one side provides an API and the other side calls that API.
A Bounded Context that uses an API becomes dependent on the Bounded Context that provides the API.

![](https://user-images.githubusercontent.com/43809168/98816310-d3e47f00-246b-11eb-89d7-573b17a2b2e7.png)

The downstream component, the Catalog Context, depends on the data and functionality provided by the upstream component, the Recommendation Context.
The upstream component acts as a service provider, while the downstream component acts as a customer using that service.
The upstream component defines communication protocols that downstream components can use and provides services using REST APIs or protocol buffers.

### Open Host Service

If there are multiple downstream components, the upstream component creates an API that can accommodate various requirements and exposes it as a service to maintain service consistency, which is called an **Open Host Service (OHS)**.

![](https://user-images.githubusercontent.com/43809168/98816561-32116200-246c-11eb-96a5-d7c8bab3710b.png)

### Anti-Corruption Layer
The services of the upstream component follow the domain model of the upstream Bounded Context.
The downstream component establishes an **Anti-Corruption Layer (ACL)** to prevent the external system's domain model from encroaching on its own domain model.
This layer handles model transformations between the two Bounded Contexts, allowing it to maintain its own domain model.

### Shared Kernel
When two Bounded Contexts share the same model to prevent redundant design, the model shared by the two teams is called a **Shared Kernel (SK)**.
However, because two teams share one model, they must maintain a close relationship.

### Separate Way
A **Separate Way (SW)** relationship is one where Bounded Contexts do not integrate with each other.
Since there is no integration between the two Bounded Contexts, they develop their models independently.
In a Separate Way, integration between two Bounded Contexts is done manually.

![](https://user-images.githubusercontent.com/43809168/98816957-c4196a80-246c-11eb-9be3-621b08dfdc5f.png)

## Context Map
> A Context Map shows the level of understanding of the entire system.

When engrossed in individual Bounded Contexts, one can sometimes lose sight of the whole.
A **Context Map** is used to display the relationships between Bounded Contexts, allowing for an overview of the entire business.

![](https://user-images.githubusercontent.com/42582516/159495223-144345d7-8653-4bc4-8baf-7bd9681c6c45.png)

If key aggregates are also displayed within the Bounded Context areas, the relationships within the model become clearer.
