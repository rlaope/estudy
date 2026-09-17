# Lombok 반드시 알고 써야하는 @annotation 모음

### Lombok
어노테이션을 사용하여 긴 소스코드를 간결하게 만들어주는 Java 라이브러리 중 하나다.

### Lombok Annotation 종류

#### @NotNull
필드의 값이 null이 될 수 없음을 명시

#### @Getter 
필드의 get 메서드를 생성  
@Getter(lazy=true) : getter 최초 호출 시 값을 한번 계산한 후 그 값을 캐시해서 사용(매번 getter를 호출하는 것이 아닌 최초 한번만 호출하여 사용하기 위해)AccessLevel 지정 가능

#### @Setter
필드의 set 메서드를 생성  
AccessLevel 지정 가능

#### @NoArgsConstructor
파라미터가 없는 기본 생성자 생성

#### @RequiredArgsConstructor
초기화되지 않은 `final 필드` , @NotNull인 필드에 대한 생성자 생성

#### @AllArgsConstructor
모든 필드를 가진 생성자 생성

#### ToString
toString() 메서드 생성  
exclude  속성을 사용하여 필요없는 속성 제거 @ToString(exclude ="value")

#### @EqualsAndHashCode
equeals() , hashCode() 메서드 생성  
exclude 속성을 사용 가능

#### @Data
@Getter + @Setter + @RequiredArgsConstructor + @ToString + @EqualsAndHashCode

#### @Value
불변을 의미하는 어노테이션  
@Value가 붙은 멤버 필드는 private 접근 제어자와 final이 붙은 상수가 된다.(final이 붙기 때문에 setter가 존재할 수 없다.)  

#### @Log
Logger 자동 생성 가능  
자동으로 log 필드를 만들고 해당 class 명으로 로거 객체를 생성하여 할당해준다.


#### @Builder

Buillder 자동 생성  
@Singular 어노테이션 사용시 원소를 하나씩 추가할 수 있다.

<br>

### @Builder + @Singular 예제
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

`toString()` : 객체의 값들을 문자열로 변환해 리턴하는 메서드  
`equals()` : 동등성 비교 연산자 (두 객체의 내용이 같은지 비교)  
`hashCode()` : 동일성 비교 연산자 (두 객체가 같은지 비교)