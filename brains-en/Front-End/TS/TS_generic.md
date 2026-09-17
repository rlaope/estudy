# Generic

## Why Use Generics
- Data structures are a useful case for generics. Let's assume we implement a stack data structure in TypeScript as follows.

```ts
class Stack{
  private data : any[] = [];

  constructor() {}

  push(item : any) : void {
    this.data.push(item);
  }

  pop(): any {
    return this.data.pop();
  }
}
```
In TypeScript, implementing it using `any` as shown above is the easiest way.  
However, implementing it with `any` creates a problem where the types of the stored data are not all the same. Therefore, when extracting data from the stack above, type checking must always be performed at runtime.

```ts
const stack = new Stack();
stack.push(1);
stack.push('a');
stack.pop.substring(0) // 'a'
stack.pop.substring(0) // Throw TypeError
```
However, if we always restrict it to only accept `number` type variables to guarantee the data type, its versatility decreases.  
Of course, this can also be handled with inheritance.

```ts
class NumberStack extends Stack{
  constructor() {
    super();
  }
  push(itme : number): void{
    super.push(item);
  }
  pop() : number {
    return super.pop();
  }
}
```
However, when using inheritance, it's equally cumbersome because you have to add a new class and generate duplicate code every time you add a data type.  
  
In such cases, generics can be very useful.

<br>

## Generic Syntax

### class
If we re-implement the stack data structure using generics, it will take the following form.

```ts
class Stack<T> {
  private data : T[] = []
  constructor() {}

  push(itme: T) : void {
    this.data.push(item);
  }
  pop() : T {
    return this.data.pop();
  }
}
```
- `T` is an abbreviation for Type and is conventionally used when declaring generics in other languages as well. Anything that can be used as an identifier can go here. For example, `$` or `_` are also possible. However, `T` is typically used and is called a type variable.
- When a class declares that it will use generics, `T` becomes a specific type that can be used within that class.

```ts
const numberStack = new Stack<number>();
const stringStack = new Stack<string>();
numberStack.push(1);
stringStack.push('a');
```
Now, each stack always knows, stores, and returns only the type declared at creation. This allows the compiler to know the return type, enabling auto-completion in editors, which contributes to increased productivity.  
  
However, since types are checked at compile time, they cannot be prevented at runtime.

```ts
numberStack.push('' as any);
```
Such code bypasses compile-time type checking and therefore cannot be prevented.

<br>

### Function
Suppose we need to implement a function that takes an array as input and returns its first element (like `lodash.head()`). If we don't use generics, we would write it like this.

```ts
function first(arr : any[]) : any {
  return arr[0];
}
```
The code above can also accept an array of any type, so the return type is unknown. Using generics, it can be implemented simply as follows.

```ts
function first<T>(arr : T[]) : T {
  return arr[0];
}
```
What's added is `<T>` next to the function identifier, just like with classes. Similarly, within this function, `T` is treated as a specific type.  
  
  Likewise, when using it, you just need to specify the type using generic syntax when calling the function.
  
  ```ts
  first<number>([1,2,3]); / 1
  ```

<br>

### Two or More Type Variables
Generic functions or classes can also use two or more type variables. Suppose we need to implement a function that takes two variables and returns them as a pair, as follows.

```ts
function toPair(a: any, b : any) : [any , any] {
  return [a , b];
}
```
If we assume the two input variables have different types, then two type variables are needed.

```ts
function toPari<T,U>(a : T, b : U) : [T,U]{
  return [a,b];
}
```
- Using generics, it can be implemented in the form shown above.
- You will see two type variables, `T` and `U`. As mentioned earlier, `T` is conventionally used, and subsequent type variables can follow alphabetically. This is similar to the conventional use of `i` and `j` as index variables in loops.  
  
Function usage
```ts
toPari<string, number>('1',1) // ['1',1]
```

<br>

### Extended Type Variables
Type variables can also extend existing types. This allows you to restrict the types of variables that can be received as input. Additionally, editors can predict methods or properties of that type, enabling auto-completion.  
  
  Applying this with multiple type variables allows you to write code like the following.

  ```ts
  function getFirst<T extends Stack<U>, U>(container : T) : U {
    const item = container.pop();
    container.push(item);
    return item;
  }
  ```
  `getFirst` is a function that returns the first element of the input stack. When using it, you can do so as shown below. If you use a type other than `Stack` or a type that extends `Stack` for the first type argument, an error will occur.
  ```ts
  getFirst<Stack<number>,number>(numberStack);
  getFirst<number,number>(1); // error
  ```
