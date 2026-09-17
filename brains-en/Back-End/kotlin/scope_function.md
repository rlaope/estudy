# Kotlin Scope Functions

### Scope Functions

Scope functions are special functions used to define the scope of an object. They operate similarly to methods in object-oriented programming, but their purpose is to limit the scope of an object.

Kotlin provides the following five scope functions: let, apply, run, also, with

Let's refer to this diagram.

#### let

The `let` function takes an object as an argument, executes a lambda expression, and returns the result. For example, the code below executes `println` if `nullableVariable` is not null.

```
nullableVariable?.let { value ->
    println(value)
}
```

The `let` function is useful for handling null values. For example, the code below returns `defaultValue` if `nullableVariable` is null.

```
val result = nullableVariable?.let { value ->
    value
} ?: defaultValue
```

#### apply

The `apply` function takes an object as an argument, executes a lambda expression, and returns the object itself. For example, the code below creates a `Person` object and initializes its field values.

```
val person = Person().apply {
    name = "esperer"
    age = 18
    address = "우리집"
}
```

#### run

The `run` function takes an object as an argument, executes a lambda expression, and returns the result. For example, the code below creates a `Person` object, initializes its field values, and then returns the result of executing the `displayName` function.

```
val result = Person().run {
    name = "esperer"
    age = 18
    address = "우우리이집"
    displayName()
}
```

#### also

The `also` function takes an object as an argument, executes a lambda expression, and returns the object itself. For example, the code below creates a `Person` object, initializes its field values, then executes the `println` function, and returns the object itself.

```
val person = Person().also {
    it.name = "esperer"
    it.age = 10
    it.address = "12myhouse"
    println(it)
}
```

#### with

The `with` function takes an object as an argument, executes a lambda expression, and returns the result. For example, the code below creates a `Person` object, initializes its field values, and then returns the result of executing the `displayName` function.

```
val result = with(Person()) {
    name = "esperer"
    age = 18
    address = "12asfasdfmyhouseSt"
    displayName()
}
```

### Characteristics, Differences, and Use Cases of Each Scope Function

![](image/scope_function_0.png)

1. `apply`: A function that **creates a scope for initialization tasks before a new instance is created and assigned to a specific variable**. Its characteristic is that after all commands are executed, the commands are applied, and the newly created instance is returned.
2. `run`: The clear difference from `apply` is that it returns **the result of executing commands within the scope, not the created instance**. Therefore, it can be used when you need the value of an already created instance or a specific calculated value derived from it.
3. `with`: Somewhat anticlimactically, `with` only differs from `run` in appearance; there are **no operational or characteristic differences**.
4. `also`/`let` and `apply`/`run` have consistent behavior (return values) with the functions as shown in the diagram above. However, a difference lies in **the use of `it`**.

In terms of scope functions performing specific operations within a particular object context, **`also` and `let` seem to be the best scope functions**. Personal opinion.

You should use the appropriate scope function according to the situation and purpose.
