# [Kotlin, Spring boot] copy() Deep Copy, Shallow Copy Performance Benefits? 🤔

### **Overview**

While building a project using Kotlin Spring Boot with a clean architecture, I used the copy() method to encode passwords when converting a RequestDto to a Dto (toDto) and then back to an entity (toEntity). This had the advantage of the converter not depending on the password encoder, with only the service depending on it. (If the password were encoded directly in toDto, the converter would depend on the password encoder.)

![](image/copy_0.png)

Given that the copy() method creates a new object and allocates it to a different memory space, I wondered if there would be a significant performance difference. After benchmarking the performance, using copy() turned out to be slightly faster.

![](image/copy_1.png)

However, I suddenly became curious: even if it allocates space using different memory, would this truly be safe and fast when considering the entire project? So, I investigated copy(), deep copy, and shallow copy.

#### **copy(), Shallow Copy, and Deep Copy**

**copy()**

* In Kotlin, the copy() method is used to copy objects. Kotlin automatically generates copy() for data classes.
* copy() allows you to copy an object and then modify some of its properties.
* When using copy in Kotlin, performance differences can occur, and generally, performance is determined by the **size of the object** using the copy function.
* Naturally, small objects operate very quickly, but large objects can take a very long time to copy.

**Shallow Copy**

* The copy function performs a shallow copy, which means it does not copy the internal references of an object, but **simply copies the references themselves.**
* Therefore, since it references the original object, the copied object and the original object refer to the same object. **So, if you change the value of the copied object, the value of the original object also changes.**
* Performance can degrade if the object contains many internal references or if a deep copy of the object is required.

```
val originalList = mutableListOf(1, 2, 3)
val shallowCopyList = originalList

shallowCopyList[0] = 0
println(originalList) // [0, 2, 3]
println(shallowCopyList) // [0, 2, 3]
```

As shown in the code above, when a shallow copy is made, if the value of a list within the object is modified in the copied object, the value of the list in the original object is also modified.

**Deep Copy**

* A deep copy copies the entire content of an object. This means the copied object and the original object refer to different objects, so even if you change the value of the copied object, the value of the original object does not change.
* For example, if you deep copy a list and change its values as follows, only the copied list will be modified.

```
val originalList = mutableListOf(1, 2, 3)
val deepCopyList = originalList.toMutableList()

deepCopyList[0] = 0
println(originalList) // [1, 2, 3]
println(deepCopyList) // [0, 2, 3]
```

Therefore, depending on the object's copying method, shallow copy and deep copy can result in the copied object and the original object referring to either the same or different objects. In Kotlin, most objects support both shallow and deep copying.

```
data class Person(val name: String, val age: Int, val address: String, val phone: Phone)
data class Phone(val number: String)

val person1 = Person("hope", 18, "Korea", Phone("123-4567"))
val person2 = person1.copy(phone = Phone("555-5555"))
```

As shown in the code above, if you have two data classes and use copy to create a new Phone object with a different value, a **deep copy** occurs, resulting in two distinct objects. If no values are changed, a **shallow copy** occurs.

*If you want to change the contents of an object without modifying the original object, you must perform a deep copy. In this case, when creating a new object by passing arguments to the copy method, you need to create and pass the necessary new objects.*

#### **Equality and Identity of Shallow-Copied and Deep-Copied Objects**

***The equality and identity of shallow-copied and deep-copied objects are different.***

A shallow-copied object only copies the reference value of the object. Thus, the copied object and the original object refer to the same object. Therefore, the result of the object comparison operators == and === is true. In short, both equality and identity are satisfied.

On the other hand, a deep-copied object is a different object from the original, so both its identity and equality are different. Because a new object is created, the original object of a deep-copied object refers to a different object. Therefore, a deep-copied object and the original object are neither identical nor equal.

The operators == and === will result in false.

* ***Shallow-copied objects: ==, === are all true***
* ***Deep-copied objects: ==, === are all false***

#### **Advantages of using the copy() method**

* **Reduced Memory Usage**: When the copy() method copies an object, it allocates new memory space and copies the data. In this process, the copied object uses a separate memory space from the original object, thus reducing memory usage.
* **Improved Speed**: When the copy() method copies an object, it uses a fast algorithm during the data copying process. Therefore, the copying process is handled quickly. This can help improve performance in large-scale data processing.
* **Ensured Stability**: When the copy() method copies an object, it uses a separate memory space from the original object, which can prevent errors or exceptions that might occur during the copying process. Furthermore, it can be used safely in multi-threaded environments.
* **Code Conciseness**: Using the copy() method eliminates the need to write object copying code manually, thus maintaining code conciseness. This can improve code readability and maintainability.

***Returning to the project context, what are the benefits of using copy during the SignUpDto conversion process?***

-> The password encoding process is a relatively time-consuming operation. Therefore, when converting to an entity object, the task of creating the user entity object and **the password encoding task occur together. This means the object creation and password encoding tasks are performed concurrently. Consequently, the conversion process can take a long time.**

If you create a new object using copy, you can maintain the original object's state because the original object is left untouched while a new one is created. Therefore, by creating a new object using copy and handling the password encoding separately, you can preserve the original object's state. This means you can perform the conversion without altering the original object's state by creating a new object via copy and processing password encoding separately.

**Here's a question: what are the advantages of leaving the original object untouched? I've considered it.**

1. Maintaining the original object's state allows you to use the original object as-is for other operations. That is, even if the password was encoded for a User, the SignUpDto's values remain unchanged, allowing for other processing.
2. When an object's state changes, those changes can affect other unexpected parts of the code. Using copy to preserve the object reduces the risk of such errors.
3. When an object's state changes, it can also affect other code that uses that object.

Therefore, considering performance benefits, it seems better to use copy to create a new object and handle password encoding separately during the DTO to entity conversion process.

**Why does performance improve when separating the password encoding and entity conversion processes?**

1. **Password encoding is a security-sensitive operation that demands significant CPU computation.** However, converting a DTO object to an entity object incurs more overhead in object creation and copying than in CPU computation. Therefore, performing password encoding and entity conversion as a single operation can slow down the entire process due to the CPU-intensive password encoding task required for security reasons.
2. Separating password encoding and entity conversion allows you to skip the password encoding step when using the entity object later. There's no need to re-encode the password when validating the entity object's password. Therefore, by separating password encoding and entity conversion, you can avoid unnecessary work and improve performance.
3. Keeping DTO and entity objects separate allows for a clear distinction of their roles. DTOs are used for data exchange between client and server, while entities are used for persistence in conjunction with the database. Therefore, clearly distinguishing between these two types of objects can improve readability and maintainability.

Thus, I resolved my curiosity regarding copy performance while working on the project. I will continue to favor the copy function, research performance-related code, and strive to become a better developer through further study. I extend my gratitude for reading this long and profound post.
