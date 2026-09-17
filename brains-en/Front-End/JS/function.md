# Function

### What is a Function?
- A `function` is an independent block designed to perform a single, specific task.

- These functions can be called repeatedly to perform that task whenever needed.

### Function Declaration

 ```js
 // Function without a return value
 function sayHello(){
  console.log("hello");
 }
// Function with a return value
 function plus(a, b){
  console.log(a + b);
 }
 ```

 ### Arrow Functions
 - A way to create functions with simpler and more concise syntax than function expressions.
  
  ```js
  let func = (arg1, arg2, ...argN) => expression
  ```
  >This code creates a function `func` that takes arguments `arg1...argN`.

  >The function `func` evaluates the expression to the right of the arrow (`=>`) and returns the result of that evaluation.

  Example:
  ```js
  // returns a + b
  let plus = function(a, b){
    return a + b;
  }

  // returns a + b
  let plus = (a,b)=> a + b;
  ```

  As shown above, the two functions perform the same task.  
  - If there is only one argument, the parentheses enclosing the argument can be omitted.
  - Omitting parentheses can further reduce code length.

  ### Timer Functions
  1. `setTimeout(function, time): Executes a function after a specified time.`
  2. `setInterval(function, time): Executes a function at regular time intervals.`
  3. `clearTimeout(); Terminates the set Timeout function.`
  4. `clearInterval(); Terminates the set Interval function.`

  ```js
  // Prints "hello" every 3 seconds
  const timer = setInterval( () => {
  console.log("hello");
}, 3000);

  // Stops when the HTML h1 tag is clicked
  const h1El = document.querySelector("h1");
  h1El.addEventListener("click",() => {
  clearTimeout(timer);
})

  ```

  ### Callbacks
  - A function used as an argument to another function.

  ```js
  function timeout(callback) {
  setTimeout(() =>{
    console.log("Hello");
    callback();
  }, 3000);
}

timeout(() => {
  console.log("done");
});
  ```

  If written as above, "hello" will be printed after 3 seconds, followed by "done".  
  If not written like that and the screen is executed from outside, "done" will be printed first.

<br>

  ### this
  - Refers to itself within an object.

  ```js
  const Huemang = {
    firstName : "Huemang",
    lastName : "Kim",
    getFullName: function(){
      return `${this.firstName} ${this.lastName}`
    }
  }
  console.log(Huemang.getFullName()); // Huemang Kim
  ```

  - Regular functions define `this` based on their call site.
  - Arrow functions define `this` based on the scope where they are declared!
  

### Constructor Functions
- It is a function used with the `new` keyword. We can define our own functions and use the `new` keyword to create constructor functions, or we can use constructor functions built into JavaScript by default.
```js
function user(first, last){
  this.firstName = first,
  this.lastName = last
}

const huemang = new user('Huemang',"Kim");

console.log(huemang); // The huemang object is printed
```
