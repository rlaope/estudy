# Methods of Java's Object Class

When declaring a Java class, if you don't explicitly inherit another class using the `extends` keyword, it implicitly inherits the `java.lang.Object` class.

Therefore, all classes in Java are either direct children or descendants of the Object class.

> The Object class has no fields and consists only of methods. Since all classes inherit the Object class, these methods are available in all classes.

### equals() Object Comparison

Signature: `public boolean equals(Object obj) {...}`

Parameter Type: Object, meaning any object can be passed as an argument. (Because Object is the top-level type, all objects can be automatically type-converted to Object type.)

Return Type: boolean, true if the two objects are identical, false otherwise.

The equals() method returns the same result as the comparison operator ==.

Logical equivalence means that the data stored is identical, regardless of whether they are the same object or different objects.

The equals method of a String object checks if the strings are identical, not their memory addresses, returning true if they are the same and false otherwise. This is because the String class overrides Object's equals method, changing it from an address comparison to a string comparison.

### Overriding the equals() Method

First, you must check if the parameter is an object of the same type as the current object.

Since an Object type parameter can accept any object as an argument, you must first use the instanceof operator to check if it's the same type as the current object.

If the object being compared is of a different type, the equals method should return false. If the objects are of the same type, cast the compared object to the current object's type and check if their field values are identical.

```java
public class Member {
    public String id;

    public Member(String id) {
        this.id = id;
    }

    @Override
    public boolean equals(Object obj) {
        if(obj instanceof Member) {
            Member member = (Member) obj;
            if(id.equals(member.id)) {
                return true;
            }
        }
        return false;
    }
}
```

### Object Hash Code hashCode()

Object Hash Code: A single integer value that identifies an object.

Object's hashCode() method generates and returns a hash code using the object's memory address. Therefore, each object has a different value.

When comparing for logical equivalence, it's necessary to override hashCode(). Collection frameworks like HashSet, HashMap, and Hashtable compare two objects for equivalence using the following method:

![](https://velog.velcdn.com/images/rg970604/post/3fc6c770-d9c3-42d0-b723-53f66b01546/image.png)

First, it executes the hashCode() method and checks if the returned hash code values are the same.

If the hash code values are different, they are considered different objects. If they are the same, the equals() method is used for a second check.

Therefore, even if the hashCode() method returns true, if the equals() method returns a different value, they are considered different objects.
