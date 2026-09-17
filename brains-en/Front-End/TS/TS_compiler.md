# TypeScript Compiler

TypeScript involves a process of being compiled into JavaScript by babel. There are various options for how to compile it, and the `tsconfig` file plays that role.

## 1. Compiler Option
- Within the tsconfig file, you can set various options required for compilation. There are many options.
- Typically: target, module, rootDir, allowJS, SourceMap, etc.

<br>

### 1. target
- TypeScript will be compiled to the version specified in `target`. Generally, `ES5` or `ES6` is used.

### 2. outDir
- Sets the path to the directory where TypeScript compiled files will be placed.

```json
outDir : "./build"
```

### 3. rootDir
- Sets the top-level directory where TypeScript files can be created. For example,

```json
rootDir : "./src"
```

if specified, all `.ts` files must be created inside the `src` directory.

### 4. sourceMap
- `SourceMap : True` is useful for debugging.

Debugging is primarily done in the browser using `Developer Tools - Sources`. At this point, the files that can be viewed are compiled `.js` files, which differ from the `.ts` files you wrote directly. This can cause confusion for developers. SourceMap generates a map file for each `.ts` file during compilation, which allows debugging by viewing the `.ts` file in the browser.

<br>

## 2. exclude && include
When compiling, you can exclude specific `.ts` files or limit compilation to only certain `.ts` files. `exclude` and `include` provide this functionality.
```json
exclude : ["./src/login.ts"], // Compiles all .ts files except login.ts
include : ["./src/shopping.ts", "./str/buy.ts"] // Compiles only shopping.ts and buy.ts
```

<br>

## 3. strict
- Controls options that strictly check types all at once.
> Must be set to true.

### noImplicitAny
- If the compiler determines a type to be `any` during type inference, it throws a compilation error, prompting explicit type specification.

```ts
function test(a){ // Error due to noImplicitAny
  //Todo..
}

test(10) // Cannot be used
```

- `suppressImplicitAnyIndexErrors`: When `noImplicitAny` is used, if an index signature is missing for an indexable object, an error occurs. This option handles that exception.

<br>

### noImplicitThis
- Throws an error if `this` expressions are used without explicitly specifying an `any` type.

```ts
function test(name : string){
  this.name = name // Error occurs
}
```

> Solution
> ```ts
> function test(this: any, name : string){
> this.name : name
> }
> ```

Place `this` in the first parameter position and specify its type. This syntax is only allowed in `TypeScript`.

> In Classes, `noImplicitThis` errors do not occur when using `this`. The class type is automatically assigned to `this` for methods.
> However, `this` cannot be used as the first parameter of a `constructor` function.

<br>

### strictNullCheck
- Removes `null` and `undefined` which are automatically included in all types.
> Exception: `void` can be assigned to `undefined`.

<br>

- `strictNullCheck` not set
  
```ts
function test(a : number){
  if(a > 0){
    return a * 5;
  }
}
test(5) // 25
test(-5) // NaN, no compilation error
```

- `strictNullCheck` set
```ts
function test(a : number){
  if(a > 0){
    return a * 5;
  }
}
test(5) // 25
test(-5) // Compilation error occurs (undefined)
```

<br>

### strictFunctionTypes Option
- When assigning a function, if the parameter types of the function are not the same or a supertype (contravariance), a compilation error occurs.

```ts
class Person{}
class Man extends Person{}
class Boy extends Man{}

function tset(f : (p : Man) => Man){}

test((p : Boy) : Man => {}) // Compilation error if parameter is a subtype
```

> `Contravariance`: Function parameters can only be assigned if they are the same type or a supertype.
> `Covariance`: Can be assigned if they are the same type or a subtype.
> ```ts
> let sub : string = ''
> let sup : string | number = sub // Possible
> ```
> Function parameters must be contravariant (same or supertype), and return values must be covariant (same or subtype).

<br>

## 4. Conclusion
It's impossible to memorize all the options within the tsconfig file. Therefore, you should follow the link at the very top of the file to check the options and use only what you need.
