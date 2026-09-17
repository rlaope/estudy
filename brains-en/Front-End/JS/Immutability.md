# Data Immutability
- Data can be categorized into primitive data and reference data.

### Primitive Data
> String, Number, Boolean, undefined, null

### Reference Data
> Object, Array, Function

## Data Immutability
- Primitive data is `immutable`.
- Reference data is `mutable`.

```js
let x = {
  name: 'huemang'
};

let y = x;

x.name = 'gyeongsu';

console.log(y.name); //  gyeongsu
console.log(x === y) // true
```
- Since y references x, even when the name is changed to `gyungsu`, y still points to x, so the changed value `gyungsu` is output.

> For reference data, even if the values are the same, a comparison might return false.

<br>

## Shallow Copy, Deep Copy

```js
const user = {
  name : 'Huemang',
  age : 17,
  emails : ['thesecon@gmail.com']
}
const copyUser = Object.assign({}, user)
//const copyUser = {...user}로도 가능
console.log(user === copyUser)

user.age == 22;
console.log('user',user);// 22
console.log('copyUser',copyUser); // 17
```
- Shallow Copy
> When an object is directly assigned, assignment by reference occurs, so both hold the same data (address).

```js
user.emails.push('piyrw9754@gmail.com')
console.log(user.emails === copyUser.emails); // true
console.log('user',user)
console.log('copyUser',copyUser)
```

- What is Deep Copy?

> Copies the data itself entirely.

> The two copied objects occupy completely independent memory.

> Value type objects undergo deep copying.

```js
import _ from 'lodash'

const copyUser = _.cloneDeep(user)
console.log(copyUser === user) // false

console.log('user',user.age) // 22
console.log('copyUser',copyUser.age) // 17
```

- It's a bit difficult to create a deep copy with plain JavaScript, so I used lodash's help.

- `_.cloneDeep(value)`: Recursively copies values.
