# Spring Validation을 이용한 유효성 검증

### Validation
- 올바르지 않은 데이터를 걸러내고 보안을 유지하기 위해 데이터 검증은 여러 계층에 걸쳐서 적용됨.
- Client의 데이터는 조작이 쉬울 뿐더러 모든 데이터가 정상적인 방식으로 들어오는 것도 아니기 때문에, `Client Side`뿐만 아니라 `Server Side`에서도 데이터 유효성을 검사해야 할 필요가 있습니다.
- 스프링부트 프로젝트에서는 @Validated를 이용해 유효성을 검증할 수 있습니다.

### Bean Validation
스프링의 기본적인 validation인 Bean valiation은 클래스 필드에 특정 어노테이션을 적용하여 필드가 갖는 제약 조건을 정의하는 구조로 이루어진 검사입니다.  
validator가 어떠한 비즈니스적 로직에 대한 검증이 아닌, 그 클래스로 생성된 객체 자체의 필드에 대한 유효성 여부를 검증합니다.

<br>

#### @Valid, @Validated 차이
@Valid는 java에서 지원해주는 어노테이션이고, @Validated는 Spring에서 지원해주는 어노테이션입니다. @Validated는 @Valid의 기능을 포함하고, 유효성을 검토할 그룹을 지정할 수 있는 기능을 추가로 가지고 있습니다.

<br>

### @NotNull, @NotEmpty, @NotBlank
-  단순하게 null이 아니고, 비어있지 않고, 공백이 아니다라고는 알고있다.
-  그렇다면 String의 경우 null도 안되고, 비어있지도 않고, 공백도 아니여야 한다면 세가지 어노테이션을 다 달아줘야 할까?


#### @NotNull
제한된 CharSequence, Collection, Map, Array는 null만 아니라면 유효하지만, 비어있을 수는 있다.

#### @NotEmpty
제한된 CharSequence, Collection, Map, Array는 null이 아니고 size또는 length가 0보다 커야한다.

#### @NotBlank
String 은 null이 아니고 trimmed length가 0보다 커야한다.