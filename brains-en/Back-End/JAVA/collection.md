# Collection Framework
The java.util package contains numerous data structure classes for handling data. These data structure classes are collectively known as the Collection Framework.

### Data Structures
- A data structure is a structure that can store data.
- For example, just as you use a bookshelf to store books, you need methods to manage various types of data in different ways. Data structures and the Collection Framework provide these methods.

<br>

### Collection Framework
- The most fundamental interface in the Collection Framework is the Collection interface.
  - The Collection interface signifies the presence of data.
  - The Collection interface allows duplicates and does not remember the order in which data was stored.
  - Key methods of the Collection interface include `add()`, `size()`, and `iterator()`.
  - Since Collection does not remember the storage order, it cannot provide functionalities like "give me the first element" or "give me the second element."
  - Collection returns an `Iterator` interface, which allows retrieving stored data one by one.
    - The `Iterator` has a `hasNext()` method to check if there are more elements to retrieve and a `next()` method to retrieve an element.

<br>

### Set Data Structure
- An interface representing a data structure that does not allow duplicates.
  - It inherits from the Collection interface.
  - The `add` method of the Set interface returns `false` if the element already exists and `true` if it's a new element.

### List Data Structure
- Represents a data structure that allows duplicates and remembers the order.
  - Similar to the Set interface, it inherits from the Collection interface.
  - Since List remembers the order, it has a `get` method to retrieve the n-th element.

### Map Data Structure
- It is a data structure that holds Key-Value pairs.
  - When storing data, it uses the `put(key, value)` method to store key-value pairs together.
  - To retrieve a desired value, it uses the `get()` method, which takes the key as a parameter.
  - All Keys stored in a Map must not have duplicate values.
  - Due to this characteristic of Keys, Map has a `keySet()` method that returns a Set containing information about all its Keys.

<br>

[더욱 자세한 문법](https://github.com/KIMHUEMANG/StudyJavaGrammers/tree/master/src/_CollectionFramwork)
