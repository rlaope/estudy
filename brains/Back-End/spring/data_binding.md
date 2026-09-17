# Spring Data Binding, Converter, Formatter

## Data Binding
사용자나 외부 서버의 요청 데이터를 특정 도메인 객체에 저장해서 우리 프로그램 Request에 담아준다.

## Converter

`Converter\<S,T> Interface`
  
S(Soource)타입에서 T(Target) 타입으로 변환해주는 인터페이스

```java
package org.springframework.core.convert.converter;

public interface Convert<S, T> {
	T convert(S source);
}
```

### 데이터를 특정 object에 담고 싶은 경우
PathParameter, RequestBody 등 담고 싶은 경우

- ConvensionService라는 Spring에 내장된 서비스에서 Converter 구현체 Bean들을 Converter List에 등록함
- 외부 데이터가 들어오고, Source Class Type -> Target Class Type이 컨버터에 등록된 형식과 일치하면 해당 컨버터가 자동으로 동작한다.

파라미터에 JSON 형식 문자열이 담겨오는 경우 해당 문자열을 특정 DTO에 담기

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

헤더에 담긴 json 형식 문자열을 XAuthUser에 바로 담고 싶은 경우 아래와 같이 Converter를 Bean으로 등록한다.

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
특정 객체 -> String 간의 변환을 담당한다.
  
응답을 할 때도 활용.

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

> Formatter도 Converter와 마찬가지로 Spring Bean으로 등록하면 자동으로 ConversionService에 등록시켜준다.