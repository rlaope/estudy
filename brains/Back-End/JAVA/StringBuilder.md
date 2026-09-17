# StringBuilder 사용법과 주요 메서드

> StringBuilder(java.lang.StringBuilder)

### String Builder란

**말 그대로 String을 만드는 객체다.**

API를 살펴보면 첫줄에 A mutable sequence of characters라고 써있다.  
String은 immutable 한 객체로 값을 수정하면 새로운 객체를 만들어 내서 메모리를 잡아먹고 시간도 잡아먹게 된다.  
하지만 StringBuilder를 사용하면 mutable한 sequence를 수정하는 것이므로 같은 주소에 값만 수정되는 개념이 되게되서 훨씬 빠르게 작동하게 되는 것이다.(메모장에 써 넣어 놓는다고 생각이 든다.)

### 생성자
- StringBuilder sb = new StriugBuilder(); : 객체 선언
- StringBuilder sb = new StringBuilder("문자열을 바로 넣을 수도 있다");

### 주요 메서드

#### .append()
문자열을 추가한다.  
```java
sb.append("bbb").append(4);
```

#### .insert(int offset, String str)
offset 위치에 str을 추가한다.
```java
sb.insert(2,"ccc");
```

### .replace()
첫번째와 두번째 파라미터로 받는 숫자 인덱스에 위치한 문자열을 대체한다. 
```java
.replace(3,6,"yeye");
```

### substring(int start, int end)
인덱싱. 파라미터가 하나라면 해당 인덱스부터 끝까지, 두개라면 시작점과 끝점 -1 까지 인덱싱

```java
sb.substring(5);
sb.substring(3,7);
```

#### .deleteCharAt(int index)
인덱스에 위치한 문자 하나를 삭제한다.

#### .delete(int start, int end)
start 부터 end -1 까지 문자를 삭제한다.

#### .toString()
String으로 변환한다.

#### .reverse() : 해당 문자 전체를 뒤집는다.

#### .setCharAt(int index , String s) 
index 위치의 문자를 s로 변경

#### .setLength(int len)
- 문자열 길이 조정
- 현재 문자열보다 길게 조정하면 공백으로 채워짐
- 현재 문자열보다 짧게 조정하면 나머지 문자는 삭제


#### .trimToSize()
- 문자열이 저장된 char[] 배열 사이즈를 현재 문자열 길이와 동일하게 조정,
- String 클래스의 trim()이 앞 뒤 공백을 제거하는 것과 같이 공백 사이즈를 제공하는 것,
- 배열의 남는 사이즈는 공백이므로, 문자열 뒷부분의 공백을 모두 제거해준다고 보면 됨

```java
import java.lang.StringBuilder;

public class sb {
    public static void main(String[] args) throws IOException{
        StringBuilder sb = new StringBuilder("aaa");

        // 문자열 추가
        System.out.println(sb.append("bbb")); // aaabbb
        System.out.println(sb.append(4)); // aaabbb4

        // 문자열 삽입
        System.out.println(sb.insert(2, "ccc")); // aacccabbb4
        
        // 문자열 치환, 문자열 교체
        System.out.println(sb.replace(3, 6, "ye")); // aacyebbb4

        // 인덱싱, 문자열 자르기
        System.out.println(sb.substring(5)); // bbb4
        System.out.println(sb.substring(3, 7)); // yebb

        // 문자 삭제
        System.out.println(sb.deleteCharAt(3)); // aacebbb4

        // 문자열 삭제
        System.out.println(sb.delete(3, sb.length())); // aac

        // 문자열 변환
        System.out.println(sb.toString()); // aac

        // 문자열 뒤집기
        System.out.println(sb.reverse()); // caa

        // 문자 대체, 문자 교체, 문자 치환
        sb.setCharAt(1, 'b');
        System.out.println(sb); // cba

        // 문자열 길이 조정
        sb.setLength(2);
        System.out.println(sb); // cb
    }
}

```