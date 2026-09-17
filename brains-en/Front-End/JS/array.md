# Array

What is an array?
- An array is used to store multiple values sequentially in a single variable.
- Arrays in JavaScript are objects and include useful built-in methods.
- An array is an object of type Array created by the Array constructor, and its prototype object is Array.prototype.

<br>

## Array Methods

### Array.prototype.find()
- The `find()` method returns the value of the `first element` in the provided array that satisfies the provided testing function. Otherwise, it returns `undefined`.

```js
const array1 = [5, 12, 8 , 130, 44];
const found = array1.find(element => element > 10);
console.log(found); // 12
```

1. Starting from the 0th index within `array1`, the `find` function uses an arrow function as an argument. If the condition is not met, it continues; if it is met, the callback returns the value.
2. You can see `12` being output through `console.log()`.

<br>

`callback`: A function to execute on each value in the array. It takes the following three arguments.
1. element: The current element being processed in the callback function.
2. index: The index of the current element being processed in the callback function.
3. array: The array `find` was called upon.

`thisArg`: Optional. An object to use as `this` when executing the callback.

## Array API

### .length
- Returns the length of the array.

```js
const numbers = [ 1 , 2 , 3 , 4];
const fruits = ['Apple' , 'Banana' , 'Cherry'];

console.log(numbers.length); // 4
console.log(fruits.length); // 3
console.log([1,2].length]); // 2
console.log([].length); // 0
```

`.length` is often used to check if an array contains any values.

<br>

### .concat()

- Merges two array data sets and returns a new array data set.
> The original data is not modified.

```js
const numbers = [ 1 , 2 , 3 , 4];
const fruits = ['Apple' , 'Banana' , 'Cherry'];

console.log(numbers.concat(fruits)) // 1 , 2 , 3 , 4 , 'Apple' , 'Banana' , Cherry

console.log(numbers); // 1 , 2 , 3 , 4
console.log(fruits); // 'Apple' , 'Banana' , 'Cherry'
```

### .forEach()
- This method executes the callback function provided as an argument for each item in the array data it is called on.

```js
const numbers = [ 1 , 2 , 3 , 4];
const fruits = ['Apple' , 'Banana' , 'Cherry'];

fruits.forEach(function (element, index, array){
  console.log(element, index, array);
})
// Apple 0 (array of fruits)
// Banana 1 (array of fruits)
// Cherry 2 (array of fruits)
```

### .map()

- Unlike `forEach()`, `map()` uses `return` to create and return a new array with the values.

```js
const numbers = [ 1 , 2 , 3 , 4];
const fruits = ['Apple' , 'Banana' , 'Cherry'];

const a = fruits.forEach(function(element, index){
  console.log(`${element}-${index}`) // Apple-0 Banana-1 Cherry-2
});
consle.log(a) // undefined

const b = fruits.map(funtion(element, index){
  return `${element}-${index}`;
})

console.log(b) // ['Apple-0' , 'Banana-1' , 'Cherry-2']

```
- `map()` can also return object data as an array.

```js
 const numbers = [ 1 , 2 , 3 , 4];
const fruits = ['Apple' , 'Banana' , 'Cherry'];

const a = fruits.forEach(function(element, index){
  console.log(`${element}-${index}`) // Apple-0 Banana-1 Cherry-2
});
consle.log(a) // undefined

const b = fruits.map(funtion(element, index){
  return {
    id : index, // index
    name: element // element
  } // Can also create an array containing object data.
})

console.log(b)
//      [{id : 0 , name : Apple}
//      {id : 0 , name : Apple}
//      {id : 0 , name : Apple}]
```
