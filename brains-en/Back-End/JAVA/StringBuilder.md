# StringBuilder Usage and Key Methods

> StringBuilder(java.lang.StringBuilder)

### What is StringBuilder?

**It is, as the name suggests, an object that builds Strings.**

If you look at the API, the first line says 'A mutable sequence of characters'.
String is an immutable object; modifying its value creates a new object, which consumes memory and time.
However, using StringBuilder allows you to modify a mutable sequence, meaning only the value at the same memory address is changed, making it operate much faster. (It's like writing notes in a notepad.)

### Constructors
- StringBuilder sb = new StriugBuilder(); : Object declaration
- StringBuilder sb = new StringBuilder("You can directly insert a string");

### Key Methods

#### .append()
Appends a string.
```java
sb.append("bbb").append(4);
```

#### .insert(int offset, String str)
Inserts `str` at the `offset` position.
```java
sb.insert(2,"ccc");
```

### .replace()
Replaces the string located at the numeric indices received as the first and second parameters.
```java
.replace(3,6,"yeye");
```

### substring(int start, int end)
Indexing. If there's one parameter, it indexes from that index to the end; if there are two, it indexes from the start to end - 1.

```java
sb.substring(5);
sb.substring(3,7);
```

#### .deleteCharAt(int index)
Deletes a single character at the specified index.

#### .delete(int start, int end)
Deletes characters from `start` up to `end - 1`.

#### .toString()
Converts to String.

#### .reverse() : Reverses the entire sequence of characters.

#### .setCharAt(int index , String s)
Changes the character at `index` to `s`.

#### .setLength(int len)
- Adjusts string length
- If adjusted to be longer than the current string, it's padded with spaces.
- If adjusted to be shorter than the current string, remaining characters are deleted.


#### .trimToSize()
- Adjusts the size of the `char[]` array storing the string to be equal to the current string length,
- Similar to how the `String` class's `trim()` removes leading/trailing spaces, this provides space trimming,
- Since the remaining size of the array is empty space, it can be seen as removing all trailing spaces from the string.

```java
import java.lang.StringBuilder;

public class sb {
    public static void main(String[] args) throws IOException{
        StringBuilder sb = new StringBuilder("aaa");

        // Append string
        System.out.println(sb.append("bbb")); // aaabbb
        System.out.println(sb.append(4)); // aaabbb4

        // Insert string
        System.out.println(sb.insert(2, "ccc")); // aacccabbb4
        
        // Replace string, substitute string
        System.out.println(sb.replace(3, 6, "ye")); // aacyebbb4

        // Indexing, substring
        System.out.println(sb.substring(5)); // bbb4
        System.out.println(sb.substring(3, 7)); // yebb

        // Delete character
        System.out.println(sb.deleteCharAt(3)); // aacebbb4

        // Delete string
        System.out.println(sb.delete(3, sb.length())); // aac

        // Convert string
        System.out.println(sb.toString()); // aac

        // Reverse string
        System.out.println(sb.reverse()); // caa

        // Replace character, substitute character, change character
        sb.setCharAt(1, 'b');
        System.out.println(sb); // cba

        // Adjust string length
        sb.setLength(2);
        System.out.println(sb); // cb
    }
}

```
