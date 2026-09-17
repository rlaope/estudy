# Monolithic Architecture

### Monolithic Architecture
- Refers to traditional architecture. All components of the software are integrated into a single project.
- In a monolithic architecture, all processes are tightly coupled and run as a single service.
- Therefore, if demand for one process of the application surges, the entire architecture must be scaled.
- As the codebase grows, adding or improving features in a monolithic application becomes more complex.

### Advantages
1. Reasonable for small-scale projects.
2. Easy to develop, build, deploy, and test.

### Disadvantages
1. Application startup time increases, and build and deployment times become longer.
2. Even for minor modifications, the entire application must be rebuilt and redeployed.
3. Maintenance is difficult due to the large amount of concentrated code.
4. An error in one part affects the entire system.
5. It's challenging to select appropriate technologies, languages, and frameworks for each feature.
6. Scale-out is not possible.

<br>

### Comparison with Microservice Architecture

![](./image/모놀리식.png)
