# Kotlin Class, Field vs Property

This is an example of a Java field:

```java
public String name = "hope"
```

Here is an example of Kotlin Property:

```kt
var name: String = "hope"
```

They both look very similar, but these are two different concepts. Direct Java equivalent of above Kotlin property is following:

```kt
private String name = "hope";

public String getName() {
    return name;
}

public void setName(String name) {
    this.name = name;
}
```

The default implementation of Kotlin property includes field and accessors(getter for `val`, and getter and setter for `var`).

Thanks to that, we can always replace accessors default implementation with a custom one. 

For instance, if we want to accept only non-blank values, then we can define the following setter:

```kt
var name: String = "hope"
    set(value) {
        if(value.isNotBlank())
            field = value
    }
name = "hope"
name = ""
print(name) // hope
```

If we want to be sure that the returned property value is capitalized, we can define a custom getter which capitalizes it:

```kt
var name: String = "hope"
    get() = field.capitalize()
name = "hope"
print(name) // Hope
```

The key fact regarding properties is that they actually are defined by their accessor.

A property does not need to include any field at all.

When we define custom accessors that are not using any field, then the field is not generated.

```kt
var name: String
    get() = "hope"
```

This is why we can use property delegation. See example of property delegate below:

```kt
var name: String by NameDelegate()
```

Above code is compiled to the following implementation:
```kt
val name$delegate = NameDelegate()
var name: String
    get() = name$delegate.getValue(this, this::name)
    set(value) { name$delegate.setValue(this, this::name, value) }
```

Moreover, while a property is defined by its accessor, we can specify it in the interface:

```kt
interface Person {
    val name: String
}
```

Such declaration means that there must be a getter defined in classes that implement interface Person.


As you can clearly see. Kotlin properties give developers much bigger possibilities Java fields. 

Yet, they look nearly the same and Kotlin properties can be used exactly the same as Java fields.

This is a greate example, how Kotlin hide complexity under the carpet and gives us possibilities even if some developers remain unaware of them