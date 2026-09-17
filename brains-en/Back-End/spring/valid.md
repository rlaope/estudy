# Validation using Spring Validation

### Validation
- Data validation is applied across multiple layers to filter out incorrect data and maintain security.
- Client data is easy to manipulate, and not all data comes in through normal channels. Therefore, data validation is necessary not only on the `Client Side` but also on the `Server Side`.
- In Spring Boot projects, validation can be performed using @Validated.

### Bean Validation
Bean validation, which is Spring's basic validation, is a check structured by applying specific annotations to class fields to define the constraints those fields must satisfy. The validator checks the validity of the fields of the object itself, created from that class, rather than validating any business logic.

<br>

#### Difference between @Valid and @Validated
@Valid is an annotation supported by Java, and @Validated is an annotation supported by Spring. @Validated includes the functionality of @Valid and additionally provides the ability to specify validation groups.

<br>

### @NotNull, @NotEmpty, @NotBlank
- I simply know that they mean not null, not empty, and not blank.
- So, for a String that must not be null, not empty, and not blank, should all three annotations be applied?

#### @NotNull
For constrained CharSequence, Collection, Map, and Array, it is valid as long as it is not null, but it can be empty.

#### @NotEmpty
For constrained CharSequence, Collection, Map, and Array, it must not be null and its size or length must be greater than 0.

#### @NotBlank
For String, it must not be null and its trimmed length must be greater than 0.
