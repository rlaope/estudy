# Functions in TypeScript

When implementing web applications, functions, which are frequently used, can have their types defined in TypeScript in three main ways.

- Function parameter types
- Function return types
- Function structure types

<br>

## Basic Function Type Declaration

1. JavaScript Function Declaration
```js
function su,(a,b){
  return a + b;
}
```

2. TypeScript Function Declaration

```ts
 function sum(a : number, b : number) : number {
  return a + b;
 }
```

> We've added `types` to the parameters and return values of functions, building upon the existing JavaScript function declaration style.

> If you don't specify a type for the function's return value, use `void`.

## Function Arguments

In TypeScript, all function arguments are considered mandatory. Therefore, if you set function parameters, you must pass arguments, even `undefined` or `null`, and the compiler checks if the defined parameter values have been passed. In other words, it means that only the defined parameter values can be received, and no additional arguments can be accepted.

```ts
function sum(a: number, b : number) : number {
  return a + b;
}
sum(10 , 20); // 30
sum(10 , 20 , 30); // error
sum(10); // error
```

- This characteristic is contrary to JavaScript's behavior, where you don't have to pass as many arguments as there are defined parameters.
- If you want to preserve this characteristic, you can define it using `?`.

```ts
function sum(a : number, b? : number) : number {
  return a + b;
}

sum(10 , 20); // 30
sum(10 , 20 , 30); // error
sum(10); // 10
```

> JavaScript does not require passing values for all defined parameter arguments.

<br>

## Parameters with REST Syntax

The Rest syntax, supported in ES6, can be used in TypeScript as follows.

```ts
function num(a : number , ...numb : number[]) : number {
  const totalOfNums = 0;
  for(let key in nums){
    totalOfNums += nums[key];
  }
  return a + totalOfNums;
}
```

<br>

## this

TypeScript can detect when JavaScript's `this` is used incorrectly.

- To explicitly specify what `this` refers to in TypeScript, use the following syntax.

```ts
function 함수명(this : 타입){
  // ...
}
```

```ts
interface Vue {
  el : string;
  count : number;
  init(this : Vue) : () => {};
}

let vm: Vue = {
  el : '#app',
  count : 10,
  init: function(this: Vue){
    return () => {
      return this.count;
    }
  }
}

let getCount = vm.init();
let count = getCount();
console.log(count); // 10
```

> When compiling the above code with TypeScript, no error will occur even if the `--noImplicitThis` option is enabled.

## `this` in Callbacks

Unlike the general `this` usage we've seen, there are times when `this` needs to be distinguished when a function is passed as a callback. In such cases, it can be enforced as follows.

```ts
interface UIElement {
  // The `this : void` code below means that the `this` type of the function does not need to be declared.
  addClickListener(onclick : (this : void, e : Event) => void) : void;
}

class Handler {
  info: string,
  onClick(this : Handler, e : Event){
    // An error occurs here because `this` was used, even though the `UIElement` interface specification above indicated it was not needed.
    this.info = e.message;
  }

  let handler = new Handler();
  uiElemnet.addClickListener(handler.onClick); // error !
}
```

If you need to implement `Handler` according to the `UIElement` interface specification, modify it as follows.

```ts
class Handler {
  info: string;
  onClick(this : void , e : Event){
    // Since the type of `this` is void, `this` cannot be used here.
    console.log('clicked!');
  }
}

let handler = new Handler();
uiElement.addClickListener(handler.onClick);
```
