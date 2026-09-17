# Spring Data Binding, Converter, Formatter

## Data Binding
Request data from users or external servers is stored in a specific domain object and then included in our program's Request.

## Converter

`Converter\<S,T> Interface`
  
An interface that converts from S (Source) type to T (Target) type.

```java
package org.springframework.core.convert.converter;

public interface Convert<S, T> {
	T convert(S source);
}
```

### When you want to put data into a specific object
When you want to include data, e.g., in PathParameter, RequestBody.

- In Spring's built-in service called `ConversionService`, Converter implementation Beans are registered in the Converter List.
- When external data comes in, if the Source Class Type -> Target Class Type matches the format registered in the converter, that converter operates automatically.

When a JSON string is included in a parameter, to put that string into a specific DTO.

```java
GET /user-info
x-auth-user : {"id":123, "name":"Esperer"}

//User Object
public class XAuthUser {
	private int id;
	private String name;
}

@GetMapping("/user-info")
public UserInfoResponse getUserInfo(
	@RequestHeader("x-auth-user") XAuthUser xAuthUser) {
	
	//get User Info logic
}
```

If you want to directly put the JSON string contained in the header into `XAuthUser`, register the Converter as a Bean as shown below.

```java
@Component
public class XAuthUserConverter implements Converter<String, XAuthUser> {
	@Override
	public XAuthUser convert(String source) {
		return objectMapper.readValue(source, XAuthUser.class);
	}
}
```

<br>

## Formatter
It handles conversions between a specific object and a String.
  
Also used when generating responses.

```java
package org.springframework.format.datetime;

public final class DateFormatter implements Formatter<Date> {
	public String print(Date date, Locale locale) {
		return getDateFormat(locale).format(date);
	}

	public Date parse(String foamtted, Locale locale) throws ParseException {
		return getDateFormat(locale).parse(formatted);
	}

	//getDateFormat 등 일부 구현 생략
}
```

> Similar to Converters, if a Formatter is registered as a Spring Bean, it is automatically registered with the `ConversionService`.
