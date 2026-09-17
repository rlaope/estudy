# Proxy Pattern

### Proxy
- When translated, 'Proxy' means an agent or a representative. An agent or representative is an entity that performs a role on behalf of someone else.
- This applies equally to programs. In other words, you have the proxy perform a task on behalf of something else.

When you want to use an object, instead of directly referencing it, if you access the target object through an object that acts as its substitute, you can reference or set basic information even if the object doesn't exist in memory, and you can defer the creation of the actual object until its functionality is truly needed.

![Proxy Pattern](./image/proxy.png)

## Pros and Cons of the Proxy Pattern

### Advantages
- References can be made through the proxy even before a large object is fully loaded.
- Public and protected methods of the real object can be hidden and exposed only through an interface.
- Objects that are not local but remote can be used.
- Pre-processing can be performed for access to the original object.

### Disadvantages
- Since an extra step is involved in object creation, performance may degrade if frequent object creation is required.
- Performance may degrade if threads need to be created and synchronized within the proxy for object creation.
- The logic can become complex, reducing readability.

<br>

### Types of Proxy Patterns

- **Virtual Proxy**
  - A pattern used when you want to defer object creation until it's absolutely necessary, making it behave as if the object has already been created.
  - The proxy class handles small-scale tasks, and the subject class is only used when resource-intensive operations are required.
- **Remote Proxy**
  - An object that controls access to a remote object, exists in the local environment, and acts as a representative for the remote object.
  - A pattern that makes objects in different address spaces behave as if they are in the same address space (e.g., Google Docs).
- **Protection Proxy**
  - A pattern used to control access rights to an object or to assign different access rights to each object when controlling access to the subject class.
  - The proxy class can decide whether to allow client access to the subject class.

<br>

## Implementing the Proxy Pattern

For example, let's assume you're displaying a document with large images and text on the screen. You'll notice that text, being small in size, appears quickly, but images, being large, load slowly. If this wasn't handled this way, and the screen only appeared after both text and image loading were complete, users would have to wait until loading finished.
Therefore, it's better to display the text that finishes loading first. To achieve this, you can run separate processes for text processing and image processing.

```java
public interface Image {
    public void displayImage();
}
```
```java
// Real_Image.java
public class Real_Image implements Image {
	private String fileName;
    
    public Real_Image(String fileName) {
    	this.fileName = fileName;
    }
    
    private void loadFromDisk(String fileName) {
    	System.out.println("Loading: " + fileName);
    }
    
    @Override
    public void displayImage() {
        System.out.println("Displaying: " + fileName);
    }
}
```
```java
// Proxy_Image.java
public class Proxy_Image implements Image {
    private String fileName;
    private Real_Image realImage;
    
    public Proxy_Image(String fileName) {
    	this.fileName = fileName;
    }
    
    @Override
    public void displayImage() {
    	if (realImage == null) {
        	realImage = new Real_Image(fileName);
        }
        realImage.displayImage();
    }
}
```
```java
// Proxy_Pattern.javva
public class Proxy_Pattern {
    public static void main(String args[]) {
        Image image1 = new Proxy_Image("test1.jpg);
        Image image2 = new Proxy_Image("test2.jpg);
        
        image1.displayImage();
        image2.displayImage();
    }
}
```
As seen in the code above, the `Proxy_Pattern` class does not directly access the `Real_Image` class; instead, it creates an object in the `Proxy_Image` class to perform the task on its behalf.
