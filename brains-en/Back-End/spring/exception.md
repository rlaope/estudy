# Spring Boot Exception Handling with @ExceptionHandler

### @ExceptionHandler
- @ExceptionHandler is a feature that catches errors occurring in the Controller layer and processes them with a method.
- It excludes errors occurring in the Service and Repository layers.

```java
@Controller
public class SimpleController {

    // ...

    @ExceptionHandler
    public ResponseEntity<String> handle(IOException ex) {
        // ...
    }
}
```

As shown, within a class declared with @Controller, you can handle errors that may occur inside a method using the @ExceptionHandler annotation.

### Handling Multiple Exceptions
You can pass the `value` of @ExceptionHandler to specify which Exception to handle. If `value` is not set, it will catch all Exceptions, so it's recommended to specify the Exception concretely.

```java
@Controller
public class SimpleController {

    // ...

    @ExceptionHandler({FileSystemException.class, RemoteException.class})
    public ResponseEntity<String> handle(Exception ex) {
        // ...
    }
}
```

- The method receives `Exception ex` as an argument, and specific Exceptions are set as the `value` of @ExceptionHandler.
- If you need to catch multiple Exceptions, it's recommended to explicitly specify them like `@ExceptionHandler({FileSystemException.class, RemoteException.class})` rather than using a broad `ExceptionHandler({IOException.class})`.

<br>

### @ControllerAdvice

#### Using @ExceptionHandler in @ControllerAdvice
- `@ControllerAdvice` catches all errors occurring in `@Controller` and `handler`.
- You can catch errors using `@ExceptionHandler` within `@ControllerAdvice`.

```java
@ControllerAdvice
public class ExceptionHandlers {

    @ExceptionHandler(FileNotFoundException.class)
    public ResponseEntity handleFileException() {
        return new ResponseEntity(HttpStatus.BAD_REQUEST);
    }


}
```

### Scope Configuration
Since @ControllerAdvice catches all errors, if you only want to handle some errors, you can configure it separately.

1. Annotations
2. basePackages
3. assignableTypes

```java
// 1.
@ControllerAdvice(annotations = RestController.class)
public class ExampleAdvice1 {}

// 2.
@ControllerAdvice("org.example.controllers")
public class ExampleAdvice2 {}

// 3.
@ControllerAdvice(assignableTypes = {ControllerInterface.class, AbstractController.class})
public class ExampleAdvice3 {}
```

- base packages: Specifies the package to scan, scanning `org.example.controllers` package and all its sub-packages.
- basePackagesClasses: Specifies the class to scan, starting from the package at the top of the class.

> Caution: Since configurators like annotations and base packages are executed at runtime, using too many configurators can degrade performance!

<br>

### RestControllerAdvice
- @RestControllerAdvice includes @ControllerAdvice and @ResponseBody.
- It functions like @Controller and can return objects via @ResponseBody.

```java
@Target(ElementType.TYPE)
@Retention(RetentionPolicy.RUNTIME)
@Documented
@ControllerAdvice
@ResponseBody
public @interface RestControllerAdvice {
	// ...	
}
```

### @ControllerAdvice vs @RestControllerAdvice
- @ControllerAdvice has the @Component annotation, so it is registered as a Spring Bean through component scanning.
- @RestControllerAdvice consists of @ControllerAdvice and @ResponseBody annotations, allowing it to return values as a ResponseBody rather than an HTML view.
