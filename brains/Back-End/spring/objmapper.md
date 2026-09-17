# ObjectMapper의 동작 방식과 Spring boot가 제공하는 추가 기능들

## ObjectMapper를 이용한 직렬화 (Serialize)

### ObjectMapper의 직렬화 동작 방식
ObjectMapper는 리플렉션을 활용해서 **객체로부터 Json 형태의 문자열을 만들어내는데**, 이것을 직렬화라고 한다.  
  
해당 부분은 @RequestBody, @RestController 또는 ResponseEntity등을 사용하는 경우에 처리된다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbwpRNc%2FbtrWI88C7BL%2FqUgPdSKFWnLIn30GOGBfY1%2Fimg.png)

Spring에서는 기본적으로 jackson 모듈의 ObjectMapper라는 클래스가 직렬화를 처리한다. 그리고 그 과정에서 ObjectMapper의 writeValueAsString이라는 메서드가 사용된다.

```java
String jsonResult = objectMapper.writeValueAsString(myDTO());
```

  
객체로부터 Json 문자열을 만들기 위해서는 필드 값을 알아야 한다. 그래야만 다음과 같은 문자열을 만들 수 있다.

```java
String message = "{\"name\":\"MangKyu\",\"age\":20}"
```


하지만 ObjectMapper의 기본 설정으로는 public 필드 또는 public 형태의 getter만 접근이 가능하다.  
  
물론 ObjectMapper에 추가 설정을 통해 가시성을 높여줄 수 있지만 getter가 없는 경우는 거의 없으므로 기본 설정을 사용해도 충분하다.  
  
그러므로 ObjectMapper를 이용하는 경우, 직렬화를 위해 기본적으로 getter를 반드시 만들어두는 것이 좋다.

## ObjectMapper를 이용한 직렬화의 주의 사항

이러한 ObjectMapper를 이용한 직렬화 동작 방식 때문에 의도치 않은 json 메시지를 만들어 내기도 한다.  
  
예를 들어 DTO에 다음과 같이 getX로 시작하는 메서드를 만들었다고 하자

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

위와 같은 클래스의 객체를 ObjectMapper로 직렬화하면 다음과 같은 json 문자열이 만들어진다.  
  
```json
{"name":"MangKyu","age":20,"nameWithAge":"MangKyu(20)"}
```
getNameWithAge 역시 getX로 시작하는 getter의 메서드 규칙을 따르기 때문이다 만약 이러한 부분을 인지하지 못한다면 잘못된 json응답을 내려줄 수 있다.  
  
그러므로 ObjectMapper의 역직렬화의 동작 방식을 알고 있는 것은 도움이 될 수 있다.

## ObjectMapper를 이용한 역직렬화(Deserialize)

### ObjectMapper의 역직렬화 동작 방식

ObjectMapper는 리플렉션을 활용해서 Json 문자열로부터 객체를 만들어내는데, 이것을 역직렬화라고 한다.  
  
Spring에서 @RequestBody로 json 문자열을 객체로 받아올 때 역직렬화가 처리된다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fobjh4%2FbtrWGdpI6Sc%2Fqq8iu3TlqTWr3oA2yNJ3C0%2Fimg.png)


역직렬화는 기본적으로 다음과 같은 과정을 거쳐 처리된다.
1. 기본 생성자로 객체를 생성한다.
2. 필드값을 찾아 바인딩한다.

가장 먼저 객체를 생성하는데, 기본 생성자가 없다면 에러를 발생시킨다.  
  
기본 생성자로 객체를 생성한 후에는 필드값을 찾아야하는데, 기본적으로 public 필드 또는 public 형태의 getter/setter로 찾을 수 있다.  
  
만약 처리에 실패하면 예외가 발생하게되므로 기본 생성자와 getter 메서드는 반드시 만들어주는 것이 좋다.

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

### 우회적으로 역직렬화 처리
만약 기본 생성자가 아닌 우회적인 방법으로 객체를 만드려면 벌도의 2가지 작업이 필요하다.
- ObjectMapper에 ParameterNames 모듈 추가
- Java 컴파일의 -parameters 옵션 추가

ObjectMapper를 위한 Jackson 모듈에는 다음과 같은 parameter-names 모듈이 있다.

```maven
 // <https://mvnrepository.com/artifact/com.fasterxml.jackson.module/jackson-module-parameter-names>
implementation group: 'com.fasterxml.jackson.module', name: 'jackson-module-parameter-names'
```

해당 모듈을 ObjectMapper에 등록을 하면 ObjectMapper가 우회적인 방법을 사용할 수 있게 된다.  
  
예를 들면 파라미터가 있는 생성자와 같이 파라미터 정보를 기반으로 하는 정보들이 사용이 가능해진다.

```java
ObjectMapper objectMapper = new ObjectMapper();
objectMapper.registerModule(new ParameterNamesModule());
```

그러면 이제 파라미터 정보를 얻어올 수 있어야 ParameterNamesModule을 활용할 수 있는데, 해당 부분은 자바 컴파일의 parameters 옵션을 사용하면 된다.  
  
자바 컴파일에 parameters 옵션을 추가해주면 jdk8 이상에는 컴파일시에 리플렉션 api로 파라미터 정보를 가지고 올 수 있도록 컴파일된 클래스에 정보를 추가해준다.  
  
참고로 IntelliJ의 paramters옵션은 다음과 같이 추가해줄 수 있으며, 반드시 Build > Rebuild Projects를 해주어야 반영이 된다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FNIQH4%2FbtrVAK2moxo%2FQN0e138B0YiZOE0BT53zAK%2Fimg.png)


## Spring boot가 제공하는 추가 기능들

하지만 그레이들 기반의 스프링부트로 개발을 하다보면 기본 생성자가 없는 경우에도 에러 없이 객체가 정상적으로 만들어지는 경험을 할 수 있다.  
  
왜냐하면 그 이유는 Spring boot가 추가적인 설정과 플러그인을 제공해주기 때문이다.  
  
먼저 parameters 옵션은 Spring boot가 제공하는 Gradle의 java 플러그인 때문에 처리해준다.  
해당 플러그인을 사용하면 Java 컴파일의 -parameters 옵션이 자동 추가된다.

```gradle
plugins {
    id 'org.springframework.boot' version '2.7.5'
    id 'io.spring.dependency-management' version '1.0.11.RELEASE'
    id 'java'
}
```

ParamterNames 모듈을 추가하는 부분은 Jackson을 AutoConfigure(자동 설정)하는 과정에서 처리된다.  
  
다른 모듈들과 마찬가지로 해당 모듈의 의존성이 있으면 자동으로 설정을 추가해준다.

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

## 결론 DTO 클래스는 다음과 같이

위에서 설명한 부분을 머리에 담고 생각하여 개발하는 것은 비효율적이다.  
그리고 이러한 것은 뇌에 불필요한 인지 부하를 줄 뿐이다.  
그래서 나름대로 한 가지 규칙을 만들었는데, DTO에는 다음과 같은 코드를 무지성으로 붙여주는 것이다.  
  
그러면 우리는 부담없이 DTO에 일련된 방식을 제공해줄 수 있다.  
만약 @ModelAttribute 사용이 필요하다면 무지성으로 @Setter까지 넣어주면 된다.

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
