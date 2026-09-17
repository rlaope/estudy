# Lombok: A Collection of @Annotations You Must Know

### Lombok
One of the Java libraries that makes long source code concise using annotations.

### Types of Lombok Annotations

#### @NotNull
Specifies that a field's value cannot be null.

#### @Getter
Generates a getter method for the field.
@Getter(lazy=true): Calculates the value once on the first getter call, then caches and uses that value (to use it only once on the first call, rather than every time the getter is called). AccessLevel can be specified.

#### @Setter
Generates a setter method for the field.
AccessLevel can be specified.

#### @NoArgsConstructor
Generates a default constructor with no parameters.

#### @RequiredArgsConstructor
Generates a constructor for uninitialized `final fields` and fields annotated with @NotNull.

#### @AllArgsConstructor
Generates a constructor with all fields.

#### ToString
Generates a toString() method.
Removes unnecessary properties using the `exclude` attribute, e.g., @ToString(exclude ="value").

#### @EqualsAndHashCode
Generates equals() and hashCode() methods.
The `exclude` attribute can be used.

#### @Data
@Getter + @Setter + @RequiredArgsConstructor + @ToString + @EqualsAndHashCode

#### @Value
An annotation that signifies immutability.
Member fields annotated with @Value become constants with a private access modifier and `final` (since they are `final`, setters cannot exist).

#### @Log
Can automatically generate a Logger.
Automatically creates a `log` field and assigns a logger object with the class name to it.

#### @Builder
Automatically generates a Builder.
When using the @Singular annotation, elements can be added one by one.

<br>

### @Builder + @Singular Example
```java
// 선언
@Builder
public class Movie {
	private String title;
    @Singular
    private List<String> actress;
}

// 사용 예제
// movie(title="삼진그룹 영어토익반", actress= ["고아성", "이솜", "박혜수"])
Movie movie = Movie.builder()
				.title("삼진그룹 영어토익반")
                .actress("고아성")
                .actress("이솜")
                .actress("박혜수")
                .build();
```

<br>

`toString()`: A method that converts an object's values into a string and returns it.
`equals()`: An equality comparison operator (compares whether the contents of two objects are the same).
`hashCode()`: An identity comparison operator (compares whether two objects are the same).
