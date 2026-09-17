# HashMap 메소드 및 사용법
- HashMap 이란 '키에 대한 해시 값을 사용하여 값을 저장하고 조회하며, 키-값 쌍의 개수에 따라 동적으로 크기가 증가하는 associate array' 라고 할 수 있다.
- 이 associate array(연관 배열)은 Map, Dictionary , Symbol Table 이라고도 불리운다.
  
  
간단하게 말하면 key-value의 형태를 가진 키와 값이 **1:1 매핑이 되어 하나의 쌍으로 하여 중복된 키를 허용하지 않는 기본적으로 순서가 없는 자료구조**이다.  
  
기본적으로 equals() 를 사용하여 중복을 판단하기에 primitive data type은 걸러지지만, 객체는 객체의 값이 같더라도 equals() 에서 서로 다르다고 판단하기 때문에 걸러지지 않는다.  
따라서 중복된 객체를 막으려면 equals를 오버라이드 해줘야한다.  

<br>

## HashMap 메서드

### HashMap Constructor
기본적으로 데이터타입은 primitive data type은 불가능하다.   

```java
HashMap<String , Integer> map8 = new HashMap<>();
```
- HashMap은 데이터가 추가되면 저장공간을 약 두배로 늘린다.
- 따라서 초기에 저장할 데이터 개수를 알고 있다면 초기 capacity를 지정해주는 것이 좋다.

<br>

### void clear();
- HashMap안에 들어있던 기존에 요소들을 모두 지운다. 반환값은 없다.

<br>

### boolean isEmpty()
- HashMap 에 element 가 있는지를 판단한다. 없다면 true 있다면 false를 반환한다.

<br>

### boolean containsKey(Object Key)
- 인자로 주어진 Key가 현재 HashMap에 존재하는지를 판단하여 boolean 값을 반환한다. (있으면 참 없으면 거짓)

<br>

### boolean containsValue(Object value)
- 인자로 주어진 Value를 가진 Key가 현재 HashMap에 존재하는지를 판단하여 boolean 값을 반환한다.

<br>

### Set<Map.Entry<K,V>> entrySet();
- HashMap의 모든 요소를 "키=밸류" 형태로 묶어 Set으로 반환한다.

<br>

### Set<K> KeySet()
- HashMap의 모든 요소의 키만 키 형태로 묶어 Set으로 반환한다.

<br>

### Collection<V> values()
- HashMap의 모든 요소의 값만 묶어 반환한다.

<br>

### V get
- 인자로 주어진 key와 매핑되는 value 를 반환해준다.
- HashMap 에 키가 없다면 null을 반환하다.

<br>

### V put(K key, V value)

- 인자로 주어진 key=value 쌍을 HashMap에 추가한다.
- 만약 이미 HashMap안에 Key 가 존재할 경우, 나중에 put된 value가 들어간다.

<br>

### V remove(Object key)
- HashMap에 주어진 key 가 있으면 그 key=value 쌍을 제거하고 value를 반환한다.
- 주어진 key가 HashMap에 없다면 null을 반환한다.

<br>

### V replace(key, value)
- 기존에 존재하던 HashMap의 key=old_value 를 새로운 key=value로 바꾼다.
- replace에 성공하면 기존에 존재하던 old_value 를 반환하고, key가 존재하지 않아 replace에 실패하면 null을 반환한다.

<br>

### void forEach
- forEach를 사용하여 HashMap의 각 key=value 쌍에 접근할 수 있다.
- lambda식도 사용가능하므로 Iterator를 통한 순회보다 더욱 간단하게 코드를 짤 수 있다.
- keySet()이나 entrySet(),values()를 통하여 만들어진 Set들도 forEach문으로 접근이 가능하다. 