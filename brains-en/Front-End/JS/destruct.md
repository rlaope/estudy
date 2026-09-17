# Destructuring Assignment

### What is Destructuring Assignment?
- It is a JS expression that allows you to unpack properties from arrays or objects into individual variables.

```js
const user = {
  name : `huemang`,
  age : 17,
  email : 's22043@gsm.hs.kr'
}

const { name , age , email, address} = user

console.log(name);
// humang

console.log(age);
// 17

console.log(email);
// s22043@gsm.hs.kr

console.log(address);
// undefined

```
- Arrays can also use destructuring assignment.
- If you want to extract only the third value, you can append a comma after the variable you want to extract the value into.

```js
const fruits = ['apple' , 'banana' , 'Cherry']
const =[, , fruit] = fruits

console.log(fruit);
// Cherry
```

- If you assign a default value to a variable, that value will be used instead when the destructured value is `undefined`.

```js
let a, b;

[a=5, b=7] = [1];
console.log(a); // 1
console.log(b); // 7
```
