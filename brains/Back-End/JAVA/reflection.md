# 자바 리플랙션 Reflection 

## 리플랙션
JVM은 **클래스 정보를 클래스 로더를 통해 읽어와서 해당 정보를 JVM 메모리에 저장한다.**  
  
그렇게 저장된 클래스에 대한 정보가 마치 거울에 투영된 모습과 닮아있어, 리플렉션이라는 이름을 가지게 되었다.  
  
리플렉션을 사용하면 **생성자, 메서드, 필드**등 클래스에 대한 정보를 아주 자세히 알아낼 수 있다.  
  
대표적으로 여러 라이브러리, 프레임워크에서 사용되는 어노테이션이 리플렉션을 사용한 예시다.  
  
방금 말했듯 리플렉션을 사용하면 클래스와 메서드에 어떤 어노테이션이 붙어 있는지 확인할 수 있다.  
  
어노테이션은 그 자체로는 아무 역할도 하지 않는다.  
리플렉션 덕분에 우리가 스프링에서 `@Component`, `Bean`과 같은 어노테이션을 프레임워크의 기능을 사용할 수 있는 것이다.

## Class 클래스
리플랙션의 가장 핵심은 Class이다. 클래스는 **java.lang** 패키지에서 제공된다. 어떻게 특정 클래스의 `Class` 인스턴스를 획득할 수 있을까?

### Class 객체 획득 방법


```java
Class<Member> aClass = Member.class; // (1)

Member member1 = new Member();
Class<? extends Member> bClass = member1.getClass(); // (2)

Class<?> cClass = Class.forName("hudi.reflection.Member"); // (3)
```

1. 첫번째 방법으로는 클래스의 class 프로퍼티를 통해 획득하는 방법이다. 
2. 두번째 방법으로는 인스턴스의 `getClass() 메서드를 사용하는 것이다.  
3. 세번째 방법으로는 Class 클래스의 forName() 정적 메소드에 FQCN(Fully Qualified Class Name)을 전달해 해당 경로와 대응하는 클래스에 대한 Class 클래스 인스턴스를 얻는 방법이다.

이런 Class의 객체는 Class에 public 생성자가 존재하지 않아 우리가 직접 생성할 수 있는 방법은 없다.  
대신 Class의 객체는 JVM이 자동으로 생성해준다.

### getXXX() vs getDelcaredXXX()

Class 객체 메서드 중 `getFields()`, `getMethods()`, `getAnnotations()`와 같은 형태가  
`getDeclaredFields()`, `getDeclaredMethod()`, `getDeclaredAnnotations()`와 같은 형태로 메솓가 정의되어 있는 것을 확인할 수 있다.  
  
이 메서드들은 클래스에 정의된 필드, 메서드, 어노테이션 목록을 가져오기 위해 사용된다.  
  
편의상 두 형태를 각각 getXXX(), getDeclaredXXX()로 부르겠다. 이 둘은 무엇이 다를까?

- getXXX(): 상속받는 클래스와 인터페이스를 포함하여 모든 public 요소들을 가져온다.
  - 예를 들어 getMethods()는 해당 클래스가 상속받은 그리고 구현한 인터페이스에 대한 모든 public 메서드를 가져온다.
- getDeclaredXXX()는 상속받은 인터페이스를 제외하고 해당 클래스에 직접 정의된 내용만 가져온다. 또한 접근 제어자와 상관 없이 요소에 접근할 수 있다.
  - 예를 들어 getDeclaredMethods()는 해당 클래스에만 직접 정의된 private, protected, public 메서드를 전부 가져온다.

## Constructor
Class 를 사용해서 생성자를 Constructor 타입으로 가져올 수 있다. Constructor는 java.lang.reflect 패키지에서 제공하는 클래스이며 클래스 생성자에 대한 정보와 접근을 제공한다. 리플렉션으로 생성자에 직접 접근하고, 객체를 생성해보자.

```java
Constructor<?> constructor = aClass.getDeclaredConstructor();
// 생성자 가져오기

Object object = constructor.newInstance();
// 이렇게 인스턴스를 생성할 수 있다.

Member member = (Member) constructor.newInstance();
// 타입 캐스팅을 사용해서 위와 같이 받아올 수 있다.
```

위와 같이 Class 타입 객체의 getConstructor()를 사용하여 Constructor를 획득할 수 있다.  
이것의 newInstance() 메서드를 사용해 객체를 직접 생성할 수 있다.  
타입 캐스팅을 사용하지 않으면 Object 타입으로 받아와지므로, 타입 캐스팅을 사용하자.  
  
위 예시는 기본 생성자를 가져오는 예시이다. 생성자에 파라미터가 있다면 어떻게 할까? getConstructor() 메서드에 생성자 파라미터에 대응하는 타입을 전달하면 된다.

```java
Constructor<?> noArgsConstructor = aClass.getDeclaredConstructor();
Constructor<?> onlyNameConstructor = aClass.getDeclaredConstructor(String.class);
Constructor<?> allArgsConstructor = aClass.getDeclaredConstructor(String.class, int.class);
```

파라미터가 존재하는 생성자를 가져왔다면, 아래와 같이 그냥 생성자 사용하듯이 객체를 생성할 수 있게된다.
```java
Member member = (Member) allArgsConstructor.newInstance("희망", 18);
```

그런데 위와 같은 방법으로 private 생성자로 객체를 생성하면 `Java.lang.IllegalAccessException`이 발생한다. 아래와 같이 setAccessible(true)를 사용하면 해결할 수 있다.

```java
noArgsConstructor.setAccessible(true);
Member member = (Member) noArgsConstructor.newInstance();
```
> 참고로 Class 타입의 newInstance() 메서드는 deprecated니까 사용하지 말자


### Field
리플랙션을 사용해 필드에 타입 오브젝트를 획득해 객체 필드에 직접 접근할 수 있다.

```java
Class<Member> aClass = Member.class;
Member member = new Member("희망", 18);

for (Field field : aClass.getDeclaredFields()) {
    field.setAccessible(true);
    String fieldInfo = field.getType() + ", " + field.getName() + " = " + field.get(member);
    System.out.println(fieldInfo);
}

/*
    class java.lang.String, name = 희망
    int, age = 18
*/
```

아래와 같이 set() 메서드를 사용해 Setter가 없어도 강제로 객체의 필드 값을 변경할수도 있다.
```java
Class<Member> aClass = Member.class;
Member member = new Member("희망", 18);

Field name = aClass.getDeclaredField("name");
name.setAccessible(true);
name.set(member, "hope"); // 필드값 변경

System.out.println("member = " + member);
// member = Member{name='hope', age=18}
```

### Method
리플랙션을 사용해 `Method` 타입의 오브젝트를 획득해 객체 메서드에 접근할 수 있다.
```java
Class<Member> aClass = Member.class;
Member member = new Member("hope", 18);

Method sayMyName = aClass.getDeclaredMethod("sayMyName");
sayMyName.invoke(member);
// 내 이름은 hope
```

위 예제와 같이 Method 타입의 invoke()를 사용해 메서드를 직접 호출할 수 있다.

## 어노테이션 가져오기
```java
Class<Member> aClass = Member.class;

Entitiy entityAnnotation = aClass.getAnnotation(Entitiy.class);
String value = entityAnnotation.value();
System.out.println("value = " + value);
// 멤버
```

위와 같이 getAnnotation() 메서드에 직접 어노테이션 타입을 넣어주면, 클래스에 붙어있는 어노테이션을 가져올 수 있다.  
어노테이션이 가지고 있는 필드에도 접근할 수 있는 것을 확인할 수 있다.

## 리플렉션의 단점
일반적으로 메서드를 호출하면, 컴파일 시점에 분석된 클래스를 사용하지만 리플렉션은 **런타임에 클래스를 분석하므로 속도가 느리다.**  
  
JVM을 최적화할 수 없기 때문이라고 한다. 그리고 이런 특징으로 인해 **타입 체크가 컴파일 타임에 불가능**하다.  
  
또한 객체의 추상화가 깨진다는 단점도 존재한다.  
  
따라서 일반적인 웹 어플리케이션 개발자는 사실 리플랙션을 사용할 일이 거의 없으며 보통 라이브러리나 프레임워크를 개발할 때 사용된다.  
따라서 정말 **필요한 곳에만 리플랙션을 한정적으로 사용해야한다.**