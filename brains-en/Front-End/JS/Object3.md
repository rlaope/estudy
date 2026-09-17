# Object Methods

## Object.assign()
- The `Object.assign()` method is used to copy properties from one or more enumerable source objects to a target object.
- It returns the target object.

**Parameters**:
- `target` The target object.

**Return value**
- The target object.

```js
const target = {a : 1, b : 2};
const source = {b : 4, c : 5};

const returnedTarget = Object.assign(target, source);

console.log(target);
// { a : 1  , b : 4 , c : 5 }

console.log(returnedTarget);
// { a : 1  , b : 4 , c : 5 }
```

> `target` and `returnedTarget` are the same data.

> Even if objects look the same, their data can be different.

 <br>

 ## keys()
- `keys()` extracts the object's keys and turns them into an array.

 ```js
 const user ={
  name : "huemang",
  age : 17,
  email: "s22043@gsm.hs.kr"
 }

 const keys = Object.Keys(user);
 console.log(keys);
 // ['name' , 'age' , 'email']

 console.log(user['email']);

 const values = keys.map(key => user[key]);
 console.log(values);
 // ['huemang' , 17 , 's22043@gsm.hs.kr ]
 ```

> The `indexing` method using square brackets was used for object data. Let's learn more about it later.
