# Adapter Pattern

### What is the Adapter Pattern?
- It is primarily used when converting the interface of one class to another interface that you want to use.
- It's a pattern that allows classes with incompatible interfaces, which normally cannot be used together, to be connected and used through an association.

### Advantages
1. Allows unrelated interfaces to be used together
2. Easier program inspection
3. Increased class reusability, etc.

### Java Adapter Pattern - Structure Explanation
It can be represented by the following diagram.
![어댑터 구조](../http/image/adapt.png)

<br>

#### Below is the AudioPlayer interface and the MP3 class that implements the AudioPlayer interface.

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

#### Below is the VideoPlayer interface and the MP4, MKV classes that implement the VideoPlayer interface.

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

Below is the FormatAdapter Class, which helps make VideoPlayer formats usable with AudioPlayer formats.
The FormatAdapter class inherits the AudioPlayer interface and uses VideoPlayer as a member variable.
It receives a VideoPlayer as a constructor argument and uses that video format.

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

The Main Class below is an example of using the Adapter Pattern.
An `mp3Player` object was created as an AudioPlayer reference variable with an MP3 instance.
By using an adapter with an MP4 instance, MP4 can also be used with `mp3Player`.

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

Running the code above will produce the following output:
```
> Playing MP3 File ♪ : file.mp3
> Using Adapter : Playing MP4 File ▶ : file.mp4
> Using Adapter : Playing MKV File ▶ : file.mkv
```
Thus, through the Adapter Pattern, `mp3Player` can also play video format files.
