# Lodash

### What is Lodash?
- It is one of the popular `libraries` in JS. It is typically used to easily handle essential data structures such as arrays, collections, and dates.

<br>

### _.uniq()
> Keeps only unique values from duplicate values within an array.

```js
import _ from 'lodash'

const usersA = [
  { userId : '1', name : 'Huemang'},
  { userId : '2', name : "Neo"}
]

const usersB = [
  { userId : '1', name : 'Huemang'},
  { userId : '3', name : "Amy"}
]

const usersC = usersA.concat(usersB)

console.log('concat',usersC); // User objects are returned. There are two objects with id: 1 and name: Huemang.
console.log('uniqBy', _.uniqBy(usersC,'user')) // It became unique, and duplicate values were reduced to one.

const userD = _.unionBy(usersA, usersB, 'userId') // It combines the functionalities of uniqBy and concat.
// It became unique, duplicate values were reduced to one, and values from A and B are merged.
```

- `uniqBy()`: A method that makes values `unique` based on a specific property name within a single array.
- `unionBy()`: A method that merges multiple arrays, makes them `unique`, and returns the result.

### _.find( arguments , condition )

- Finds and returns the object data that matches the condition.

### _.findIndex( arguments , condition )

- Returns the index number of the object that matches the condition.

### ._remove( arguments , condition )

- Deletes the object that matches the condition.

```js
import _ from 'lodash'

const users - [
  { userId : '1' , name : 'Huemang'},
  { userId : '2' , name : 'Neo'},
  { userId : '3' , name : 'Amy'},
  { userId : '4' , name : 'Evan'},
  { userId : '5' , name : 'Lewis'},
]

const foundUser = _.find(users, {name : 'Amy'})
console.log(foundUser) // The corresponding object
const foundUserIndex = _.findIndex(users, {name : 'Amy'})
console.log(foundUserIndex) // 2

_.remove(users, { name : 'Huemang'})
// Deletes the object where name is Huemang.
```

> The Lodash library is useful for data processing in frontend development, so it's good to be familiar with it!
