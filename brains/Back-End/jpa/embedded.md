# 임베디드 타입 @Embedded, @Embeddable

## 임베디드 타입

임베디드 타입은 복합 값 타입으로 불리며 새로운 값 타입을 직접 정의해 사용하는 JPA 방법이다.  
  
아래의 코드를 보면 User엔티티는 id, name, email, gender, address를 데이터를 가지고 있는 주소 정보가 도시, 구, 상세조수, 우편번호 등으로 여러개의 컬럼으로 나눠져 있는 것을 볼 수 있다. 이렇게 상세한 데이터를 그대로 갖고 있는 것은 객체지향적이지 않으며 응집력을 떨어뜨린다.  
  
이럴때 임베디드 타입을 사용하면 더욱더 객체지향적인 코드를 만들 수 있다.

```java
// user.java
@Entity
public class User {
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    private Long id;

    @NonNull
    private String name;

    @NonNull
    private String email;


    @Enumerated(value = EnumType.STRING)
    private Gender gender;
    
    // 주소 정보
    private String city; // 도시
    private String district; // 구
    private String detail; // 상세주소
    private String zipCode; // 우편번호

}
```

## @Enbedded, @Embeddable
- 임베디드 타입을 적용하려면 새로운 클래스를 만들고 해당 클래스에 임베디드 타입으로 묶으려던 Attribute들을 넣어준 뒤 @Embeddable를 붙여줘야한다.
- 사용법
  - `@Embeddable`: 값 타입을 정의하는 곳에 표시
  - `@Embedded`: 값 타입을 사용하는 곳에 표시
- 아래의 임베디드 타입을 적용한 코드를 보면 주소의 관련된 속성들이 하나의 타입으로 바껴 사용되는 것을 볼 수 있다, 이를 보면 위의 코드보다 더욱 더 객체지향적이고 응집도 있는 코드로 바뀐 것을 확인할 수 있다.

```java
// user.java
@Entity
public class User {
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    private Long id;

    @NonNull
    private String name;

    @NonNull
    private String email;


    @Enumerated(value = EnumType.STRING)
    private Gender gender;
    
    @Embedded
    private Address address;

}

// Address.java
@Embeddable
@Data
@AllArgsConstructor
@NoArgsConstructor
public class Address {
    // 주소 정보
    private String city; // 도시
    private String district; // 구

    @Column(name = "address_detail")
    private String detail; // 상세 주소
    private String zipCode; // 우편번호
}
```

```java
// 데이터 추가하는 방법
user.setAddress(new Address("서울시", "강남구", "강남대로 123", "16427"));
```

- 임베디드 타입을 사용하여도 동일하게 Column들이 생성되는 것을 확인할 수 있습니다.

![](https://velog.velcdn.com/images%2Fseongwon97%2Fpost%2F496d4fb2-b04c-4060-b041-e992460ce04f%2F%EC%BA%A1%EC%B2%98.PNG)

<br>

## @AttributeOverride: 속성명 재정의
- 같은 종류의 Attribute에 대해서는 중복된 코드를 적을 필요 없이 간단하고 직관성있게 선언 가능하다.
- 예시로는 회사의 주소, 집 주소의 주소 형태는 시,구,상세주소,우편번호로 동일하다. 이러한 경우 @Embedded와 @AttributeOverrides, @AttributeOverride를 통해 하나의 class를 사용해 여러 표현을 할 수 있다.
- 아래의 코드는 객체를 재활용하는 대신 @AttributeOverrides, @AttributeOverride를 사용해 컬럼의 이름을 전부 재정의해 사용하기에 코드가 지저분해 보일 수 있다. -> 객체를 재활용 하지 않고 따로 선언해 하는 대신 깔끔하게 보이는 코드를 작성할 지, 객체의 재활용을 하는 코드를 작성할지는 개발자가 결정해야한다.

```java
@Entity
public class User {

.....

    @Embedded
    @AttributeOverrides({
            @AttributeOverride(name = "city", column = @Column(name = "home_city")), // city를 home_city라는 column명으로 사용
            @AttributeOverride(name = "district", column = @Column(name = "home_district")),
            @AttributeOverride(name = "detail", column = @Column(name = "home_address_detail")),
            @AttributeOverride(name = "zipCode", column = @Column(name = "home_zipCode"))
    })
    private Address homeAddress;

    @Embedded
    @AttributeOverrides({
            @AttributeOverride(name = "city", column = @Column(name = "company_city")),
            @AttributeOverride(name = "district", column = @Column(name = "company_district")),
            @AttributeOverride(name = "detail", column = @Column(name = "company_address_detail")),
            @AttributeOverride(name = "zipCode", column = @Column(name = "company_zipCode"))
    })
    private Address companyAddress;
   
   	.....
}
```

![](https://velog.velcdn.com/images%2Fseongwon97%2Fpost%2F4776e278-32b6-42b9-90a5-301f302dc252%2Fimage.png)

### 주의
하나의 클래스를 통해 여러개의 정보를 만들고 싶은데 위와 같이 @AttributeOverrides, @AttributeOverride를 통해 컬럼명을 재정의해주지 않으면 아래와 같이 Repeated column in mapping for entity 에러가 나오니 꼭 컬럼명을 재정의해서 사용해야한다.

```java
@Embedded
private Address homeAddress;

@Embedded
private Address companyAddress;
```

```
Caused by: javax.persistence.PersistenceException: [PersistenceUnit: default] Unable to build Hibernate SessionFactory; nested exception is org.hibernate.MappingException: Repeated column in mapping for entity: com.example.jpa_study.entity.User column: city (should be mapped with insert="false" update="false")
	at org.springframework.orm.jpa.AbstractEntityManagerFactoryBean.buildNativeEntityManagerFactory(AbstractEntityManagerFactoryBean.java:421)
	at org.springframework.orm.jpa.AbstractEntityManagerFactoryBean.afterPropertiesSet(AbstractEntityManagerFactoryBean.java:396)
	at org.springframework.orm.jpa.LocalContainerEntityManagerFactoryBean.afterPropertiesSet(LocalContainerEntityManagerFactoryBean.java:341)
	at org.springframework.beans.factory.support.AbstractAutowireCapableBeanFactory.invokeInitMethods(AbstractAutowireCapableBeanFactory.java:1845)
	at org.springframework.beans.factory.support.AbstractAutowireCapableBeanFactory.initializeBean(AbstractAutowireCapableBeanFactory.java:1782)
	... 107 more
```

## 임베디드 타입과 null
임베디드 타입 자체를 null로 지정했을 때 임베디드의 타입의 속성 값을 null로 설정했을 때는 모두 해당 column의 값이 null 값으로 나오게 된다.  
  
아래의 코드를 수행하여 결과를 보면 관련 컬럼들의 값이 모두 null인 것을 확인할 수 있다.  
  
즉, **임베디드 타입의 객체가 null인 경우 내부의 모든 컬럼이 null인 것과 동일하게처리**한다.