# Date, SimpleDateFormat Current Date, Time Output

### Date Class

In Java, the Date object is used to get the current time.

Import java.util.Date as shown in line 1.

Then, create a Date object and print it.

```java
import java.util.Date;
 
public class CurrentTime{
 
  public static void main(String[] args){
    Date today = new Date();
    System.out.println(today);
  }
 
}
```

The printed Date object is as follows:

```
Mon Dec 7 22:05:02 KST 2022
```

<br>

### SimpleDateFormat Class

If you just print a Date object, you'll notice it's not displayed in a commonly seen format.

Let's find out how to easily display it as 2022/12/7 and 10:05:02 PM.

This is possible by using the SimpleDateFormat class.

date will be represented as "yyyy/MM/dd"
time will be represented as "hh:mm:ss a".

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
