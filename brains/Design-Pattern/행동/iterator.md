# 이터레이터 패턴

이터레이터 패턴이란 순회를 할 대상의 구체적인 **컬렉션타입을 알지 않고도 대상을 더욱 쉽게 순회할 수 있는 방법을 제공하는 디자인 패턴**이다.

대표적으로 자바의 Iterator 인터페이스 스프링의 CompositeIterator, StAX가 있다.

## 예시

예를 들어 Book 엔티티가 담겨있는 리스트가 있다고해보자. 이 엔티티를 시간순으로 정렬하고 싶다면 Collections.sort()를 이용해 조건을 넣고 정렬을 한 후 하나씩 for문으로 인덱스 참조 혹은 get 하여 출력해야할 것이다.

그러지 않고 RecentPostIterator와 같은 이터레이터 인터페이스를 구현해 다음과 같이 객체 순회 기능을 제공할 수 있다.

```java
public class RecentPostIterator implements Iterator<Post> {

    private Iterator<Post> internalIterator;

    public RecentPostIterator(List<Post> posts) {
        Collections.sort(posts, (p1, p2) -> p2.getCreatedDateTime().compareTo(p1.getCreatedDateTime()));
        this.internalIterator = posts.iterator();
    }

    @Override
    public boolean hasNext() {
        return this.internalIterator.hasNext();
    }

    @Override
    public Post next() {
        return this.internalIterator.next();
    }

}
```

이런식으로 이터레이터를 인터페이스를 사용한다면 구체적인 컬렉션(Set, List)를 알지 않고도 순회를 할 수 있어 순회하는 방식이 간단해진다.

### 장단점


물론 순회하는 방식마다 새로운 Iterator 클래스를 만들고 순회를 한다면 성능적으로도 느린 단점이 있지만,

더욱 유지보수가 쉽고 접근성이 가벼워지며, **객체를 순회한다는 책임을 이터레이 클래스에게 위임함으로써 응집도를 낮추어 더욱 확장성있는 코드 설계가 가능해진다.**