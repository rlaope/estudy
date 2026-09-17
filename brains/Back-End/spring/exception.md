# Spring boot @ExceptionHandler를 통한 예외처리

### @ExceptionHandler 
- @ExceptionHandler는 Controller계층에서 발생하는 에러를 잡아서 메서드로 처리해주는 기능이다. 
- Service, Repository에서 발생하는 에러는 제외한다.

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

이렇게 @Controller로 선언된 클래스 안에서 @ExceptionHandler 어노테이션으로 메서드 안에서 발생할 수 있는 에러를 처리할 수 있다.

### 여러개의 Exception 처리
@ExceptionHandler의 value값으로 어떤 Exception을 처리할 것인지 넘겨줄 수 있는데, value를 설정하지 않으면 모든 Exception을 잡게 되기 때문에 Exception을 구체적으로 적어주는 것이 좋다고 한다.

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

- 메서드의 인자로 Exception ex를 받고 있고 @ExceptionHandler의 value값으로 특정 Exception들을 설정해주고 있다.  
- 여러개의 Exception을 잡아야한다면, ExceptionHandler({IOException.class})처럼 포괄적인게 아닌 @ExceptionHandle({FileSystemException.class, RemoteException.class})로 구체적으로 명시해주는 것을 권장한다고 한다.

<br>

### @ControllerAdvice

#### @ControllerAdvice에서 @ExceptionHandler 사용
- `@ControllerAdvice`는 `@Controller`와 `handler`에서 발생하는 에러들을 모두 잡아준다.
- `@ControllerAdvice`안에서 `@ExceptionHandler`를 사용하여 에러를 잡을 수 있다.

```java
@ControllerAdvice
public class ExceptionHandlers {

    @ExceptionHandler(FileNotFoundException.class)
    public ResponseEntity handleFileException() {
        return new ResponseEntity(HttpStatus.BAD_REQUEST);
    }


}
```


### 범위 설정
@ControllerAdvice는 모든 에러를 잡아주기 때문에 일부 에러만 처리하고 싶을 경우에는 따로 설정을 해주면 된다.

1. 어노테이션
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

- base packages : 탐색 패키지 지정, org.example.controllers 패키지, 하위 패키지까지 모두 탐색
- basePackagesClasses : 탐색 클래스 지정, 클래스의 맨 위에 있는 package 부터 시작


> 주의 : 어노테이션, 베이스패키지 등 설정자들은 runtime시 수행되기 때문에 너무 많은 설정자들을 사용하면 성능이 떨어질 수 있다!



<br>

### RestControllerAdvice
- @ResControllerAdvice는 @ControllerAdvice와 @ResponseBody을 가지고 있다.
- @Controller처럼 작동하며 @ResponseBody를 통해 객체를 리턴할 수 있다.

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
- @ControllerAdvice는 @Component 어노테이션을 가지고 있어 컴포넌트 스캔을 통해 스프링빈으로 등록된다.
- @RestControllerAdvice는 @ControllerAdvice와 @ResponseBody 어노테이션으로 이루어져있고 HTML 뷰 보다는 ResponseBody로 값을 리턴할 수 있다.

