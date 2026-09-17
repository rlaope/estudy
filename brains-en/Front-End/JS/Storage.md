# Storage

## Local Storage
- The `localStorage` read-only property allows access to a `Storage` object for the `Document`'s origin.
- Stored data is shared across browser sessions.

<br>

### Difference from session Storage
> `localStorage` data does not expire, while `session Storage` data disappears when the page session ends.

 ## Examples
`Storage.setItem(Key , valuse)` : Stores a value in Storage
 ```js
 localStorage.setItem('myCat', 'Tom');
 ```

 `Storage.getItme()` : Checks an item in Storage

 ```js
 const cat = localStorage.getItem('myCat');
 ```

 `Storage.removeItem()` : Removes an item from Storage

 ```js
 localStorage.removeItem('myCat');
 ```
