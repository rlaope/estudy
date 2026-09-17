# How ObjectMapper Works and Additional Features Provided by Spring Boot

## Serialization Using ObjectMapper

### How ObjectMapper's Serialization Works
ObjectMapper uses reflection to **create a JSON string from an object**, which is called serialization.

This process occurs when using `@RequestBody`, `@RestController`, or `ResponseEntity`, among others.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbwpRNc%2FbtrWI88C7BL%2FqUgPdSKFWnLIn30GOGBfY1%2Fimg.png)

In Spring, the `ObjectMapper` class from the Jackson module handles serialization by default. The `writeValueAsString` method of `ObjectMapper` is used in this process.

```java
String jsonResult = objectMapper.writeValueAsString(myDTO());
```

To create a JSON string from an object, the field values must be known. Only then can a string like the following be created.

```java
String message = "{\"name\":\"MangKyu\",\"age\":20}"
```

However, with `ObjectMapper`'s default settings, only public fields or public getters are accessible.

While additional settings can be configured in `ObjectMapper` to increase visibility, getters are almost always present, so the default settings are usually sufficient.

Therefore, when using `ObjectMapper`, it's generally best practice to always create getters for serialization.

## Caveats of Serialization Using ObjectMapper

Due to how `ObjectMapper` handles serialization, it can sometimes produce unintended JSON messages.

For example, let's say you create a method in a DTO that starts with `getX`, like this:

```java
@Getter
@NoArgsConstructor
@AllArgsConstructor
public class MangKyuRequest {

    private String name;
    private Integer age;

    public String getNameWithAge() {
        return name + "(" + age + ")";
    }

}
```

Serializing an object of the class above with `ObjectMapper` will produce the following JSON string:

```json
{"name":"MangKyu","age":20,"nameWithAge":"MangKyu(20)"}
```
This is because `getNameWithAge` also follows the method naming convention for getters (starting with `getX`). If you're not aware of this, it can lead to incorrect JSON responses.

Therefore, understanding how `ObjectMapper`'s deserialization works can be helpful.

## Deserialization Using ObjectMapper

### How ObjectMapper's Deserialization Works

ObjectMapper uses reflection to create an object from a JSON string, which is called deserialization.

In Spring, deserialization occurs when a JSON string is received as an object via `@RequestBody`.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fobjh4%2FbtrWGdpI6Sc%2Fqq8iu3TlqTWr3oA2yNJ3C0%2Fimg.png)

Deserialization is typically processed through the following steps:
1. Create an object using the default constructor.
2. Find and bind field values.

First, an object is created; if a default constructor is not present, an error is thrown.

After creating an object with the default constructor, field values must be found, which can typically be done via public fields or public getters/setters.

If the process fails, an exception will occur, so it's best to always create a default constructor and getter methods.

```
com.fasterxml.jackson.databind.exc.InvalidDefinitionException: Cannot construct instance of `com.mang.atdd.membership.objectmapper.MyDTO` (no Creators, like default constructor, exist): cannot deserialize from Object value (no delegate- or property-based Creator)
 at [Source: (String)"{"name":"MangKyu","age":20}"; line: 1, column: 2]

	at com.fasterxml.jackson.databind.exc.InvalidDefinitionException.from(InvalidDefinitionException.java:67)
	at com.fasterxml.jackson.databind.DeserializationContext.reportBadDefinition(DeserializationContext.java:1904)
	at com.fasterxml.jackson.databind.DatabindContext.reportBadDefinition(DatabindContext.java:400)
	at com.fasterxml.jackson.databind.DeserializationContext.handleMissingInstantiator(DeserializationContext.java:1349)
	at com.fasterxml.jackson.databind.deser.BeanDeserializerBase.deserializeFromObjectUsingNonDefault(BeanDeserializerBase.java:1415)
	at com.fasterxml.jackson.databind.deser.BeanDeserializer.deserializeFromObject(BeanDeserializer.java:352)
	at com.fasterxml.jackson.databind.deser.BeanDeserializer.deserialize(BeanDeserializer.java:185)
	at com.fasterxml.jackson.databind.deser.DefaultDeserializationContext.readRootValue(DefaultDeserializationContext.java:323)
	at com.fasterxml.jackson.databind.ObjectMapper._readMapAndClose(ObjectMapper.java:4674)
	at com.fasterxml.jackson.databind.ObjectMapper.readValue(ObjectMapper.java:3629)
	at com.fasterxml.jackson.databind.ObjectMapper.readValue(ObjectMapper.java:3597)
```

### Handling Deserialization Indirectly

If you need to create an object using an indirect method rather than a default constructor, two separate steps are required:
- Add the `ParameterNames` module to `ObjectMapper`
- Add the `-parameters` option to Java compilation

The Jackson module for `ObjectMapper` includes the `parameter-names` module, as follows:

```maven
 // <https://mvnrepository.com/artifact/com.fasterxml.jackson.module/jackson-module-parameter-names>
implementation group: 'com.fasterxml.jackson.module', name: 'jackson-module-parameter-names'
```

Registering this module with `ObjectMapper` allows `ObjectMapper` to use indirect methods.

For example, information based on parameter details, such as constructors with parameters, becomes usable.

```java
ObjectMapper objectMapper = new ObjectMapper();
objectMapper.registerModule(new ParameterNamesModule());
```

To leverage `ParameterNamesModule`, you need to be able to retrieve parameter information, which can be done by using Java compilation's `-parameters` option.

Adding the `-parameters` option to Java compilation (for JDK 8 and above) adds information to the compiled class, allowing parameter information to be retrieved via the reflection API at compile time.

For reference, IntelliJ's `-parameters` option can be added as shown below, and it requires a `Build > Rebuild Projects` to take effect.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FNIQH4%2FbtrVAK2moxo%2FQN0e138B0YiZOE0BT53zAK%2Fimg.png)

## Additional Features Provided by Spring Boot

However, when developing with Spring Boot on a Gradle-based project, you might experience objects being created successfully without errors, even when a default constructor is absent.

This is because Spring Boot provides additional configurations and plugins.

First, the `-parameters` option is handled by Spring Boot's Gradle Java plugin.

Using this plugin automatically adds the `-parameters` option to Java compilation.

```gradle
plugins {
    id 'org.springframework.boot' version '2.7.5'
    id 'io.spring.dependency-management' version '1.0.11.RELEASE'
    id 'java'
}
```

The addition of the `ParameterNames` module is handled during Jackson's AutoConfiguration.

Similar to other modules, if this module's dependency is present, the configuration is automatically added.

```java
@Configuration(proxyBeanMethods = false)
@ConditionalOnClass(ParameterNamesModule.class)
static class ParameterNamesModuleConfiguration {

    @Bean
    @ConditionalOnMissingBean
    ParameterNamesModule parameterNamesModule() {
        return new ParameterNamesModule(JsonCreator.Mode.DEFAULT);
    }
```

## Conclusion: DTO Classes Should Be As Follows

It is inefficient to develop while constantly keeping the details explained above in mind.

Furthermore, this only adds unnecessary cognitive load.

So, I've established a rule for myself: mindlessly attach the following code to DTOs.

This allows us to provide a consistent approach to DTOs without burden.

If `@ModelAttribute` usage is required, simply add `@Setter` without overthinking it.

```java
@Getter
@Builder
@NoArgsConstructor
@AllArgsConstructor
public class MangKyuRequest {

    private String name;
    private Integer age;

}
```
