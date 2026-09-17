# data 클래스로 persistence 구현하기

`kotlin-jpa` 플러그인을 빌드 파일에 추가하면 JPA를 쉽게 사용할 수 있다.

```kt
data class Person(
    val name: String,
    val dob: LocalDate
)
```

JPA 관점에서 data클래스는 두 가지 문제가 있다.
1. JPA는 모든 속성에 기본값을 제공하지 않는 이상 기본 생성자가 필수지만 data 클래스에는 기본 생성자가 없다.
2. val 속성과 함께 data 클래스를 생성하면 불변 객체가 생성되는데, JPA 객체와 더불어 잘 동작하도록 설계되어 있지 않다.


## 기본 생성자 문제
코틀린은 기본 생성자 문제를 해결하기 위해 2가지 플러그인을 제공한다.  
`no-arg` 플러그인은 인자가 없는 생성자를 추가할 클래스를 선택할 수 있고, 기본 생성자 추가를 호출하는 어노테이션을 정의할 수 있다. `no-arg`플러그인은 코틀린 엔티티에 기본 생성자를 자동으로 구성한다.

```kts
plugins {
	kotlin("plugin.jpa") version "1.3.72"
}

dependencies {
	implementation("org.springframework.boot:spring-boot-starter-data-jpa")
}
```

`kotlin-spring` 플러그인 처럼 빌드 파일에 필요한 문법을 추가해 `no-arg` 플러그인을 사용할 수 있다. 컴파일러 플러그인은 합성한 기본 연산자를 코틀린 클래스에 추가한다. 즉 자바나 코틀린에서는 합성 기본 연산자를 호출할 수 없다. 하지만 스프링에서는 리플렉션을 사용해 합성 기본 연산자를 호출할 수 있다.  
  
`kotlin-jpa` 플러그인이 `no-arg` 플러그인보다 사용하기 더 쉽다 `kotlin-jpa` 플러그인은 `no-arg` 플러그인을 기반으로 만들어졌다. `kotlin-jpa` 플러그인은 다음 어노테이션으로 자동 표시된 클래스에 기본 생성자를 추가한다.

- @Entity
- @Embeddable
- @MappedSuperClass

## JPA 엔티티에 불변 클래스 사용의 어려움
JPA 엔티티에 불변 클래스를 사용하고 싶지 않는다. 따라서 스프링 개발 팀은 엔티티로 사용하고 싶은 코틀린 클래스에 필드 값을 변경할 수 있는 속성을 var 타입으로 사용하는 단순 클래스를 추천한다.