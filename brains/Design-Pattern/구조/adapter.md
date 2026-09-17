# 어댑터 패턴

### 어댑더 패턴이란 ?
- 한 클래스의 인터페이스를 사용하고자 하는 다른 인터페이스로 변환할 때 주로 사용한다.
- 이를 이용하면 인터페이스 호환성이 맞지 않아 같이 쓸 수 없는 클래스를 연관 관계로 연결해서 사용할 수 있게 해주는 패턴이다.

### 장점
1. 관계까 없는 인터페이스 간 같이 사용 가능
2. 프로그램 검사 용이
3. 클래스 재활용성 증가 등

### Java Adapter Pattern - 구조 설명
아래와 같은 다이어그램으로 나타낼 수 있다.
![어댑터 구조](../http/image/adapt.png)

<br>

#### 아래는 AudioPlayer 인터페이스와 AudioPlayer 인터페이스를 구현하는 mp3 클래스다.

> AudioPlayer.java

```java
public interface AudioPlayer {
  void play(String filename);
}
```

> MP3.java

```java
public class MP3 implements AutoPlayer {

  @Override
  void play(String filename) {
    System.out.println("Playing MP3 FILE :" + filename);
  }
}
```

#### 아래는 VideoPlayer 인터페이스와 VideoPlayer 인터페이스를 구현하는 mp4 , mkv 클래스다

> VideoPlayer.java

```java
public interface VideoPlayer {

  void play(String filename);
}
```

> MP4.java
```java
public class MP3 implements VideoPlayer{
   
   @Override
   void play(String filename){
      System.out.println("Playing MP4 File ▶ : "filename);
   }
   
}
```

> MKV.java

```java
public class MKV implements VideoPlayer{
   
   @Override
   void play(String filename){
      System.out.println("Playing MKV File ▶ : "filename);
   }
   
}
```

아래는 VideoPlayer 포맷을 AudioPlayer 포맷에서도 사용할 수 있게 도와주는 FormatAdapter Class 이다.  
FormatAdapter class는 AudioPlayer 인터페이스를 상속받고, 멤버 변수로 VideoPlayer를 사용한다.  
생성자로 VideoPlayer를 입력받아 해당 Video포맷을 사용한다.

> FormatAdapter.java

```java
public class FormatAdapter implements AudioPlayer{
   
   private VideoPlayer media;
   
   public FormatAdapter(VideoPlayer video){
      this.media = video;
   }
   
   @Override
   void play(String filename){
      System.out.println("Using Adapter : ");
      media.playFile(filename);
   }
   
}
```

아래 Main Class 는 어댑터 패턴 사용의 예시이다.  
MP3 인스턴스를 AudioPlayer 참조변수로 mp3Player 객체를 생성하였는데,  
MP4 인스턴스에 어댑터를 사용하면 MP4도 mp3Player 에서도 사용할 수 있게된다.  

> Main.java

```java
public class Main{

   public static void main(String[] args){
   
   AudioPlayer mp3Player = new MP3();
   mp3Player.play("file.mp3");
   
   mp3Player = new FormatAdapter(new MP4());
   mp3Player.play("file.mp4");
   
   mp3Player = new FormatAdapter(new MKV());
   mp3Player.play("file.mkv");
   
   }
   
}
```

위 코드를 실행시키면 다음과 같은 결과값이 출력된다.
```
> Playing MP3 File ♪ : file.mp3
> Using Adapter : Playing MP4 File ▶ : file.mp4
> Using Adapter : Playing MKV File ▶ : file.mkv
```
이렇게 어댑터 패턴을 통해 mp3Player 에서도 video 포맷의 파일을 재생시킬 수 있다.