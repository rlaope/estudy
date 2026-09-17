# Date, SimpleDateFormat 현재 날짜, 시간 출력

### Date 클래스

java에서 현재 시간을 알기 위해 Date라는 객체를 사용한다.  
  
1번 라인처럼 java.util.Date를 임포트한다.  
  
그리고 Date 객체를 만들고 출력해본다.

```java
import java.util.Date;
 
public class CurrentTime{
 
  public static void main(String[] args){
    Date today = new Date();
    System.out.println(today);
  }
 
}
```

출력된 Date 객체는 아래와 같습니다.

```
Mon Dec 7 22:05:02 KST 2022
```

<br>

### SimpleDateFormat 클래스

Date 객체를 그냥 출력하면 평소에 보는 방법과 다르다는걸 볼 수 있다.  

편하게 2022/12/7과 10:05:02 PM과 같이 출력하는 방법을 알아보자.  
  
바로 SimpleDateFormat 클래스를 사용하면 가능하다.  
  
date는 "yyyy/MM/dd"  
time은 "hh:mm:ss a"로 표현하겠다.  
  
```java
import java.text.SimpleDateFormat;
import java.util.Date;
 
public class CurrentTime {
 
  public static void main(String[] args) {
    Date today = new Date();
    System.out.println(today);
        
    SimpleDateFormat date = new SimpleDateFormat("yyyy/MM/dd");
    SimpleDateFormat time = new SimpleDateFormat("hh:mm:ss a");
        
    System.out.println("Date: "+date.format(today));
    System.out.println("Time: "+time.format(today));
  }
 
}
```

```
Mon Dec 25 22:05:02 KST 2017
Date: 2017/12/25
Time: 10:05:02 PM
```