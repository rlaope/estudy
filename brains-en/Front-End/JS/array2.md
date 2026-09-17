# Array 2

### .filter()

- Unlike `map`, it returns `values` from an array that match a specific criterion for each item.
- It does not affect the original array, `same as map()`.
```js
 const numbers = [ 1 , 2 , 3 , 4];
const fruits = ['Apple' , 'Banana' , 'Cherry'];

const a = numbers.map(number => number < 3)
console.log(a); // [true ,true, false, false]

const b = numbers.filter(number => number < 3)
console.log(b); // [1,2]

console.log(numbers); // [ 1 , 2 , 3 , 4]

```

<br>

## .find() , findIndex()

- find()
  - Finds and returns an array `element` that matches a specific condition.

- findIndex()
  - Finds and returns an array `element` that matches a specific condition, and also returns its index.
```js
const numbers = [ 1 , 2 , 3 , 4];
const fruits = ['Apple' , 'Banana' , 'Cherry'];

const a = fruits.find(fruit => {
  return /^B/.test(fruit) // '/^B/' is a regular expression. We'll look into this later. (For now, just know it means finding something that starts with B.)
})

console.log(a); // Banana

const b = fruits.findIndex(fruit => {
  return /^C/.test(fruit) 
})

console.log(b); // Cherry , 2
```

<br>

## .includes()

- Returns a boolean indicating whether a specific value is included as an element in the array.
```js
const numbers = [ 1 , 2 , 3 , 4];
const fruits = ['Apple' , 'Banana' , 'Cherry'];

const a = numbers.includes(3)
console.log(a) // true

const b = fruits.includes('Huemang');
console.log(b) // false
```

<br>

## .push() .unpush()

- These methods modify the original array.
1. `push()`: Inserts a specific argument at the end of the array.
2. `unshift()`: Inserts a specific argument at the beginning of the array.

```js
const numbers = [ 1 , 2 , 3 , 4];
const fruits = ['Apple' , 'Banana' , 'Cherry'];

numbers.push(5);
console.log(numbers) // [ 1 , 2 , 3 , 4 , 5 ]

numbers.unshift(0)
console.log(numbers) // [ 0 , 1 , 2 , 3 , 4 , 5 ]
```

<br>

## .reverse()
- This method modifies the original array.
- `reverse` means to turn something backward, and true to its name, it reverses the order of elements in an array.


```js
const numbers = [ 1 , 2 , 3 , 4];
const fruits = ['Apple' , 'Banana' , 'Cherry'];

numbers.reverse();
fruits.reverse();

console.log(numbers) // 4 , 3 , 2 , 1
console.log(fruits) // 'Cherry' , 'Banana' , 'Apple'
```

<br>

## .splice()
- This method modifies the original array.
- Usage: `splice(startIndex, deleteCount, itemsToAdd)`
- It's a method for deleting items from an array.
- It can also insert items into an array.

```js
const numbers = [ 1 , 2 , 3 , 4];
const fruits = ['Apple' , 'Banana' , 'Cherry'];

numbers.splice(2,1) // Delete 1 item starting from index 2.

console.log(numbers) // 1 , 2, 4
```

```js
const numbers = [ 1 , 2 , 3 , 4];
const fruits = ['Apple' , 'Banana' , 'Cherry'];

numbers.splice(2,0,999) // Don't delete, just insert 999 at index 2.

console.log(numbers) // 1 , 2 , 999 , 3 , 4
```

## Spread Operator

- It expands an array's data, outputting each item separated by commas.

> Used by placing `...` next to a variable.

```js
const fruits = ['Apple' , 'Banana' , 'Cherry'];

console.log(...fruits)
// Apple Banana Cherry

function toObject(a,b,c){
  return {
    a: a,
    b: b,
    c: c
  }
}
console.log(toObject(...fruits))
```

- Using the spread operator makes it easy to output items.
- The spread operator can also be used in the parameter section, where it's called a `rest parameter`.
