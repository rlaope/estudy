# Math

### toFixed()

- When the `toFixed()` method is called, you can specify how many decimal places to maintain as an argument.

- It returns a string data type.

```js
const pi = 3.14159265359879
console.log(pi); // 3.14159265358979

const str = pi.toFixed(2);
console.log(str); // 3.14
console.log(typeof str); // Stirng
```

<br>

### parseInt() , parseFloat()

These functions are also global functions.
- parseInt() extracts characters and returns them as an `integer` type.
- parseFloat() extracts characters and returns them as a `float` type.

```js
const pi = 3.14159265359879
console.log(pi); // 3.14159265358979

const str = pi.toFixed(2);
console.log(str); // '3.14'
console.log(typeof str); // Stirng

const integer = parseInt(str);
const float = parseFloat(str);

console.log(integer); // 3
console.log(float); // 3.14
```

### Math.abs()
- `abs : absolute` - The `Math.abs()` function returns the absolute value of the given number.

<br>

### Math.min() , Math.max()
- min : Returns the `smaller value` among the given arguments.
- max : Returns the `larger value` among the given arguments.

<br>

### Math.ceil() Math.cloor()
- ceil : `Rounds up` the given argument to the nearest integer.
- floor : `Rounds down` the given argument to the nearest integer.

### Math.round()
- `Rounds` the given argument.

<br>

### Math.random()
- Returns a random number.

<br>

```js
console.log('abs :', Math.abs(-12)) // abs : 12

console.log('min :' ,Math.min(2,8)) // min : 2

console.log('max :', Math.max(2,8)) // max : 8

console.log('ceil :', Math.ceil(3.14)) // ceil : 4

console.log('floor :', Math.floor(3.14)) // floor : 3

console.log('round :' ,Math.round(3.5)) // round : 4

console.log('random :' ,Math.random()) // 랜덤한 수 0 ~ 1
```

<br>

### Let's print a random integer from 1 to 10.

```js
let rand = Math.floor(Math.random() * 10 + 1);
console.log(rand) // 1 ~ 10
```

1. Execute the `floor()` method on the `rand` variable, and inside it, execute `random()`.

2. Executing `random()*10` outputs numbers from 0 to 9, so we add +1. Then, applying `floor()` to round down will result in random integers from 1 to 10.
