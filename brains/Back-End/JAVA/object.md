# 자바 Object 클래스의 메소드 

자바의 클래스를 선언할 때 extends 키워드로 다른 클래스를 상속하지 않으면 암시적으로 `java.lang.Object` 클래스를 상속하게 된다.

따라서 자바의 모든 클래스는 Object 클래스의 자식이거나 자손 클래스가 된다.

> Object 클래스는 필드가 없고 메서드들로 구성되어 있다. 이 메서드들은 모든 클래스들이 Object 클래스를 상속하므로, 모든 클래스에서 이용할 수 있다.

### equals() 객체 비교

형태 `public boolean equals(Object obj) {...}`

매개 타입: Object로 모든 객체가 매개 값이 대입될 수 있다. (Object가 최상위 타입이므로 모든 객체는 Object 타입으로 자동 타입 변환이 될 수 있기 때문)

리턴 타입: boolean 두 객체가 동일한 객체라면 true, 그렇지 않으면 false

equals() 메서드는 비교 연산자인 ==과 동일한 결과가 리턴된다.

논리적으로 동등하다는 의미는 같은 객체이건 다른 객체이건 상관없이 저장하고 있는 데이터가 동일함이다.

String 객체의 equals 메소드는 String 객체의 번지값이 아닌 문자열이 동일한지 조사해서 같다면 true 그렇지 않다면 false를 리턴한다. 즉 String 클래스가 Object의 equals 메서드를 재정의해서 번지 비교가 아닌 문자열 비교로 변경했기 때문이다.

### equals() 메서드를 재정의

매개값이 기준 객체와 동일한 타입의 객체인지 먼저 확인해야 한다.

Object 타입의 매개 변수는 모든 객체가 매개값으로 제공될 수 있기 때문에 instanceof 연산자로 기준 객체와 동일한 타입인지 제일 먼저 확인해야 한다.

만약 비교 객체가 다른 타입이라면 equals 메서드는 false를 리턴해야한다. 비교 객체가 동일한 타입이라면 기준 객체 타입으로 강제 타입 변환해서 필드 값이 동일한지 검사한다.

```java
public class Member {
    public String id;

    public Member(String id) {
        this.id = id;
    }

    @Override
    public boolean equals(Object obj) {
        if(obj instanceof Member) {
            Member member = (Member) obj;
            if(id.equals(member.id)) {
                return true;
            }
        }
        return false;
    }
}
```

### 객체 해시 코드 hashCode()

객체 해시코드: 객체를 식별할 하나의 정수값

Object의 hashCode() 메서드는 객체의 메모리 번지를 이용해 해시코드를 만들어서 리턴한다. 따라서 객체마다 다른 값을 갖고 있다.

논리적 동등 비교시 hashCode()를 오버라이딩할 필요가 있는데, 컬렉션 프레임워크에서 HashSet, HashMap, Hashtable등은 아래와 같은 방법으로 두 객체가 동등한지 비교한다.



![](https://velog.velcdn.com/images/rg970604/post/3fc6c770-d9c3-42d0-b723-53f668d01546/image.png)

우선 hashCode() 메서드를 실행해서 리턴된 해시코드 값이 같은지 검사한다.

해시코드 값이 다르면 다른 객체로 판단하고, 같으면 equals() 메서드로 다시 검사한다.

따라서 hashCode() 메서드가 true로 나와도 equals()의 리턴 값이 다르면 다른 객체가 된다.

