# Type

## Basic Types in TypeScript
- You can define types for JavaScript code, such as variables or functions, using TypeScript.
- TypeScript primarily has the following 12 basic types:

  1. Boolean
  2. Number
  1. String
  1. Object
  1. Array
  1. Tuple
  1. Enum
  1. Any
  1. Void
  1. Null
  1. Undefined
  1. Never

## Primitive Type
- These are data types that store actual values, not objects or references.
- The ability to use built-in functions for primitive types is due to how JavaScript handles them.
- (As of ES2015) 6 types
  * boolean
  * number
  * string
  * symbol
  * null
  * undefined

### boolean
- The most basic data type.
- Simply a true or false value.
- Called 'boolean' in JS / TS.

### number
- Like JavaScript, all numbers in TypeScript are floating-point values.
- In addition to hexadecimal and decimal literals, TypeScript supports binary and octal literals introduced in ECMAScript 2015.
- NaN
- Notation like 1_000_000 is possible.

### string
- As in other languages, we use the `string` type to refer to this text format.
- Like JavaScript, TypeScript uses double quotes "" and single quotes '' to enclose string data.

### Template String
- Strings that can span multiple lines or embed expressions.
- These strings are enclosed by backtick characters.
- Embedded expressions are used in the form `${expr}`.

### symbol
- This is ECMAScript 2015's Symbol.
- Cannot be used with `new Symbol`.
- You can create a symbol type by using `Symbol` as a function.
- It is used to hold primitive type values.
- It creates unique and immutable values.
- Therefore, it is often used to control access.

### null & undefined
- In TypeScript, `undefined` and `null` actually have their own types: `undefined` and `null` respectively.
- Similar to `void`, they are not particularly useful on their own.
- Both exist only in lowercase.
- To allow assigning `undefined` and `null`, you must use a `union type`.

### object
- A type used when you want to represent `something that is not a primitive type`.

### array
- Originally, arrays in JavaScript are objects.
- Usage
  * Array<Type>
  * Type[]

### Tuple
- An array where the types are not uniform.
- It is also an object.
- Caution is required when extracting and using elements.
    * If you destructure the array, the types are correctly inferred.

### any
- A type that can be anything.
- The key is to avoid using it as much as possible.
- This is because type checking does not occur properly at compile time.
- Therefore, some compiler options will throw an error if `any` should be used but isn't.
    * `noImplicitAny`

### unknown
- When writing applications, you may need to describe the type of a variable you don't know.
- You might intentionally want these values to accept any value from dynamic content.
- You can narrow down to a more specific variable by performing `typeof` checks, comparison checks, or `advanced type guards`.

### never
- Used in return types.
- When used in return types, it mostly covers the following three cases:
> The `never` type is a subtype of all types and can be assigned to all types.
> However, nothing can be assigned to `never`.
> Not even `any` can be assigned to `never`.
> It is also used to prevent mistakes of inserting incorrect types.

### void
- Represents an empty state that holds no type.
- It has a type but no value.
- It is lowercase.
- Generally used as the return type for functions that do not return a value.
- The only assignable value is `undefined`.

## [Examples](https://github.com/KIMHUEMANG/Study_TypeScript/tree/main/TypeScript_Essentials)
