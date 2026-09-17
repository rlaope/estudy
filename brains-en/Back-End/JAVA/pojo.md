# 🧐 Understanding POJO (Plain Old Java Object)

#### Overview

What exactly is a POJO? I was looking for topics to study today when

POJO caught my eye. Its name seemed somewhat endearing, so I decided to study it today.

### POJO

POJO (Plain Old Java Object), when literally translated, means an old-fashioned, simple Java object.

![](image/pojo_0.png)

To elaborate a bit more on what an old-fashioned, simple object means, it refers to a pure Java object that operates independently of any specific technology.

```
@Getter
@Setter
public class UserDto{

    private String name;
    private String email;
    private String password;
}
```

For example, the object above only has pure Java features like Getter and Setter.

Since it's a pure Java object not dependent on any specific technology, it can be called a POJO.

An opposing example is:

```
public class WindowExam extends WindowAdapter{
	
    @Override
    public void windowColsing(WindowEvent e){
    	System.exit(0);
    }
}
```

As shown, it inherits the WindowAdapter class to use Window functionality.

This creates a problem where a large amount of code needs to be refactored when trying to use a different solution.

Thus, being dependent on specific technologies and environments not only reduces code readability but also creates difficulties in maintenance and scalability.

The concept of POJO emerged to revive Java, which had lost the advantages of object-orientation.

### Why Aim for POJO?

Before the Spring Framework, there were enterprise technologies, and objects were designed to directly use those technologies. This development approach was prevalent, and **Java code that became dependent on specific technologies and environments** suffered from reduced readability, leading to maintenance difficulties. Furthermore, inheriting classes or directly depending on specific technologies resulted in a significant lack of scalability. **This meant that Java, the embodiment of object-orientation, had lost the advantages of object-oriented design.**

Thus, the concept of POJO emerged. It refers to pure Java objects, using the old-fashioned way to leverage Java's inherent advantages.

### How to Check if Code is POJO-Based

1. Is it independent of specific conventions or environments?

2. Is it designed in an object-oriented manner?

3. Is it easy to test?

In "Toby's Spring," a true POJO is defined as follows:

*So, can we say that anything not dependent on specific technical conventions and environments is a POJO? This is one of the biggest misunderstandings many developers have. ...(omitted)...*
***A true POJO refers to an object designed in a way that adheres to object-oriented principles, is independent of environment and technology, and can be reused as needed.***

#### POJO Framework

There are two representative frameworks that provide the technical foundation for POJO programming, and these are the most familiar to us: **Spring Framework and Hibernate**.

#### Advantages of POJO

1. Code becomes cleaner.

2. Favorable for automated testing.

3. Object-oriented design can be applied freely.
