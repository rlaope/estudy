# HashMap Methods and Usage
- A HashMap can be defined as an 'associate array that stores and retrieves values using hash values for keys, and whose size dynamically increases based on the number of key-value pairs.'
- This associate array is also called a Map, Dictionary, or Symbol Table.

Simply put, it's a **fundamentally unordered data structure** where keys and values form **1:1 mappings as pairs, and duplicate keys are not allowed.**

By default, `equals()` is used to determine duplicates, so primitive data types are filtered out. However, objects are not filtered out because `equals()` considers them different even if their values are the same.
Therefore, to prevent duplicate objects, you must override `equals()`.

<br>

## HashMap Methods

### HashMap Constructor
Basically, primitive data types are not allowed for the type parameters.

```java
HashMap<String , Integer> map8 = new HashMap<>();
```
- When data is added, HashMap approximately doubles its storage capacity.
- Therefore, if you know the initial number of data items to store, it's good practice to specify the initial capacity.

<br>

### void clear();
- Clears all existing elements within the HashMap. It has no return value.

<br>

### boolean isEmpty()
- Checks if the HashMap contains any elements. Returns `true` if empty, `false` otherwise.

<br>

### boolean containsKey(Object Key)
- Determines if the given `Key` exists in the current HashMap and returns a boolean value. (`true` if present, `false` if not).

<br>

### boolean containsValue(Object value)
- Determines if a `Key` associated with the given `Value` exists in the current HashMap and returns a boolean value.

<br>

### Set<Map.Entry<K,V>> entrySet();
- Returns all elements of the HashMap as a `Set`, bundled in "key=value" form.

<br>

### Set<K> KeySet()
- Returns all keys of the HashMap as a `Set`, bundled in key form.

<br>

### Collection<V> values()
- Returns all values of the HashMap, bundled together.

<br>

### V get
- Returns the `value` mapped to the given `key`.
- If the `key` is not present in the HashMap, it returns `null`.

<br>

### V put(K key, V value)

- Adds the given `key=value` pair to the HashMap.
- If the `Key` already exists in the HashMap, the `value` from the later `put` operation will be stored.

<br>

### V remove(Object key)
- If the given `key` exists in the HashMap, it removes that `key=value` pair and returns the `value`.
- If the given `key` is not in the HashMap, it returns `null`.

<br>

### V replace(key, value)
- Replaces an existing `key=old_value` in the HashMap with a new `key=value`.
- If the `replace` operation is successful, it returns the `old_value` that existed previously; if the `key` does not exist and `replace` fails, it returns `null`.

<br>

### void forEach
- You can access each `key=value` pair of the HashMap using `forEach`.
- Lambda expressions can also be used, allowing for simpler code compared to iteration via an `Iterator`.
- `Set`s created through `keySet()`, `entrySet()`, or `values()` can also be accessed with a `forEach` loop.
