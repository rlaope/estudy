# Controller, Service, Repository

## What is a Controller?
- Corresponds to C in the MVC pattern, primarily processing user requests and then passing model objects to the specified view.
- In other words, it's the entry point for user requests, and it delegates the decision of how to handle a request to the Service. After that, the Service passes the actual processed content to the View.

<br>

### Why use a Controller?
Suppose there are services A, B, and C within a large-scale system. Instead of creating a single class to handle all these types of services, we create an intermediary controller called 'Controller'. If we design and code according to `roles` like this, where A-Controller handles service A and B-Controller handles service B, development and maintenance costs are significantly reduced, which is why Controllers are used.

<br>

### How to use Controller in Spring
In Spring, the annotations used to designate a controller are `@Controller` and `@RestController`.

<br>

1. @Controller(Spring MVC Controller)
The traditional Spring MVC controller, `@Controller`, is primarily used to return a View. However, when used with the `@ResponseBody` annotation, it performs the same function as a RestController.

```java
@Controller
public class UsingController {
    @GetMapping("/home") // If a GET request comes to /home
    public String homePage(){
        return "home.html"; // Creates home.html
    }
}
```

2. @RestController(Spring Restful Controller)Permalink
RestController has the effect of having the `@ResponseBody` annotation applied to a Controller.

Its main purpose is to return object data in JSON/XML format.
```java
@RestController // Declares that data will be exchanged in JSON
public class ProductRestController{

  private final ProductService productService;
  private final ProductRepository productRepository

  // Retrieve list of all registered products
  @GetMapping("/api/products")
  public List<Product> getProducts(){
    return productRepository.findAll();
  }
}
```

<br>

## What is a Service?
Let's look at the big picture to understand Service.
1. The Client sends a Request.
2. The appropriate Controller for the Request URL receives it. (@Controller, @RestController)
3. The Controller calls the Service to process the incoming request.
4. The Service processes the appropriate information and passes the data to the Controller.
5. The Controller delivers the Service's result to the Client.

The process by which the Service processes the appropriate information is called `performing business logic`.
The Service performs business logic and retrieves result values using a DAO that accesses the database.

<br>

### What is DAO?
- For simple projects that merely load pages and retrieve DB information all at once, it is said that there may be little difference between Service and DAO.
- Simply put, a DAO is an `object` that can access a MySQL server and execute SQL statements.

<br>

### DAO and JPA
Spring Data JPA allows you to implement DAOs with very little code. That is, simply by creating an interface, it enables you to perform insert, update, delete, and select operations on an Entity (`@Entity`) class. Furthermore, just by declaring methods in the interface, it performs tasks equivalent to creating code that executes lightweight queries.

One might then think that if using JPA, DAOs don't need to be implemented directly. However, the code that JPA can generate replaces very lightweight and simple queries, making it very difficult to use when complexity increases.

In such cases, if only JPA is used, performance might be worse than developing directly with SQL.
Therefore, one either studies JPA deeply to write complex queries with JPA, or uses DAO together for highly complex parts.
![dao](./image/dao.png)

<br>

## What is a Repository?
- It is an interface for using methods to access the database created by an Entity.
- If you have created a DB structure with the `@Entity` annotation, you need to perform CRUD operations on it. You can think of it as the layer that defines how this will be done.

<br>

### Conclusion
![spring flow](./image/springdto.png)
